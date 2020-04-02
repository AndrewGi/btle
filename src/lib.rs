//! Generic BLE driver targeting mostly Bluetooth Advertisements. Implements the HCI layer.

// For development, allow dead_code
#![warn(clippy::pedantic)]
// Clippy complains about the mass enum matching functions
#![allow(clippy::too_many_lines)]
// #[must_use] doesn't need to be on absolutely everything even though it should.
#![allow(clippy::must_use_candidate)]
#![allow(
    clippy::missing_errors_doc,
    clippy::range_plus_one,
    clippy::type_complexity,
    clippy::doc_markdown
)]
#![deny(unconditional_recursion)]
#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg_attr(not(feature = "std"), macro_use)]
extern crate alloc;
/// Workaround for returning futures from async Traits.
pub type BoxFuture<'a, T> = futures_core::future::BoxFuture<'a, T>;
/// Workaround for returning streams from async Traits.
pub type BoxStream<'a, T> = futures_core::stream::BoxStream<'a, T>;
#[cfg(feature = "std")]
#[macro_use]
extern crate std;

use core::convert::{TryFrom, TryInto};

pub mod asyncs;
pub mod bytes;
pub mod error;
pub mod hci;
pub mod le;
pub mod uri;
#[cfg(feature = "windows_drivers")]
pub mod windows;
/// Basic `ConversionError` for when primitives can't be converted to/from bytes because of invalid
/// states. Most modules use their own errors for when there is more information to report.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ConversionError(());
/// Byte Packing/Unpacking error. Usually used for packing/unpacking a struct/type into/from
/// a byte buffer.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PackError {
    BadOpcode,
    BadLength { expected: usize, got: usize },
    BadBytes { index: Option<usize> },
    InvalidFields,
}
impl PackError {
    /// Ensure `buf.len() == expected`. Returns `Ok(())` if they are equal or
    /// `Err(HCIPackError::BadLength)` not equal.
    #[inline]
    pub fn expect_length(expected: usize, buf: &[u8]) -> Result<(), PackError> {
        if buf.len() == expected {
            Ok(())
        } else {
            Err(PackError::BadLength {
                expected,
                got: buf.len(),
            })
        }
    }
    /// Returns `PackError::BadBytes { index: Some(index) }`.
    #[inline]
    pub fn bad_index(index: usize) -> PackError {
        PackError::BadBytes { index: Some(index) }
    }
}
impl error::Error for PackError {}

/// Received Signal Strength Indicator (RSSI). Units: `dBm`. Range -127 dBm to +20 dBm. Defaults to
/// 0 dBm.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RSSI(i8);
impl RSSI {
    pub const MIN_RSSI_I8: i8 = -127;
    pub const MAX_RSSI_I8: i8 = 20;
    pub const MAX_RSSI: RSSI = RSSI(Self::MAX_RSSI_I8);
    pub const MIN_RSSI: RSSI = RSSI(Self::MIN_RSSI_I8);
    /// Creates a new RSSI from `dbm`.
    /// # Panics
    /// Panics if `dbm < MIN_RSSI || dbm > MAX_RSSI`.
    pub fn new(dbm: i8) -> RSSI {
        assert!(
            dbm >= Self::MIN_RSSI_I8 && dbm <= Self::MAX_RSSI_I8,
            "invalid rssi '{}'",
            dbm
        );
        RSSI(dbm)
    }
    pub const UNSUPPORTED_RSSI: i8 = 127;
    pub fn maybe_rssi(val: i8) -> Result<Option<RSSI>, ConversionError> {
        match val {
            -127..=20 => Ok(Some(RSSI(val))),
            127 => Ok(None),
            _ => Err(ConversionError(())),
        }
    }
}
impl From<RSSI> for i8 {
    fn from(rssi: RSSI) -> Self {
        rssi.0
    }
}

impl From<RSSI> for u8 {
    fn from(rssi: RSSI) -> Self {
        rssi.0 as u8
    }
}
impl TryFrom<i8> for RSSI {
    type Error = ConversionError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value > Self::MAX_RSSI_I8 || value < Self::MIN_RSSI_I8 {
            Err(ConversionError(()))
        } else {
            Ok(RSSI(value))
        }
    }
}
impl TryFrom<u8> for RSSI {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        (value as i8).try_into()
    }
}
/// Stores milli-dBm.
/// So -100 dBm is = `RSSI(-100_000)`
/// 0 dBm = `RSSI(0)`
/// 10.05 dBm = `RSSI(10_050)`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MilliDBM(pub i32);
impl MilliDBM {
    pub fn new(milli_dbm: i32) -> MilliDBM {
        MilliDBM(milli_dbm)
    }
}
/// Bluetooth address length (6 bytes)
pub const BT_ADDRESS_LEN: usize = 6;

/// Bluetooth Address. 6 bytes long.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BTAddress(pub [u8; BT_ADDRESS_LEN]);
impl BTAddress {
    pub const LEN: usize = BT_ADDRESS_LEN;
    pub const ZEROED: BTAddress = BTAddress([0_u8; 6]);
    /// Creates a new 'BTAddress' from a byte slice.
    /// # Panics
    /// Panics if `bytes.len() != BT_ADDRESS_LEN` (6 bytes).
    pub fn new(bytes: &[u8]) -> BTAddress {
        assert_eq!(bytes.len(), BT_ADDRESS_LEN, "address wrong length");
        BTAddress(bytes.try_into().expect("length checked by assert_eq above"))
    }
    pub fn unpack_from(bytes: &[u8]) -> Result<Self, PackError> {
        PackError::expect_length(BT_ADDRESS_LEN, bytes)?;
        Ok(Self::new(bytes))
    }
    pub fn pack_into(self, bytes: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(BT_ADDRESS_LEN, bytes)?;
        bytes.copy_from_slice(&self.0[..]);
        Ok(())
    }
}
