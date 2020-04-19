use crate::hci;
use crate::hci::command::Command;
use crate::hci::event::EventPacket;
use driver_async::asyncs::future::BoxFuture;
use driver_async::bytes::Storage;
use driver_async::error::IOError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum Error {
    BadParameter,
    IOError(IOError),
    StreamError(hci::StreamError),
    ErrorCode(hci::ErrorCode),
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::IOError(e)
    }
}
impl From<hci::StreamError> for Error {
    fn from(e: hci::StreamError) -> Self {
        Error::StreamError(e)
    }
}
impl From<hci::ErrorCode> for Error {
    fn from(e: hci::ErrorCode) -> Self {
        Error::ErrorCode(e)
    }
}

///WIP HCI Adapter trait
pub trait Adapter {
    fn send_command<Cmd: Command>(
        &mut self,
        command: Cmd,
    ) -> BoxFuture<'_, Result<Cmd::Return, Error>>;
    fn read_event<S: Storage<u8>>(&mut self) -> BoxFuture<'_, Result<EventPacket<S>, Error>>;
}
