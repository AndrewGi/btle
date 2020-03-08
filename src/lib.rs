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
#![allow(dead_code)]
//#![no_std]
extern crate alloc;
pub type BoxFuture<'a, T> = futures_core::future::BoxFuture<'a, T>;
#[cfg(feature = "std")]
#[macro_use]
extern crate std;

pub mod adapter;
pub mod advertisement;
pub mod advertiser;
pub mod bytes;
pub mod error;
#[cfg(feature = "hci")]
pub mod hci;
pub mod manager;
pub mod uri;
/// Stores Received Signal Strength Indicated as milli-dBm.
/// So -100 dBm is = `RSSI(-100_000)`
/// 0 dBm = `RSSI(0)`
/// 10.05 dBm = `RSSI(10_050)`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RSSI(pub i32);
impl RSSI {
    pub fn new(milli_dbm: i32) -> RSSI {
        RSSI(milli_dbm)
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BTAddress();
