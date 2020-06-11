use crate::hci::adapters::DummyUnrecognizedEventHandler;

pub struct Central<A: crate::hci::adapter::Adapter> {
    pub hci_adapter: crate::hci::adapters::le::LEAdapter<A, DummyUnrecognizedEventHandler>,
}
impl<A: crate::hci::adapter::Adapter> Central<A> {}
