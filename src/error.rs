//! Generic Error Trait. Similar to `std::error::Error`.

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
    Pipe,
    Overflow,
    Other,
    Code(i32),
}
impl core::fmt::Display for IOError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for IOError {}

#[cfg(feature = "std")]
impl From<std::io::ErrorKind> for IOError {
    fn from(e: std::io::ErrorKind) -> Self {
        match e {
            std::io::ErrorKind::NotFound => IOError::NotFound,
            std::io::ErrorKind::PermissionDenied => IOError::NotFound,
            std::io::ErrorKind::ConnectionRefused => IOError::Refused,
            std::io::ErrorKind::ConnectionReset => IOError::NotConnected,
            std::io::ErrorKind::ConnectionAborted => IOError::NotConnected,
            std::io::ErrorKind::NotConnected => IOError::NotConnected,
            std::io::ErrorKind::AddrInUse => IOError::AlreadyExists,
            std::io::ErrorKind::AddrNotAvailable => IOError::Refused,
            std::io::ErrorKind::BrokenPipe => IOError::Closed,
            std::io::ErrorKind::AlreadyExists => IOError::AlreadyExists,
            std::io::ErrorKind::WouldBlock => IOError::Other,
            std::io::ErrorKind::InvalidInput => IOError::InvalidArgument,
            std::io::ErrorKind::InvalidData => IOError::InvalidData,
            std::io::ErrorKind::TimedOut => IOError::TimedOut,
            std::io::ErrorKind::WriteZero => IOError::InvalidArgument,
            std::io::ErrorKind::Interrupted => IOError::Interrupted,
            std::io::ErrorKind::Other => IOError::Other,
            std::io::ErrorKind::UnexpectedEof => IOError::InvalidData,
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

#[cfg(feature = "std")]
impl std::error::Error for IOError {}
