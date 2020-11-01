pub mod adapter;
pub mod device;
pub mod supported;

use crate::error::IOError;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct Error(pub IOError);
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "USB Error '{:?}'", self.0)
    }
}
impl From<usbw::libusb::error::Error> for Error {
    fn from(e: usbw::libusb::error::Error) -> Self {
        Error(match e {
            usbw::libusb::error::Error::Io => IOError::Other,
            usbw::libusb::error::Error::InvalidParam => IOError::InvalidArgument,
            usbw::libusb::error::Error::Access => IOError::AccessDenied,
            usbw::libusb::error::Error::NoDevice => IOError::NotConnected,
            usbw::libusb::error::Error::NotFound => IOError::NotFound,
            usbw::libusb::error::Error::Busy => IOError::Refused,
            usbw::libusb::error::Error::Timeout => IOError::TimedOut,
            usbw::libusb::error::Error::Overflow => IOError::Overflow,
            usbw::libusb::error::Error::Pipe => IOError::Pipe,
            usbw::libusb::error::Error::Interrupted => IOError::Interrupted,
            usbw::libusb::error::Error::NoMem => IOError::OutOfMemory,
            usbw::libusb::error::Error::NotSupported => IOError::NotImplemented,
            usbw::libusb::error::Error::Other => IOError::Other,
            usbw::libusb::error::Error::BadDescriptor => IOError::InvalidArgument,
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
