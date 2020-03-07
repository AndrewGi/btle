pub mod le;
use crate::bytes::Storage;
use crate::hci::command::Command;
use crate::hci::event::ReturnParameters;
use crate::hci::stream;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter, Stream};
use core::fmt::Formatter;
use core::pin::Pin;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Error {
    StreamError(stream::Error),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}
pub struct Adapter<S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> {
    pub stream: Stream<S, Buf>,
    _marker: (),
}
impl<S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> Adapter<S, Buf> {
    pub fn new(stream: Stream<S, Buf>) -> Self {
        Self {
            stream,
            _marker: (),
        }
    }
    pub fn into_stream(self) -> Stream<S, Buf> {
        self.stream
    }
    fn stream_pinned(self: Pin<&mut Self>) -> Pin<&mut Stream<S, Buf>> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) }
    }
    pub fn le(&mut self) -> le::LEAdapter<S, Buf> {
        le::LEAdapter::new(self)
    }
    pub async fn send_command<Cmd: Command, Return: ReturnParameters>(
        self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<Return, Error> {
        self.stream_pinned()
            .send_command::<Cmd, Return>(command)
            .await
            .map_err(Error::StreamError)
    }
}
impl<S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> AsRef<Stream<S, Buf>>
    for Adapter<S, Buf>
{
    fn as_ref(&self) -> &Stream<S, Buf> {
        &self.stream
    }
}

impl<S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> AsMut<Stream<S, Buf>>
    for Adapter<S, Buf>
{
    fn as_mut(&mut self) -> &mut Stream<S, Buf> {
        &mut self.stream
    }
}
