pub mod ble;
use driver_async::error::IOError;
pub struct WindowsError(pub IOError);
#[cfg(feature = "winrt")]
impl From<winrt::Error> for WindowsError {
    fn from(e: winrt::Error) -> Self {
        WindowsError(match e {
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
        })
    }
}
