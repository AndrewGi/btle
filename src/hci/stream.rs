use crate::hci::{
    Command, ErrorCode, EventPacket, HCIConversionError, HCIPackError, FULL_COMMAND_MAX_LEN,
};
use alloc::boxed::Box;
use core::convert::TryFrom;

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

pub trait HCIWriter<'w> {
    type WriteFuture: core::future::Future<Output = Result<(), StreamError>> + 'w;
    fn send_command<Cmd: Command>(
        &'w mut self,
        command: Cmd,
    ) -> Result<Self::WriteFuture, HCIPackError> {
        let mut buf = [0_u8; FULL_COMMAND_MAX_LEN];
        let len = command.full_len();
        command.pack_full(&mut buf[..len])?;
        Ok(self.send_bytes(&buf[..len]))
    }
    fn send_bytes(&'w mut self, bytes: &[u8]) -> Self::WriteFuture;
}
pub trait HCIReader<'r> {
    type EventFuture: core::future::Future<Output = Option<Result<EventPacket<Box<[u8]>>, StreamError>>>
        + 'r;
    fn read_event(&'r mut self) -> Self::EventFuture;
}
#[cfg(feature = "std")]
pub mod byte {
    use crate::hci::stream::{HCIReader, HCIWriter, StreamError};
    use crate::hci::{EventCode, EventPacket, FULL_COMMAND_MAX_LEN};
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use core::pin::Pin;
    use core::task::Poll;

    use futures::task::Context;
    use futures::{AsyncRead, AsyncWrite, Stream, StreamExt};

    const EVENT_HEADER_LEN: usize = 2;

    pub struct ByteStream<'r, R: AsyncRead + Unpin> {
        reader: &'r mut R,
        pos: usize,
        header_buf: [u8; EVENT_HEADER_LEN],
        parameters: Option<Box<[u8]>>,
    }
    impl<'r, R: AsyncRead + Unpin> ByteStream<'r, R> {
        pub fn new(reader: &'r mut R) -> Self {
            Self {
                reader,
                pos: 0,
                header_buf: [0_u8; EVENT_HEADER_LEN],
                parameters: None,
            }
        }
        /// Clear the Read state from the ByteStream.
        /// If any message is in the process of being received, it will lose all that data.
        pub fn clear(&mut self) {
            self.pos = 0;
            self.header_buf = Default::default();
            self.parameters = None
        }
    }
    impl<'r, R: AsyncRead + Unpin> Stream for ByteStream<'r, R> {
        type Item = Result<EventPacket<Box<[u8]>>, StreamError>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            println!("poll next {}", self.pos);
            while self.pos < EVENT_HEADER_LEN {
                let pos = self.pos;
                let me = &mut *self;
                let amount =
                    match Pin::new(&mut *me.reader).poll_read(cx, &mut me.header_buf[pos..]) {
                        Poll::Ready(r) => match r {
                            Ok(a) => a,
                            Err(_) => return Poll::Ready(Some(Err(StreamError::IOError))),
                        },
                        Poll::Pending => return Poll::Pending,
                    };
                println!("read something");
                if amount == 0 {
                    return Poll::Ready(None);
                }
                self.pos += amount;
            }

            let opcode = match EventCode::try_from(self.header_buf[0]) {
                Ok(opcode) => opcode,
                Err(_) => return Poll::Ready(Some(Err(StreamError::BadOpcode))),
            };
            let len = usize::from(self.header_buf[1]);
            let make_buf = || {
                let mut buf = Vec::with_capacity(len);
                buf.resize(len, 0u8);
                buf.into_boxed_slice()
            };

            let me = &mut *self;
            let buf = {
                if let Some(buf) = &mut me.parameters {
                    buf.as_mut()
                } else {
                    me.parameters = Some(make_buf());
                    me.parameters
                        .as_mut()
                        .expect("just created buffer with `make_buf()`")
                        .as_mut()
                }
            };
            while me.pos < (len + EVENT_HEADER_LEN) {
                let pos = me.pos;
                let amount = match Pin::new(&mut *me.reader)
                    .poll_read(cx, &mut buf[pos - EVENT_HEADER_LEN..])
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
                me.pos += amount;
            }
            Poll::Ready(Some(Ok(EventPacket::new(
                opcode,
                self.parameters
                    .take()
                    .expect("buffer just filled by poll_read"),
            ))))
        }
    }
    impl<'f, 'r: 'f, R: AsyncRead + Unpin> HCIReader<'f> for ByteStream<'r, R> {
        type EventFuture = futures::stream::Next<'f, Self>;

        fn read_event(&'f mut self) -> Self::EventFuture {
            self.next()
        }
    }
    impl<'w, 'r: 'w, R: AsyncRead + Unpin + AsyncWrite> HCIWriter<'w> for ByteStream<'r, R> {
        type WriteFuture = ByteWrite<'w, R>;

        fn send_bytes(&'w mut self, bytes: &[u8]) -> ByteWrite<'w, R> {
            self.clear();
            println!("send");
            ByteWrite::new(self.reader, bytes)
        }
    }

    pub struct ByteWrite<'w, W: AsyncWrite + Unpin> {
        writer: &'w mut W,
        data: [u8; FULL_COMMAND_MAX_LEN],
        pos: usize,
        len: usize,
    }
    impl<'w, W: AsyncWrite + Unpin> ByteWrite<'w, W> {
        pub fn new(writer: &'w mut W, data: &[u8]) -> Self {
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
        pub fn pinned_writer(&mut self) -> Pin<&mut W> {
            Pin::new(self.writer)
        }
    }
    impl<'w, W: AsyncWrite + Unpin> core::future::Future for ByteWrite<'w, W> {
        type Output = Result<(), StreamError>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let me = &mut *self;
            let len = me.len;
            let pos = &mut me.pos;
            let buf = &me.data[..me.len];
            println!("poller");
            while *pos < len {
                let amount = match Pin::new(&mut *me.writer).poll_write(cx, &buf[*pos..]) {
                    Poll::Ready(result) => match result {
                        Ok(amount) => amount,
                        Err(_) => return Poll::Ready(Err(StreamError::IOError)),
                    },
                    Poll::Pending => return Poll::Pending,
                };
                *pos += amount;
            }
            println!("flush");
            match Pin::new(&mut *me.writer).poll_flush(cx) {
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
