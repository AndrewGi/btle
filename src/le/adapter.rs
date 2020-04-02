use crate::error::IOError;
use crate::hci;
use crate::hci::{ErrorCode, StreamError};

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
    fn from(e: StreamError) -> Self {
        Error::StreamError(e)
    }
}
impl From<hci::ErrorCode> for Error {
    fn from(e: ErrorCode) -> Self {
        Error::ErrorCode(e)
    }
}
