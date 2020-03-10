pub mod le;
use crate::bytes::Storage;
use crate::hci::command::Command;
use crate::hci::event::CommandComplete;
use crate::hci::packet::RawPacket;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter, Stream};
use crate::hci::{stream, ErrorCode};
use crate::{error, hci};
use core::fmt::Formatter;
use core::future::Future;
use core::pin::Pin;
use futures_task::{Context, Poll};

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
pub struct Adapter<S: HCIWriter + HCIReader + HCIFilterable> {
    pub stream: Stream<S>,
    _marker: (),
}
impl<S: HCIWriter + HCIReader + HCIFilterable> Adapter<S> {
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
    pub fn le(self: Pin<&mut Self>) -> le::LEAdapter<S> {
        le::LEAdapter::new(self)
    }
    pub async fn send_command<Cmd: Command>(
        self: Pin<&mut Self>,
        command: Cmd,
    ) -> Result<CommandComplete<Cmd::Return>, Error> {
        self.stream_pinned()
            .send_command::<Cmd>(command)
            .await
            .map_err(Error::StreamError)
    }
}
impl<S: HCIWriter + HCIReader + HCIFilterable> AsRef<Stream<S>> for Adapter<S> {
    fn as_ref(&self) -> &Stream<S> {
        &self.stream
    }
}

impl<S: HCIWriter + HCIReader + HCIFilterable> AsMut<Stream<S>> for Adapter<S> {
    fn as_mut(&mut self) -> &mut Stream<S> {
        &mut self.stream
    }
}

pub struct PacketStream<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> {
    adapter: &'a mut Adapter<S>,
    buf: Buf,
}
impl<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> futures_core::stream::Stream
    for PacketStream<'a, S, Buf>
{
    type Item = Result<RawPacket<Buf>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = &mut *self;
        Pin::new(&mut this.adapter.stream.read_packet(this.buf.as_mut()))
            .poll(cx)
            .map(|r| Some(r.map(|p| p.clone_buf()).map_err(Error::StreamError)))
    }
}
