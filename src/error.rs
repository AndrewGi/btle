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
