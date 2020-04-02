//! Generic Error Trait. Similar to `std::error::Error`.

use futures_io::ErrorKind;

/// Generic Error type. Similar to `std::error::Error` but supports `no_std`. If the `std` feature
/// is enabled, `Error` will implement `std::error::Error`. Automatically implements `fmt::Display`
/// by using the `Debug` implementation (`"{:?}"`).
pub trait Error: core::fmt::Debug {
    /// The lower-level source of this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct StdError<E: Error + ?Sized>(pub E);
impl<E: Error> From<E> for StdError<E> {
    fn from(e: E) -> Self {
        Self(e)
    }
}
impl<E: Error> core::fmt::Debug for StdError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}
impl<T: Error> core::fmt::Display for StdError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

impl<E: Error> Error for StdError<E> {}
#[cfg(feature = "std")]
impl<E: Error> std::error::Error for StdError<E> {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum IOError {
    Unknown,
    TimedOut,
    NotFound,
    OperationAborted,
    InvalidArgument,
    InvalidHandlePointer,
    InvalidData,
    AccessDenied,
    OutOfMemory,
    PermissionDenied,
    Closed,
    NotImplemented,
    NotConnected,
    Interrupted,
    IllegalCall,
    AlreadyExists,
    Refused,
    Other,
    Code(i32),
}

impl Error for IOError {}
#[cfg(feature = "winrt")]
impl From<winrt::Error> for IOError {
    fn from(e: winrt::Error) -> Self {
        match e {
            winrt::Error::OperationAborted => IOError::OperationAborted,
            winrt::Error::AccessDenied => IOError::AccessDenied,
            winrt::Error::UnspecifiedFailure => IOError::Unknown,
            winrt::Error::InvalidHandle => IOError::InvalidHandlePointer,
            winrt::Error::InvalidArgument => IOError::InvalidArgument,
            winrt::Error::NoSuchInterface => IOError::NotFound,
            winrt::Error::NotImplemented => IOError::NotImplemented,
            winrt::Error::OutOfMemory => IOError::OutOfMemory,
            winrt::Error::InvalidPointer => IOError::InvalidHandlePointer,
            winrt::Error::UnexpectedFailure => IOError::Unknown,
            winrt::Error::OutOfBounds => IOError::OutOfMemory,
            winrt::Error::ChangedState => IOError::Other,
            winrt::Error::IllegalMethodCall => IOError::IllegalCall,
            winrt::Error::ObjectClosed => IOError::Closed,
            winrt::Error::Other(i) => IOError::Code(i),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::ErrorKind> for IOError {
    fn from(e: std::io::ErrorKind) -> Self {
        match e {
            ErrorKind::NotFound => IOError::NotFound,
            ErrorKind::PermissionDenied => IOError::NotFound,
            ErrorKind::ConnectionRefused => IOError::Refused,
            ErrorKind::ConnectionReset => IOError::NotConnected,
            ErrorKind::ConnectionAborted => IOError::NotConnected,
            ErrorKind::NotConnected => IOError::NotConnected,
            ErrorKind::AddrInUse => IOError::AlreadyExists,
            ErrorKind::AddrNotAvailable => IOError::Refused,
            ErrorKind::BrokenPipe => IOError::Closed,
            ErrorKind::AlreadyExists => IOError::AlreadyExists,
            ErrorKind::WouldBlock => IOError::Other,
            ErrorKind::InvalidInput => IOError::InvalidArgument,
            ErrorKind::InvalidData => IOError::InvalidData,
            ErrorKind::TimedOut => IOError::TimedOut,
            ErrorKind::WriteZero => IOError::InvalidArgument,
            ErrorKind::Interrupted => IOError::Interrupted,
            ErrorKind::Other => IOError::Other,
            ErrorKind::UnexpectedEof => IOError::InvalidData,
            _ => IOError::Other,
        }
    }
}
#[cfg(feature = "std")]
impl From<&std::io::Error> for IOError {
    fn from(e: &std::io::Error) -> Self {
        e.kind().into()
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for IOError {
    fn from(e: std::io::Error) -> Self {
        e.kind().into()
    }
}
