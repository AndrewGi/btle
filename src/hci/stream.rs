use crate::hci::{Command, ErrorCode, EventPacket, HCIConversionError, HCIPackError};
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
#[cfg(std)]
pub mod byte {
    use crate::hci::stream::{HCIReader, StreamError};
    use crate::hci::EventPacket;
    use futures::AsyncRead;

    pub struct HCIByteReader<R: AsyncRead>(R);
    impl<R: AsyncRead> HCIReader for HCIByteReader<R> {
        async fn read_event<Fut>(&mut self) -> Result<EventPacket<Box<[u8]>>, StreamError> {
            let mut code_len_buf = [0_u8; 3];
            let mut pos = 0;
            while pos < code_len_buf.len() {
                let amount = self.0.poll_read(&mut code_len_buf[pos..]).await;
            }
        }
    }
}
