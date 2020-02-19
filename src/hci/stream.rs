use crate::hci::command::Command;
use crate::hci::event::{EventCode, EventPacket, ReturnParameters};
use crate::hci::{ErrorCode, HCIConversionError, HCIPackError, Opcode, FULL_COMMAND_MAX_LEN};
use alloc::boxed::Box;
use core::convert::{TryFrom, TryInto};
use core::future::Future;
use core::pin::Pin;
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
pub enum StreamError {
    CommandError(HCIPackError),
    BadOpcode,
    BadEventCode,
    StreamClosed,
    StreamFailed,
    IOError,
    HCIError(ErrorCode),
}
/*
/// HCI Stream Sink that consumes any HCI Events or Status.
pub trait StreamSink {
    fn consume_event(&self, event: EventPacket<&[u8]>);
}
/// Generic HCI Stream. Abstracted to HCI Command/Event Packets. If you only have access to a
/// HCI Byte Stream, see `byte_stream::ByteStream` instead.
pub trait WriteStream {
    /// Send a HCI Command to the Controller. Responses will be sent to the sink.
    fn send_command<Cmd: Command>(&mut self, command: &Cmd) -> Result<Cmd: , StreamError>;
}
*/

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
        out[12..14].copy_from_slice(&u16::from(self.opcode).to_le_bytes()[..]);
        out
    }
    pub fn unpack(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != FILTER_LEN {
            None
        } else {
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
        }
    }
    pub fn enable_event(&mut self, event: EventCode) {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] |= 1u32 << event;
        } else {
            self.event_mask[1] |= 1u32 << (event - 32);
        }
    }
    pub fn disable_event(&mut self, event: EventCode) {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] &= !(1u32 << event);
        } else {
            self.event_mask[1] &= !(1u32 << (event - 32));
        }
    }
    pub fn get_event(&self, event: EventCode) -> bool {
        let event = u32::from(event);
        assert!(event < 64);
        if event < 32 {
            self.event_mask[0] & (1u32 << event) != 0
        } else {
            self.event_mask[1] & (1u32 << (event - 32)) != 0
        }
    }
    pub fn enable_type(&mut self, packet_type: PacketType) {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask |= 1u32 << packet_type;
    }
    pub fn disable_type(&mut self, packet_type: PacketType) {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask &= !(1u32 << packet_type);
    }
    pub fn get_type(&self, packet_type: PacketType) -> bool {
        let packet_type = packet_type as u32;
        assert!(packet_type < 32);
        self.type_mask & (1u32 << packet_type) != 0
    }
    pub fn opcode(&self) -> Opcode {
        self.opcode
    }
    pub fn opcode_mut(&mut self) -> &mut Opcode {
        &mut self.opcode
    }
}
pub trait HCIFilterable {
    fn set_filter(self: Pin<&mut Self>, filter: &Filter) -> Result<(), StreamError>;
    fn get_filter(self: Pin<&Self>) -> Result<Filter, StreamError>;
}
pub trait HCIWriter {
    /// Work around for async in traits. Traits can't return a generic type with a lifetime bound
    /// to the function call (like below). But they can return a dynamic type with the lifetime bound
    /// to the function call. Sadly, this work around requires boxing the return value which
    /// is non-zero overhead.
    fn send_bytes<'f>(
        self: Pin<&'f mut Self>,
        bytes: &[u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), StreamError>> + 'f>>;
}
pub trait HCIReader {
    /// Work around for async in traits. Traits can't return a generic type with a lifetime bound
    /// to the function call (like below). But they can return a dynamic type with the lifetime bound
    /// to the function call. Sadly, this work around requires boxing the return value which
    /// is non-zero overhead.
    fn read_event<'f>(
        self: Pin<&'f mut Self>,
    ) -> Pin<Box<dyn Future<Output = Option<Result<EventPacket<Box<[u8]>>, StreamError>>> + 'f>>;
}
pub struct HCIStream<S: HCIWriter + HCIReader + HCIFilterable> {
    pub stream: S,
}
pub const HCI_EVENT_READ_TRIES: usize = 10;
impl<S: HCIWriter + HCIReader + HCIFilterable> HCIStream<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
    pub fn stream_pinned(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) }
    }
    pub async fn send_command<Cmd: Command, Return: ReturnParameters>(
        mut self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<Return, StreamError> {
        let mut buf = [0_u8; FULL_COMMAND_MAX_LEN];
        let len = command.full_len();
        // Pack Command
        command
            .pack_full(&mut buf[..len])
            .map_err(StreamError::CommandError)?;
        // New Filter
        let mut filter = Filter::default();
        filter.enable_type(PacketType::Command);
        filter.enable_type(PacketType::Event);

        filter.enable_event(EventCode::CommandStatus);
        filter.enable_event(EventCode::CommandComplete);
        filter.enable_event(EventCode::LEMeta);

        filter.enable_event(Return::EVENT_CODE);
        if !filter.get_event(Return::EVENT_CODE) {
            return Err(StreamError::BadEventCode);
        }
        *filter.opcode_mut() = Cmd::opcode();
        // Save Old Filter
        let old_filter = self.as_mut().stream_pinned().as_ref().get_filter()?;
        self.as_mut().stream_pinned().set_filter(&filter)?;

        // Send command Bytes
        self.as_mut()
            .stream_pinned()
            .send_bytes(&buf[..len])
            .await?;

        // Wait for response
        for _try_i in 0..HCI_EVENT_READ_TRIES {
            let event: EventPacket<Box<[u8]>> = self
                .as_mut()
                .stream_pinned()
                .read_event()
                .await
                .ok_or(StreamError::StreamClosed)??;
            if event.event_code() == Return::EVENT_CODE {
                self.stream_pinned().set_filter(&old_filter)?;
                return Return::unpack_from(event.parameters()).map_err(StreamError::CommandError);
            }
        }
        Err(StreamError::StreamFailed)
    }
}
impl<S: HCIWriter + HCIReader + HCIFilterable> HCIReader for HCIStream<S> {
    fn read_event<'f>(
        self: Pin<&'f mut Self>,
    ) -> Pin<Box<dyn Future<Output = Option<Result<EventPacket<Box<[u8]>>, StreamError>>> + 'f>>
    {
        self.stream_pinned().read_event()
    }
}
impl<S: HCIWriter + HCIReader + HCIFilterable> HCIWriter for HCIStream<S> {
    fn send_bytes<'f>(
        self: Pin<&'f mut Self>,
        bytes: &[u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), StreamError>> + 'f>> {
        self.stream_pinned().send_bytes(bytes)
    }
}
#[cfg(feature = "std")]
pub mod byte {
    use crate::hci::event::EventCode;
    use crate::hci::stream::{Filter, HCIFilterable, HCIReader, HCIWriter, StreamError};
    use crate::hci::FULL_COMMAND_MAX_LEN;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use core::pin::Pin;
    use core::task::Poll;

    use crate::hci::event::EventPacket;
    use core::future::Future;
    use core::task::Context;
    use futures_core::Stream;
    use futures_io::{AsyncRead, AsyncWrite};

    const EVENT_HEADER_LEN: usize = 2;

    pub struct ByteStream<R: AsyncRead> {
        reader: R,
        pos: usize,
        header_buf: [u8; EVENT_HEADER_LEN],
        parameters: Option<Box<[u8]>>,
    }
    impl<R: AsyncRead> ByteStream<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                pos: 0,
                header_buf: [0_u8; EVENT_HEADER_LEN],
                parameters: None,
            }
        }
        /// Clear the Read state from the ByteStream.
        /// If any message is in the process of being received, it will lose all that data.
        pub fn clear(self: Pin<&mut Self>) {
            // Safe because none of these are structually pinned.
            unsafe {
                let s = self.get_unchecked_mut();
                s.pos = 0;
                s.header_buf = Default::default();
                s.parameters = None;
            }
        }
        pub fn reader_pinned_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
            unsafe { self.map_unchecked_mut(|s| &mut s.reader) }
        }
        pub fn reader_pinned(self: Pin<&Self>) -> Pin<&R> {
            unsafe { self.map_unchecked(|s| &s.reader) }
        }
        unsafe fn explode_unsafe(
            &mut self,
        ) -> (
            Pin<&mut R>,
            &mut usize,
            &mut [u8; EVENT_HEADER_LEN],
            &mut Option<Box<[u8]>>,
        ) {
            (
                Pin::new_unchecked(&mut self.reader),
                &mut self.pos,
                &mut self.header_buf,
                &mut self.parameters,
            )
        }
        fn explode_mut(
            self: Pin<&mut Self>,
        ) -> (
            Pin<&mut R>,
            &mut usize,
            &mut [u8; EVENT_HEADER_LEN],
            &mut Option<Box<[u8]>>,
        ) {
            unsafe { self.get_unchecked_mut().explode_unsafe() }
        }
    }
    impl<R: AsyncRead + HCIFilterable> HCIFilterable for ByteStream<R> {
        fn set_filter(self: Pin<&mut Self>, filter: &Filter) -> Result<(), StreamError> {
            self.reader_pinned_mut().set_filter(filter)
        }

        fn get_filter(self: Pin<&Self>) -> Result<Filter, StreamError> {
            self.reader_pinned().get_filter()
        }
    }

    impl<R: AsyncRead> Stream for ByteStream<R> {
        type Item = Result<EventPacket<Box<[u8]>>, StreamError>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let (mut reader, pos, header_buf, parameters_op) = self.explode_mut();
            // This is safe because we structally pin reader in place.
            while *pos < EVENT_HEADER_LEN {
                let amount = match reader.as_mut().poll_read(cx, &mut header_buf[*pos..]) {
                    Poll::Ready(r) => match r {
                        Ok(a) => a,
                        Err(_) => return Poll::Ready(Some(Err(StreamError::IOError))),
                    },
                    Poll::Pending => return Poll::Pending,
                };
                if amount == 0 {
                    return Poll::Ready(None);
                }
                *pos += amount;
            }

            let opcode = match EventCode::try_from(header_buf[0]) {
                Ok(opcode) => opcode,
                Err(_) => return Poll::Ready(Some(Err(StreamError::BadOpcode))),
            };
            let len = usize::from(header_buf[1]);
            let make_buf = || {
                let mut buf = Vec::with_capacity(len);
                buf.resize(len, 0u8);
                buf.into_boxed_slice()
            };

            let buf = {
                if let Some(buf) = parameters_op {
                    buf.as_mut()
                } else {
                    *parameters_op = Some(make_buf());
                    parameters_op
                        .as_mut()
                        .expect("just created buffer with `make_buf()`")
                        .as_mut()
                }
            };
            while *pos < (len + EVENT_HEADER_LEN) {
                let amount = match reader
                    .as_mut()
                    .poll_read(cx, &mut buf[*pos - EVENT_HEADER_LEN..])
                {
                    Poll::Ready(r) => match r {
                        Ok(a) => a,
                        Err(_) => return Poll::Ready(Some(Err(StreamError::IOError))),
                    },
                    Poll::Pending => return Poll::Pending,
                };
                if amount == 0 {
                    return Poll::Ready(None);
                }
                *pos += amount;
            }
            Poll::Ready(Some(Ok(EventPacket::new(
                opcode,
                parameters_op
                    .take()
                    .expect("buffer just filled by poll_read"),
            ))))
        }
    }
    impl<R: AsyncRead> HCIReader for ByteStream<R> {
        fn read_event<'f>(
            mut self: Pin<&'f mut Self>,
        ) -> Pin<Box<dyn Future<Output = Option<Result<EventPacket<Box<[u8]>>, StreamError>>> + 'f>>
        {
            Box::pin(futures_util::future::poll_fn(move |cx| {
                self.as_mut().poll_next(cx)
            }))
        }
    }
    impl<R: AsyncRead + AsyncWrite> HCIWriter for ByteStream<R> {
        fn send_bytes<'f>(
            mut self: Pin<&'f mut Self>,
            bytes: &[u8],
        ) -> Pin<Box<dyn Future<Output = Result<(), StreamError>> + 'f>> {
            self.as_mut().clear();
            Box::pin(ByteWrite::new(self.reader_pinned_mut(), bytes))
        }
    }

    pub struct ByteWrite<'w, W: AsyncWrite> {
        writer: Pin<&'w mut W>,
        data: [u8; FULL_COMMAND_MAX_LEN],
        pos: usize,
        len: usize,
    }
    impl<'w, W: AsyncWrite> ByteWrite<'w, W> {
        pub fn new(writer: Pin<&'w mut W>, data: &[u8]) -> Self {
            let mut buf = [0_u8; FULL_COMMAND_MAX_LEN];
            buf[..data.len()].copy_from_slice(data);
            Self {
                writer,
                data: buf,
                pos: 0,
                len: data.len(),
            }
        }
        pub fn bytes_left(&self) -> usize {
            self.len - self.pos
        }
        pub fn is_done(&self) -> bool {
            self.bytes_left() == 0
        }
        pub fn buf(&self) -> &[u8] {
            &self.data[self.pos..self.len]
        }
        unsafe fn explode_unsafe(
            &mut self,
        ) -> (
            &mut Pin<&'w mut W>,
            &mut [u8; FULL_COMMAND_MAX_LEN],
            &mut usize,
            usize,
        ) {
            (&mut self.writer, &mut self.data, &mut self.pos, self.len)
        }
        pub fn explode_mut(
            self: Pin<&mut Self>,
        ) -> (
            &mut Pin<&'w mut W>,
            &mut [u8; FULL_COMMAND_MAX_LEN],
            &mut usize,
            usize,
        ) {
            // This is safe because we structally pin writer in place.
            unsafe { self.get_unchecked_mut().explode_unsafe() }
        }
    }
    impl<'w, W: AsyncWrite> Future for ByteWrite<'w, W> {
        type Output = Result<(), StreamError>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let (writer, buf, pos, len) = self.explode_mut();
            while *pos < len {
                let amount = match writer.as_mut().poll_write(cx, &buf[*pos..]) {
                    Poll::Ready(result) => match result {
                        Ok(amount) => amount,
                        Err(e) => {
                            eprintln!("error: {:?}", e);
                            return Poll::Ready(Err(StreamError::IOError));
                        }
                    },
                    Poll::Pending => return Poll::Pending,
                };
                *pos += amount;
            }
            match writer.as_mut().poll_flush(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(result) => match result {
                    Ok(_) => Poll::Ready(Ok(())),
                    Err(_) => Poll::Ready(Err(StreamError::IOError)),
                },
            }
        }
    }
}
#[cfg(feature = "std")]
pub use byte::{ByteStream, ByteWrite};
