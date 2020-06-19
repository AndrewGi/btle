//! Contains logic for HCI Adapters (usually byte streams).
pub mod buffer;
pub mod le;

use crate::bytes::Storage;
use crate::hci::adapter;
use crate::hci::adapters::le::LEAdapter;
use crate::hci::baseband::{EventMask, Reset, SetEventMask};
use crate::hci::command::Command;
use crate::hci::event::EventPacket;
use crate::Stream;

// TODO: Make this more generic
pub trait UnrecognizedEventHandler {
    type Buf: Storage<u8>;
    fn handle(&mut self, event: EventPacket<Self::Buf>) -> Result<(), adapter::Error>;
}
pub struct DummyUnrecognizedEventHandler<Buf = Box<[u8]>>(core::marker::PhantomData<Buf>);
impl<Buf> DummyUnrecognizedEventHandler<Buf> {
    pub const fn new() -> Self {
        Self {
            0: core::marker::PhantomData,
        }
    }
}
impl<Buf: Storage<u8>> UnrecognizedEventHandler for DummyUnrecognizedEventHandler<Buf> {
    type Buf = Buf;

    fn handle(&mut self, event: EventPacket<Buf>) -> Result<(), adapter::Error> {
        // Ignore the event
        core::mem::drop(event);
        Ok(())
    }
}
pub struct Adapter<A: adapter::Adapter, H: UnrecognizedEventHandler> {
    pub adapter: A,
    pub event_handler: H,
}
impl<A: adapter::Adapter> Adapter<A, DummyUnrecognizedEventHandler<Box<[u8]>>> {
    pub fn new(adapter: A) -> Self {
        Adapter {
            adapter,
            event_handler: DummyUnrecognizedEventHandler::new(),
        }
    }
}
impl<A: adapter::Adapter, H: UnrecognizedEventHandler> Adapter<A, H> {
    pub fn new_with_handler(adapter: A, event_handler: H) -> Self {
        Self {
            adapter,
            event_handler,
        }
    }
    pub fn le(self) -> le::LEAdapter<A, H> {
        LEAdapter::new(self)
    }
    pub async fn hci_send_command<'a, 'c: 'a, Cmd: Command + 'c>(
        &mut self,
        cmd: Cmd,
    ) -> Result<Cmd::Return, adapter::Error> {
        let event_handler = &mut self.event_handler;
        adapter::send_command::<_, _, H::Buf, _>(
            &mut self.adapter,
            cmd,
            Some(|e| event_handler.handle(e)),
        )
        .await
    }
    pub async fn hci_read_event<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, adapter::Error> {
        self.adapter.read_event().await
    }
    pub fn hci_event_stream<'a, 'b: 'a, Buf: Storage<u8> + 'b>(
        &'a mut self,
    ) -> impl Stream<Item = Result<EventPacket<Buf>, adapter::Error>> + 'a {
        futures_util::stream::unfold(self, move |s| async move {
            Some((s.adapter.read_event().await, s))
        })
    }
    pub async fn set_event_mask(&mut self, mask: EventMask) -> Result<(), adapter::Error> {
        self.hci_send_command(SetEventMask(mask))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn reset(&mut self) -> Result<(), adapter::Error> {
        self.hci_send_command(Reset).await?.params.status.error()?;
        Ok(())
    }
}
/*
use crate::hci::{
    adapter::Error,
    command::Command,
    event::CommandComplete,
    packet::RawPacket,
    stream::{HCIStreamable, Stream},
};
use core::pin::Pin;
use crate::bytes::Storage;

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
*/
