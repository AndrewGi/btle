pub mod adapter;
pub mod device;
pub mod manager;
pub mod supported;

use crate::error::IOError;

#[derive(Copy, Clone, Debug)]
pub struct Error(pub IOError);
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "USB Error '{:?}'", self.0)
    }
}
impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error(match e {
            rusb::Error::Success => unreachable!("success passed as error type"),
            rusb::Error::Io => IOError::Other,
            rusb::Error::InvalidParam => IOError::InvalidArgument,
            rusb::Error::Access => IOError::AccessDenied,
            rusb::Error::NoDevice => IOError::NotConnected,
            rusb::Error::NotFound => IOError::NotFound,
            rusb::Error::Busy => IOError::Refused,
            rusb::Error::Timeout => IOError::TimedOut,
            rusb::Error::Overflow => IOError::Overflow,
            rusb::Error::Pipe => IOError::Pipe,
            rusb::Error::Interrupted => IOError::Interrupted,
            rusb::Error::NoMem => IOError::OutOfMemory,
            rusb::Error::NotSupported => IOError::NotImplemented,
            rusb::Error::Other => IOError::Other,
        })
    }
}
impl From<Error> for IOError {
    fn from(e: Error) -> Self {
        e.0
    }
}
impl crate::error::Error for Error {}
#[cfg(feature = "std")]
impl std::error::Error for Error {}
