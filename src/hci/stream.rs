//! HCI Stream. Abstracts over byte read/write functions to allow for reading events and writting
//! commands.
use crate::bytes::Storage;
use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, Event, EventCode, EventPacket};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::{Opcode, FULL_COMMAND_MAX_LEN};
use crate::{error, poll_function::poll_fn, PackError};
use core::convert::{TryFrom, TryInto};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

impl From<PackError> for Error {
    fn from(e: PackError) -> Self {
        Error::CommandError(e)
    }
}
impl error::Error for Error {}
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
pub trait HCIReader: Unpin {
    /// Read some bytes into `buf`. Mirrors an `AsyncRead` trait. Returns `Ok(usize)` with actual
    /// bytes read.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>>;
}
/// HCI Stream. Wraps the `poll_read` and `poll_write` methods of [`HCIReader`] and [`HCIWriter`]
/// to provide the [`Stream::read_packet`] and [`Stream::send_command`] functions.
pub struct Stream<S: HCIReader> {
    pub stream: S,
}
pub const HCI_EVENT_READ_TRIES: usize = 10;
impl<S: HCIReader> Stream<S> {
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
            let amount = poll_fn(|cx| self.as_mut().stream_pinned().poll_write(cx, buf)).await?;
            buf = &buf[amount..];
        }
        poll_fn(|cx| self.as_mut().stream_pinned().poll_flush(cx)).await
    }
    pub async fn send_command<Cmd: Command>(
        mut self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<CommandComplete<Cmd::Return>, Error>
    where
        S: HCIWriter + HCIFilterable,
    {
        const BUF_LEN: usize = FULL_COMMAND_MAX_LEN;
        let mut buf = [0_u8; BUF_LEN];
        // Pack Command
        let len = command
            .packet_pack_into(&mut buf[..])
            .map_err(Error::CommandError)?;
        // New Filter
        let mut filter = Filter::default();
        filter.enable_type(PacketType::Command);
        filter.enable_type(PacketType::Event);

        filter.enable_event(EventCode::CommandStatus);
        filter.enable_event(EventCode::CommandComplete);
        filter.enable_event(EventCode::LEMeta);

        filter.enable_event(CommandComplete::<Cmd::Return>::CODE);
        if !filter.get_event(CommandComplete::<Cmd::Return>::CODE) {
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
            // Reuse `buf` to read the RawPacket
            let event = EventPacket::try_from(self.as_mut().read_packet(&mut buf[..]).await?)?;
            if event.event_code() == CommandComplete::<Cmd::Return>::CODE {
                if Opcode::unpack(&event.parameters().as_ref()[1..3])? == Cmd::opcode() {
                    self.stream_pinned().set_filter(&old_filter)?;
                    return CommandComplete::unpack_from(event.parameters())
                        .map_err(Error::CommandError);
                }
            }
        }
        Err(Error::StreamFailed)
    }
    pub async fn read_bytes(mut self: Pin<&mut Self>, buf: &mut [u8]) -> Result<usize, Error> {
        poll_fn(|cx| self.as_mut().stream_pinned().poll_read(cx, buf)).await
    }
    pub fn read_packet<'r, 'b>(&'r mut self, buf: &'b mut [u8]) -> ByteRead<'b, 'r, S> {
        ByteRead::new(&mut self.stream, buf)
    }
}
pub struct ByteRead<'b: 'r, 'r, R: HCIReader> {
    buf: Option<&'b mut [u8]>,
    reader: &'r mut R,
}
impl<'b: 'r, 'r, R: HCIReader> ByteRead<'b, 'r, R> {
    pub fn new(reader: &'r mut R, buf: &'b mut [u8]) -> Self {
        Self {
            buf: Some(buf),
            reader,
        }
    }
}
impl<'b: 'r, 'r, R: HCIReader> core::future::Future for ByteRead<'b, 'r, R> {
    type Output = Result<RawPacket<&'b [u8]>, Error>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<RawPacket<&'b [u8]>, Error>> {
        let this = &mut *self;
        let buf = match &mut this.buf {
            None => return Poll::Ready(Err(Error::StreamFailed)),
            Some(b) => b,
        };
        let len = match Pin::new(&mut *this.reader).poll_read(cx, *buf) {
            Poll::Ready(r) => match r {
                Ok(l) => l,
                Err(e) => return Poll::Ready(Err(e)),
            },
            Poll::Pending => return Poll::Pending,
        };
        debug_assert!(len < buf.len(), "there might be more bytes to read");
        Poll::Ready(
            RawPacket::try_from(&this.buf.take().expect("just used above")[..len])
                .map_err(|_| Error::BadPacketCode),
        )
    }
}
const EVENT_HEADER_LEN: usize = 3;
#[cfg(feature = "std")]
impl<T: futures_io::AsyncRead + Unpin> HCIReader for T {
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
pub trait HCIStreamable: HCIWriter + HCIReader + HCIFilterable + Send {}
impl<T: HCIWriter + HCIReader + HCIFilterable + Send> HCIStreamable for T {}

pub struct PacketStream<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage<u8>> {
    stream: &'a mut Stream<S>,
    buf: Buf,
}
impl<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage<u8>> futures_core::stream::Stream
    for PacketStream<'a, S, Buf>
{
    type Item = Result<RawPacket<Buf>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        Pin::new(&mut this.stream.read_packet(this.buf.as_mut()))
            .poll(cx)
            .map(|r| Some(r.map(|p| p.clone_buf())))
    }
}
