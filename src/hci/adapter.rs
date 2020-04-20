use crate::hci;
use crate::hci::event::EventPacket;
use crate::hci::packet::RawPacket;
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
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "hci adapter error {:?}", self)
    }
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
#[cfg(feature = "hci_usb")]
impl From<hci::usb::Error> for Error {
    fn from(e: hci::usb::Error) -> Self {
        Error::IOError(e.0)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl driver_async::error::Error for Error {}
///WIP HCI Adapter trait
pub trait Adapter {
    fn write_packet(&mut self, _packet: RawPacket<&[u8]>) -> Result<(), Error>;
    fn read_event<S: Storage<u8>>(&mut self) -> BoxFuture<'_, Result<EventPacket<S>, Error>>;
}
