//! HCI Stream. Abstracts over byte read/write functions to allow for reading events and writting
//! commands.
use crate::bytes::Storage;
use crate::error;
use crate::hci::command::CommandPacket;
use crate::hci::event::{EventCode, EventPacket, StaticHCIBuffer, MAX_HCI_PACKET_SIZE};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::{adapter, Opcode, StreamError};
use crate::PackError;
use core::convert::{TryFrom, TryInto};
use core::ops::Deref;
use core::ops::DerefMut;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::u32;
use futures_util::future::{poll_fn, LocalBoxFuture};

impl From<PackError> for StreamError {
    fn from(e: PackError) -> Self {
        StreamError::CommandError(e)
    }
}
impl error::Error for StreamError {}
/// HCI Filter. Sets what kind of HCI Packets and HCI Events are received by the HCI Stream.\
/// Designed around the BlueZ socket filter so this type may change in the future be more
/// platform agnostic.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Filter {
    type_mask: u32,
    event_mask: [u32; 2],
    opcode: Opcode,
}
pub const FILTER_LEN: usize = 14;
impl Filter {
    pub fn pack(&self) -> [u8; FILTER_LEN] {
        let mut out = [0_u8; FILTER_LEN];
        out[..4].copy_from_slice(&self.type_mask.to_le_bytes()[..]);
        out[4..8].copy_from_slice(&self.event_mask[0].to_le_bytes()[..]);
        out[8..12].copy_from_slice(&self.event_mask[1].to_le_bytes()[..]);
        self.opcode
            .pack(&mut out[12..14])
            .expect("hardcoded array length");
        out
    }
    pub fn unpack(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == FILTER_LEN {
            Some(Self {
                opcode: Opcode::unpack(&bytes[12..14]).ok()?,
                type_mask: u32::from_le_bytes(
                    (&bytes[..4]).try_into().expect("hardcoded array length"),
                ),
                event_mask: [
                    u32::from_le_bytes((&bytes[4..8]).try_into().expect("hardcoded array length")),
                    u32::from_le_bytes((&bytes[8..12]).try_into().expect("hardcoded array length")),
                ],
            })
        } else {
            None
        }
    }
    pub fn all_events() -> Filter {
        Filter {
            type_mask: 1 << 4,
            event_mask: [u32::MAX, u32::MAX],
            opcode: Opcode::nop(),
        }
    }
    pub fn enable_event(&mut self, event: EventCode) {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] |= 1_u32 << event;
        } else {
            self.event_mask[1] |= 1_u32 << (event - 32);
        }
    }
    pub fn disable_event(&mut self, event: EventCode) {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] &= !(1_u32 << event);
        } else {
            self.event_mask[1] &= !(1_u32 << (event - 32));
        }
    }
    pub fn get_event(&self, event: EventCode) -> bool {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] & (1_u32 << event) != 0
        } else {
            self.event_mask[1] & (1_u32 << (event - 32)) != 0
        }
    }
    pub fn enable_type(&mut self, packet_type: PacketType) {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask |= 1_u32 << packet_type;
    }
    pub fn disable_type(&mut self, packet_type: PacketType) {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask &= !(1_u32 << packet_type);
    }
    pub fn get_type(&self, packet_type: PacketType) -> bool {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask & (1_u32 << packet_type) != 0
    }
    pub fn opcode(&self) -> Opcode {
        self.opcode
    }
    pub fn opcode_mut(&mut self) -> &mut Opcode {
        &mut self.opcode
    }
}
/// Set IOCTL HCI filter. See [`Filter`] for more.
pub trait HCIFilterable {
    fn set_filter(self: Pin<&mut Self>, filter: &Filter) -> Result<(), adapter::Error>;
    fn get_filter(self: Pin<&Self>) -> Result<Filter, adapter::Error>;
}
/// Asynchronous HCI byte stream writer.
pub trait HCIWriter {
    /// Write some bytes into the `HCIWriter` stream. Mirrors an `AsyncWrite` trait. Returns
    /// `Ok(usize)` with actual bytes written.
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, adapter::Error>>;

    /// Flush any bytes in the `HCIWriter` stream. Mirrors an `AsyncWrite` trait.
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), adapter::Error>>;
}
/// Asynchronous HCI byte stream reader.
pub trait HCIReader: Unpin {
    /// Read some bytes into `buf`. Mirrors an `AsyncRead` trait. Returns `Ok(usize)` with actual
    /// bytes read.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, adapter::Error>>;
}
/// HCI Stream. Wraps the `poll_read` and `poll_write` methods of [`HCIReader`] and [`HCIWriter`]
/// to provide the [`Stream::read_packet`] and [`Stream::send_command`] functions.
#[derive(Clone, Debug)]
pub struct Stream<S: HCIReader, B: Deref<Target = S>> {
    pub stream: Pin<B>,
}
pub const HCI_EVENT_READ_TRIES: usize = 50;
impl<S: HCIReader, B: Deref<Target = S> + DerefMut> Stream<S, B> {
    pub fn new(stream: Pin<B>) -> Self {
        Self { stream }
    }
    pub fn stream_pinned(&mut self) -> Pin<&mut S> {
        self.stream.as_mut()
    }
    pub async fn send_exact(&mut self, mut buf: &[u8]) -> Result<(), adapter::Error>
    where
        S: HCIWriter,
    {
        while !buf.is_empty() {
            let amount = poll_fn(|cx| self.stream_pinned().poll_write(cx, buf)).await?;
            buf = &buf[amount..];
        }
        poll_fn(|cx| self.stream_pinned().poll_flush(cx)).await
    }
    pub async fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, adapter::Error> {
        poll_fn(|cx| self.stream_pinned().poll_read(cx, buf)).await
    }
    pub async fn read_event<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, adapter::Error> {
        let mut event_buf = StaticHCIBuffer::with_size(MAX_HCI_PACKET_SIZE);
        let len = self.read_bytes(event_buf.as_mut()).await?;
        let packet = RawPacket::try_from(&event_buf.as_ref()[..len])
            .map_err(|_| StreamError::BadPacketCode)?;
        let event_packet = EventPacket::try_from(packet).map_err(StreamError::EventError)?;
        Ok(event_packet.to_new_storage())
    }
    pub async fn send_command_packet(
        &mut self,
        packet: CommandPacket<&[u8]>,
    ) -> Result<(), adapter::Error>
    where
        S: HCIWriter,
    {
        let out = packet.pack_as_raw_packet::<StaticHCIBuffer>();
        self.send_exact(out.as_ref()).await
    }
}
impl<S: HCIWriter + HCIReader, B: Deref<Target = S> + DerefMut> adapter::Adapter for Stream<S, B> {
    fn write_command<'s, 'p: 's>(
        &'s mut self,
        packet: CommandPacket<&'p [u8]>,
    ) -> LocalBoxFuture<'s, Result<(), adapter::Error>> {
        Box::pin(self.send_command_packet(packet))
    }

    fn read_event<'s, 'p: 's, Buf: Storage<u8> + 'p>(
        &'s mut self,
    ) -> LocalBoxFuture<'s, Result<EventPacket<Buf>, adapter::Error>> {
        Box::pin(self.read_event())
    }
}
