pub mod le;
use crate::bytes::Storage;
use crate::hci::command::Command;
use crate::hci::event::{EventPacket, ReturnParameters};
use crate::hci::stream;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter, Stream};
use core::fmt::Formatter;
use core::pin::Pin;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Error {
    StreamError(stream::Error),
}
impl From<stream::Error> for Error {
    fn from(e: stream::Error) -> Self {
        Error::StreamError(e)
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}
pub struct Adapter<S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> {
    pub stream: Stream<S>,
    _marker: core::marker::PhantomData<Buf>,
}
impl<S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> Adapter<S, Buf> {
    pub fn new(stream: Stream<S>) -> Self {
        Self {
            stream,
            _marker: Default::default(),
        }
    }
    pub fn into_stream(self) -> Stream<S> {
        self.stream
    }
    fn stream_pinned(self: Pin<&mut Self>) -> Pin<&mut Stream<S>> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) }
    }
    pub fn le(self: Pin<&mut Self>) -> le::LEAdapter<S, Buf> {
        le::LEAdapter::new(self)
    }
    pub async fn send_command<Cmd: Command, Return: ReturnParameters>(
        self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<Return, Error> {
        self.stream_pinned()
            .send_command::<Cmd, Return, Buf>(command)
            .await
            .map_err(Error::StreamError)
    }
    pub fn with_buf<NewBuf: Storage>(self) -> Adapter<S, NewBuf> {
        Adapter {
            stream: self.stream,
            _marker: Default::default(),
        }
    }
    pub async fn read_event(self: Pin<&mut Self>) -> Result<EventPacket<Buf>, Error> {
        Ok(self.stream_pinned().read_event().await?)
    }
}
impl<S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> AsRef<Stream<S>> for Adapter<S, Buf> {
    fn as_ref(&self) -> &Stream<S> {
        &self.stream
    }
}

impl<S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> AsMut<Stream<S>> for Adapter<S, Buf> {
    fn as_mut(&mut self) -> &mut Stream<S> {
        &mut self.stream
    }
}
