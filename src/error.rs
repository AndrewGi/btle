/// Generic Error Trait. Similar to `std::error::Error`.
use core::fmt::Debug;
use std::fmt::Formatter;

/// Generic Error type. Similar to `std::error::Error` but supports `no_std`. If the `std` feature
/// is enabled, `Error` will implement `std::error::Error`. Automatically implements `fmt::Display`
/// by using the `Debug` implementation (`"{:?}"`).
pub trait Error: Debug {
    /// The lower-level source of this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct STDError<E: Error + ?Sized>(pub E);
impl<E: Error> From<E> for STDError<E> {
    fn from(e: E) -> Self {
        Self(e)
    }
}
impl<E: Error> core::fmt::Debug for STDError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}
impl<T: Error> core::fmt::Display for STDError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

impl<E: Error> Error for STDError<E> {}
#[cfg(feature = "std")]
impl<E: Error> std::error::Error for STDError<E> {}
