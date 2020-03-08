use crate::bytes::Storage;
use crate::error;
use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, Event, EventCode, EventPacket, ReturnParameters};
use crate::hci::stream::Error::UnsupportedPacketType;
use crate::hci::{ErrorCode, HCIConversionError, HCIPackError, Opcode, FULL_COMMAND_MAX_LEN};
use core::convert::{TryFrom, TryInto};
use core::pin::Pin;
use core::task::{Context, Poll};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
#[repr(u8)]
pub enum PacketType {
    Command = 0x01,
    ACLData = 0x02,
    SCOData = 0x03,
    Event = 0x04,
    Vendor = 0xFF,
}
impl From<PacketType> for u8 {
    fn from(packet_type: PacketType) -> Self {
        packet_type as u8
    }
}
impl From<PacketType> for u32 {
    fn from(packet_type: PacketType) -> Self {
        packet_type as u32
    }
}
impl TryFrom<u8> for PacketType {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PacketType::Command),
            0x02 => Ok(PacketType::ACLData),
            0x03 => Ok(PacketType::SCOData),
            0x04 => Ok(PacketType::Event),
            0xFF => Ok(PacketType::Vendor),
            _ => Err(HCIConversionError(())),
        }
    }
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum Error {
    CommandError(HCIPackError),
    UnsupportedPacketType(u8),
    BadOpcode,
    BadEventCode,
    StreamClosed,
    StreamFailed,
    IOError,
    HCIError(ErrorCode),
}
impl From<HCIPackError> for Error {
    fn from(e: HCIPackError) -> Self {
        Error::CommandError(e)
    }
}
impl error::Error for Error {}
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
    fn set_filter(self: Pin<&mut Self>, filter: &Filter) -> Result<(), Error>;
    fn get_filter(self: Pin<&Self>) -> Result<Filter, Error>;
}
/// Asynchronous HCI byte stream writer.
pub trait HCIWriter {
    /// Write some bytes into the `HCIWriter` stream. Mirrors an `AsyncWrite` trait. Returns
    /// `Ok(usize)` with actual bytes written.
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>>;

    /// Flush any bytes in the `HCIWriter` stream. Mirrors an `AsyncWrite` trait.
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>;
}
/// Asynchronous HCI byte stream reader.
pub trait HCIReader {
    /// Read some bytes into `buf`. Mirrors an `AsyncRead` trait. Returns `Ok(usize)` with actual
    /// bytes read.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>>;
}
pub struct Stream<S: HCIReader + HCIFilterable> {
    pub stream: S,
}
pub const HCI_EVENT_READ_TRIES: usize = 10;
impl<S: HCIReader + HCIFilterable> Stream<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
    pub fn stream_pinned(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) }
    }
    pub async fn send_exact(mut self: Pin<&mut Self>, mut buf: &[u8]) -> Result<(), Error>
    where
        S: HCIWriter,
    {
        while !buf.is_empty() {
            let amount = futures_util::future::poll_fn(|cx| {
                self.as_mut().stream_pinned().poll_write(cx, buf)
            })
            .await?;
            buf = &buf[amount..];
        }
        futures_util::future::poll_fn(|cx| self.as_mut().stream_pinned().poll_flush(cx)).await
    }
    pub async fn send_command<Cmd: Command, Return: ReturnParameters, Buf: Storage>(
        mut self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<CommandComplete<Return>, Error>
    where
        S: HCIWriter,
    {
        let mut buf = [0_u8; FULL_COMMAND_MAX_LEN];
        let len = command.full_len();
        // Pack Command
        command
            .pack_full(&mut buf[..len])
            .map_err(Error::CommandError)?;
        // New Filter
        let mut filter = Filter::default();
        filter.enable_type(PacketType::Command);
        filter.enable_type(PacketType::Event);

        filter.enable_event(EventCode::CommandStatus);
        filter.enable_event(EventCode::CommandComplete);
        filter.enable_event(EventCode::LEMeta);

        filter.enable_event(CommandComplete::<Return>::CODE);
        if !filter.get_event(CommandComplete::<Return>::CODE) {
            return Err(Error::BadEventCode);
        }
        *filter.opcode_mut() = Cmd::opcode();

        // Save Old Filter
        let old_filter = self.as_mut().stream_pinned().as_ref().get_filter()?;
        self.as_mut().stream_pinned().set_filter(&filter)?;

        // Send command Bytes
        self.as_mut().send_exact(&buf[..len]).await?;

        // Wait for response
        for _try_i in 0..HCI_EVENT_READ_TRIES {
            let event: EventPacket<Buf> = self.as_mut().read_event().await?;
            println!("event: {:?}", event);
            if event.event_code() == CommandComplete::<Return>::CODE {
                if Opcode::unpack(&event.parameters().as_ref()[1..3])? == Cmd::opcode() {
                    self.stream_pinned().set_filter(&old_filter)?;
                    return CommandComplete::unpack_from(event.parameters())
                        .map_err(Error::CommandError);
                }
            }
        }
        Err(Error::StreamFailed)
    }
    pub async fn read_exact(mut self: Pin<&mut Self>, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            let amount = futures_util::future::poll_fn(|cx| {
                self.as_mut().stream_pinned().poll_read(cx, buf)
            })
            .await?;
            buf = &mut buf[amount..];
        }
        Ok(())
    }
    pub async fn read_event<Buf: Storage>(
        mut self: Pin<&mut Self>,
    ) -> Result<EventPacket<Buf>, Error> {
        let mut header = [0_u8; EVENT_HEADER_LEN];
        self.as_mut().read_exact(&mut header[..]).await?;
        if header[0] != u8::from(PacketType::Event) {
            return Err(UnsupportedPacketType(header[0]));
        }
        let event_code = EventCode::try_from(header[1]).map_err(|_| Error::BadEventCode)?;
        let len = header[0];
        let mut buf = Buf::with_size(len.into());
        self.read_exact(buf.as_mut()).await?;
        Ok(EventPacket::new(event_code, buf))
    }
}
const EVENT_HEADER_LEN: usize = 3;
#[cfg(feature = "std")]
impl<T: futures_io::AsyncRead> HCIReader for T {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        futures_io::AsyncRead::poll_read(self, cx, buf).map_err(|_| Error::IOError)
    }
}
#[cfg(feature = "std")]
impl<T: futures_io::AsyncWrite> HCIWriter for T {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        futures_io::AsyncWrite::poll_write(self, cx, buf).map_err(|_| Error::IOError)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        futures_io::AsyncWrite::poll_flush(self, cx).map_err(|_| Error::IOError)
    }
}
/// Implements all the traits required to be a complete HCI Stream.
pub trait HCIStreamable: HCIWriter + HCIReader + HCIFilterable {}
impl<T: HCIWriter + HCIReader + HCIFilterable> HCIStreamable for T {}
