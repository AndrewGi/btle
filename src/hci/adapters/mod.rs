//! Contains logic for HCI Adapters (usually byte streams).
pub mod le;

use crate::{
    bytes::Storage,
    error,
    hci::{
        command::Command,
        event::CommandComplete,
        packet::RawPacket,
        stream::{HCIStreamable, Stream},
    },
    le::adapter::Error,
};
use core::fmt::Formatter;
use core::pin::Pin;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}
pub struct Adapter<R: HCIStreamable> {
    pub stream: Stream<R>,
    _marker: (),
}
impl<R: HCIStreamable> Adapter<R> {
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
    /// Read a `RawPacket`
    pub async fn read_packet<S: Storage<u8>>(self: Pin<&mut Self>) -> Result<RawPacket<S>, Error> {
        const PACKET_SIZE: usize = 255 + 2;
        let mut buf = [0_u8; PACKET_SIZE];
        Ok(self
            .stream_pinned()
            .read_packet(&mut buf[..])
            .await?
            .clone_buf())
    }
}
impl<R: HCIStreamable> AsRef<Stream<R>> for Adapter<R> {
    fn as_ref(&self) -> &Stream<R> {
        &self.stream
    }
}

impl<R: HCIStreamable> AsMut<Stream<R>> for Adapter<R> {
    fn as_mut(&mut self) -> &mut Stream<R> {
        &mut self.stream
    }
}
