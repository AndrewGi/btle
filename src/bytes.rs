use core::convert::TryInto;

#[derive(Copy, Clone)]
pub enum Endian {
    Big,
    Little,
}
impl Endian {
    #[cfg(target_endian = "big")]
    const fn _native() -> Endian {
        Endian::Big
    }

    #[cfg(target_endian = "little")]
    const fn _native() -> Endian {
        Endian::Little
    }
    /// Returns the target platform `Endian`. This is a NOP const fn.
    pub const fn native() -> Endian {
        Self::_native()
    }
    pub const NATIVE: Endian = Self::native();
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum BufError {
    /// The given index/`usize` is out of range. (overflow, underflow)
    OutOfRange(usize),
    /// The given index/`usize` is invalid. (misaligned, non-sensible)
    InvalidIndex(usize),
    /// The bytes at positive/`usize` are invalid. Used if its possible to pass a 'bad' sequence of
    /// bytes values. (Ex: Trying to turn a byte that isn't a 0 or 1 into a `bool`).
    BadBytes(usize),
    /// Input is completely invalid. Used when unable to pinpoint an index where the bad bytes are.
    InvalidInput,
}
pub trait ToFromBytesEndian: Sized {
    type AsBytesType: AsRef<[u8]>;

    #[must_use]
    fn byte_size() -> usize {
        core::mem::size_of::<Self::AsBytesType>()
    }

    #[must_use]
    fn to_bytes_le(&self) -> Self::AsBytesType;

    #[must_use]
    fn to_bytes_be(&self) -> Self::AsBytesType;

    #[must_use]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        if cfg!(target_endian = "big") {
            self.to_bytes_be()
        } else {
            self.to_bytes_le()
        }
    }
    #[must_use]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self>;

    #[must_use]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self>;

    #[must_use]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        if cfg!(target_endian = "big") {
            Self::from_bytes_be(bytes)
        } else {
            Self::from_bytes_le(bytes)
        }
    }
    #[must_use]
    fn to_bytes_endian(&self, endian: Option<Endian>) -> Self::AsBytesType {
        match endian {
            Some(Endian::Big) => self.to_bytes_be(),
            Some(Endian::Little) => self.to_bytes_le(),
            None => self.to_bytes_ne(),
        }
    }
    #[must_use]
    fn from_bytes_endian(bytes: &[u8], endian: Option<Endian>) -> Option<Self> {
        match endian {
            Some(Endian::Big) => Self::from_bytes_be(bytes),
            Some(Endian::Little) => Self::from_bytes_le(bytes),
            None => Self::from_bytes_ne(bytes),
        }
    }
}
/// Implement ToFromEndian for all primitive types (see beneath)
macro_rules! implement_to_from_bytes {
    ( $( $t:ty ), *) => {
        $(
            impl ToFromBytesEndian for $t {
    type AsBytesType = [u8; core::mem::size_of::<Self>()];

    #[inline]
    #[must_use]
    fn byte_size() -> usize {
        core::mem::size_of::<Self>()
    }

    #[inline]
    #[must_use]
    fn to_bytes_le(&self) -> Self::AsBytesType {
        self.to_le_bytes()
    }

    #[inline]
    #[must_use]
    fn to_bytes_be(&self) -> Self::AsBytesType {
        self.to_be_bytes()
    }

    #[inline]
    #[must_use]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        self.to_ne_bytes()
    }

    #[inline]
    #[must_use]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_le_bytes(bytes.try_into().ok()?))
    }

    #[inline]
    #[must_use]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_be_bytes(bytes.try_into().ok()?))
    }

    #[inline]
    #[must_use]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_ne_bytes(bytes.try_into().ok()?))
    }
}
        )*
    }
}
implement_to_from_bytes!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl ToFromBytesEndian for bool {
    type AsBytesType = [u8; 1];

    #[inline]
    fn to_bytes_le(&self) -> Self::AsBytesType {
        self.to_bytes_ne()
    }

    #[inline]
    fn to_bytes_be(&self) -> Self::AsBytesType {
        self.to_bytes_ne()
    }

    #[inline]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        [u8::from(*self)]
    }

    #[inline]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_ne(bytes)
    }
    #[inline]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_ne(bytes)
    }
    /// # Example
    /// ```
    /// use btle::bytes::ToFromBytesEndian;
    /// assert_eq!(bool::from_bytes_ne(&[0]), Some(false));
    /// assert_eq!(bool::from_bytes_ne(&[1]), Some(true));
    /// assert_eq!(bool::from_bytes_ne(&[2]), None);
    /// ```
    #[inline]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 1 {
            match bytes[0] {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        } else {
            None
        }
    }
}
