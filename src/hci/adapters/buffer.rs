use crate::bytes::Storage;
use crate::hci::adapter::Error;
use crate::hci::adapters::UnrecognizedEventHandler;
use crate::hci::event::EventPacket;
use alloc::collections::VecDeque;

pub struct HCIEventBuffer<Buf> {
    deque: VecDeque<EventPacket<Buf>>,
}
impl<Buf> HCIEventBuffer<Buf> {
    pub fn new() -> Self {
        HCIEventBuffer {
            deque: VecDeque::new(),
        }
    }
    pub fn into_inner(self) -> VecDeque<EventPacket<Buf>> {
        self.deque
    }
    pub fn push(&mut self, event: EventPacket<Buf>) {
        self.deque.push_back(event)
    }
    pub fn pop(&mut self) -> Option<EventPacket<Buf>> {
        self.deque.pop_front()
    }
    pub fn drain(
        &mut self,
        range: impl core::ops::RangeBounds<usize>,
    ) -> alloc::collections::vec_deque::Drain<EventPacket<Buf>> {
        self.deque.drain(range)
    }
}
impl<Buf> Iterator for HCIEventBuffer<Buf> {
    type Item = EventPacket<Buf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
impl<Buf: Storage<u8>> UnrecognizedEventHandler for HCIEventBuffer<Buf> {
    type Buf = Buf;

    fn handle(&mut self, event: EventPacket<Self::Buf>) -> Result<(), Error> {
        self.push(event);
        Ok(())
    }
}
