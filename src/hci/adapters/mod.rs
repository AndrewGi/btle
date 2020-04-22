//! Contains logic for HCI Adapters (usually byte streams).
pub mod le;

use crate::bytes::Storage;
use crate::hci::adapter;
use crate::hci::baseband::{EventMask, Reset, SetEventMask};
use crate::hci::command::Command;
use crate::hci::event::EventPacket;
use crate::hci::le::mask::{MetaEventMask, SetMetaEventMask};
use crate::Stream;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;

pub struct Adapter<A: adapter::Adapter, S: Deref<Target = A>> {
    pub adapter: Pin<S>,
}
impl<A: adapter::Adapter, S: Deref<Target = A> + DerefMut> Adapter<A, S> {
    pub fn new(adapter: Pin<S>) -> Self {
        Self { adapter }
    }
    pub fn pin(adapter: S) -> Self
    where
        A: Unpin,
    {
        Self::new(Pin::new(adapter))
    }
    pub fn adapter_mut(&mut self) -> Pin<&mut A> {
        self.adapter.as_mut()
    }
    pub fn adapter_ref(&self) -> Pin<&A> {
        self.adapter.as_ref()
    }
    pub fn as_ref(&self) -> Adapter<A, &'_ A> {
        Adapter {
            adapter: self.adapter.as_ref(),
        }
    }
    pub fn as_mut(&mut self) -> Adapter<A, &'_ mut A> {
        Adapter {
            adapter: self.adapter.as_mut(),
        }
    }
    pub async fn hci_send_command<'a, 'c: 'a, Cmd: Command + 'c>(
        &mut self,
        cmd: Cmd,
    ) -> Result<Cmd::Return, adapter::Error> {
        self.adapter_mut().send_command(cmd).await
    }
    pub async fn hci_read_event<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, adapter::Error> {
        self.adapter_mut().read_event().await
    }
    pub fn hci_event_stream<'a, 'b: 'a, Buf: Storage<u8> + 'b>(
        &'a mut self,
    ) -> impl Stream<Item = Result<EventPacket<Buf>, adapter::Error>> + 'a {
        todo!("set HCI Filter to AdvertisingReport");
        futures_util::stream::unfold(self, move |s| async move {
            Some((s.adapter.as_mut().read_event().await, s))
        })
    }
    pub async fn set_event_mask(&mut self, mask: EventMask) -> Result<(), adapter::Error> {
        self.hci_send_command(SetEventMask(mask))
            .await?
            .status
            .error()?;
        Ok(())
    }
    pub async fn reset(&mut self) -> Result<(), adapter::Error> {
        self.hci_send_command(Reset).await?.status.error()?;
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
