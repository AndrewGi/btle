pub mod le;
use crate::bytes::Storage;
use crate::hci::command::Command;
use crate::hci::event::CommandComplete;
use crate::hci::packet::RawPacket;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter, Stream};
use crate::hci::{stream, ErrorCode};
use crate::{error, hci};
use core::fmt::Formatter;
use core::pin::Pin;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Error {
    StreamError(stream::Error),
    ErrorCode(hci::ErrorCode),
}
impl From<stream::Error> for Error {
    fn from(e: stream::Error) -> Self {
        Error::StreamError(e)
    }
}
impl From<hci::ErrorCode> for Error {
    fn from(e: ErrorCode) -> Self {
        Error::ErrorCode(e)
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}
pub struct Adapter<R: HCIWriter + HCIReader + HCIFilterable> {
    pub stream: Stream<R>,
    _marker: (),
}
impl<R: HCIWriter + HCIReader + HCIFilterable> Adapter<R> {
    pub fn new(stream: Stream<R>) -> Self {
        Self {
            stream,
            _marker: Default::default(),
        }
    }
    pub fn into_stream(self) -> Stream<R> {
        self.stream
    }
    pub fn stream_pinned(self: Pin<&mut Self>) -> Pin<&mut Stream<R>> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) }
    }
    pub fn le(self: Pin<&mut Self>) -> le::LEAdapter<R> {
        le::LEAdapter::new(self)
    }
    pub async fn send_command<Cmd: Command>(
        self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<CommandComplete<Cmd::Return>, Error> {
        self.stream_pinned()
            .send_command(command)
            .await
            .map_err(Error::StreamError)
    }
    /// Read a
    pub async fn read_packet<S: Storage>(self: Pin<&mut Self>) -> Result<RawPacket<S>, Error> {
        const PACKET_SIZE: usize = 255 + 2;
        let mut buf = [0_u8; PACKET_SIZE];
        Ok(self
            .stream_pinned()
            .read_packet(&mut buf[..])
            .await?
            .clone_buf())
    }
}
impl<R: HCIWriter + HCIReader + HCIFilterable> AsRef<Stream<R>> for Adapter<R> {
    fn as_ref(&self) -> &Stream<R> {
        &self.stream
    }
}

impl<R: HCIWriter + HCIReader + HCIFilterable> AsMut<Stream<R>> for Adapter<R> {
    fn as_mut(&mut self) -> &mut Stream<R> {
        &mut self.stream
    }
}
