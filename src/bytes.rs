//! Byte buffer, packing and unpacking utilities. Provides traits for genericly packing types into
//! different endian byte buffers ([`ToFromBytesEndian`]) and for storing
//! bytes/copy-types ([`Storage`])
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::ops;
/// Byte Endian
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
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
/// Trait for types that can be packed/unpack into/from bytes in either endian.
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

/// Static byte buffer. `StaticBuf<[u8; 16]>` can store a `[u8]` array from 0-16 bytes for example.
/// Unlike other static buffers, this does NOT reallocate if you out grow the internal buffer. If
/// you try to request more bytes than its able to store, it will panic.  
#[derive(Copy, Clone, Default)]
pub struct StaticBuf<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> {
    buf: ArrayBuf,
    len: usize,
    _marker: core::marker::PhantomData<T>,
}
impl<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> StaticBuf<T, ArrayBuf> {
    /// Returns the maximum size the `StaticBuf` can hold.
    /// # Examples
    /// ```
    /// use btle::bytes::StaticBuf;
    /// assert_eq!(StaticBuf::<u8, [u8; 10]>::max_size(), 10);
    /// assert_eq!(StaticBuf::<u8, [u8; 23]>::max_size(), 23);
    /// ```
    pub fn max_size() -> usize {
        ArrayBuf::default().as_ref().len()
    }
    /// Returns the space left in `T`s (not bytes) in the `StaticBuf`.
    /// Simply (`capacity - length`).
    pub fn space_left(&self) -> usize {
        self.buf.as_ref().len() - self.len
    }
    /// Resizes the `StaticBuf` by settings `self.len` to `new_size` if `new_size <= Self::max_size()`.
    /// This is only a single variable change and WILL NOT zero or change any of the buffers bytes.
    /// # Panics
    /// Panics if the new size is greater than max_size (`new_size > Self::max_size()`).
    /// # Examples
    /// ```
    /// use btle::bytes::{StaticBuf, Storage};
    /// let mut buf = StaticBuf::<u8, [u8; 10]>::with_size(10);
    /// assert_eq!(buf.len(), 10);
    /// assert_eq!(buf[9], 0);
    /// buf[9] = 0xFF;
    /// buf.resize(1);
    /// assert_eq!(buf.len(), 1);
    /// buf.resize(10);
    /// assert_eq!(buf[9], 0xFF);
    /// ```
    pub fn resize(&mut self, new_size: usize) {
        assert!(
            new_size <= Self::max_size(),
            "requested size {} bigger than static buf size {}",
            new_size,
            Self::max_size()
        );
        self.len = new_size;
    }
    /// Appends the slice onto the end of the `StaticBuf`.
    /// # Panics
    /// Panics if appending the slice would overflow the `StaticBuf` (not enough space).
    pub fn append_slice(&mut self, slice: &[T]) {
        let cur_len = self.len;
        self.resize(cur_len + slice.len());
        self.as_mut()[cur_len..].copy_from_slice(slice);
    }
}
impl<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> AsRef<[T]>
    for StaticBuf<T, ArrayBuf>
{
    fn as_ref(&self) -> &[T] {
        &self.buf.as_ref()[..self.len]
    }
}
impl<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> AsMut<[T]>
    for StaticBuf<T, ArrayBuf>
{
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.buf.as_mut()[..self.len]
    }
}
impl<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> ops::Index<ops::RangeFull>
    for StaticBuf<T, ArrayBuf>
{
    type Output = [T];

    fn index(&self, _index: ops::RangeFull) -> &Self::Output {
        self.as_ref()
    }
}
impl<T: Copy, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> ops::IndexMut<ops::RangeFull>
    for StaticBuf<T, ArrayBuf>
{
    fn index_mut(&mut self, _index: ops::RangeFull) -> &mut Self::Output {
        self.as_mut()
    }
}
impl<T: Copy + Default, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> ops::Index<usize>
    for StaticBuf<T, ArrayBuf>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl<T: Copy + Default, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy> ops::IndexMut<usize>
    for StaticBuf<T, ArrayBuf>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}
/// Objects that store and own `T`s (`Box<[T]>`, `Vec<T>`, `StaticBuf<[T; 32]>`, etc).
/// This allows for generic byte storage types for byte buffers. This also enable generic storage
/// for any `T` type but the `Copy + Default` requirement might be too restricting for all cases.
pub trait Storage<T: Copy + Default>: AsRef<[T]> + AsMut<[T]> + Unpin {
    fn with_size(size: usize) -> Self
    where
        Self: Sized;
    fn from_slice(buf: &[T]) -> Self
    where
        Self: Sized,
    {
        let mut out = Self::with_size(buf.len());
        out.as_mut().copy_from_slice(buf);
        out
    }
    fn max_len() -> usize;
    fn space_left(&self) -> usize {
        Self::max_len() - self.len()
    }
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}
impl<T: Copy + Unpin + Default> Storage<T> for Vec<T> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        vec![T::default(); size]
    }
    fn from_slice(buf: &[T]) -> Self
    where
        Self: Sized,
    {
        Vec::from(buf)
    }
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    fn max_len() -> usize {
        usize::max_value()
    }
}
impl<T: Copy + Unpin + Default> Storage<T> for Box<[T]> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        Vec::with_size(size).into_boxed_slice()
    }
    fn from_slice(buf: &[T]) -> Self
    where
        Self: Sized,
    {
        buf.into()
    }

    fn max_len() -> usize {
        usize::max_value()
    }
}

impl<T: Copy + Unpin + Default, ArrayBuf: AsRef<[T]> + AsMut<[T]> + Default + Copy + Unpin>
    Storage<T> for StaticBuf<T, ArrayBuf>
{
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        assert!(
            size <= Self::max_size(),
            "requested size {} bigger than static buf size {}",
            size,
            Self::max_size()
        );
        Self {
            buf: ArrayBuf::default(),
            len: size,
            _marker: core::marker::PhantomData,
        }
    }
    fn max_len() -> usize {
        Self::max_size()
    }
    fn len(&self) -> usize {
        self.len
    }
}
