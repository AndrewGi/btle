use crate::hci::{Command, ErrorCode, EventPacket, HCIConversionError, HCIPackError};
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

pub trait HCIWriter {
    fn send_command<Fut, Cmd: Command>(&mut self, command: Cmd) -> Fut
    where
        Fut: core::future::Future<Output = Result<Cmd::Return, StreamError>>;
}
pub trait HCIReader {
    fn read_event<Fut>(&mut self) -> Fut
    where
        Fut: core::future::Future<Output = Result<EventPacket<Box<[u8]>>, StreamError>>;
}
#[cfg(feature = "std")]
pub mod byte {
    use crate::hci::stream::StreamError;
    use crate::hci::{EventCode, EventPacket};
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use core::pin::Pin;
    use core::task::Poll;

    use futures::task::Context;
    use futures::{AsyncRead, Stream};

    const EVENT_HEADER_LEN: usize = 2;

    pub struct HCIByteReader<'r, R: AsyncRead + Unpin> {
        reader: &'r mut R,
        pos: usize,
        header_buf: [u8; EVENT_HEADER_LEN],
        parameters: Option<Box<[u8]>>,
    }

    impl<'r, R: AsyncRead + Unpin> Stream for HCIByteReader<'r, R> {
        type Item = Result<EventPacket<Box<[u8]>>, StreamError>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
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
}
