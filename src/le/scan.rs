use crate::ConversionError;

use crate::hci::adapter;
use crate::le::advertisement::StaticAdvBuffer;
use crate::le::report::ReportInfo;
use core::convert::TryFrom;
use futures_util::future::LocalBoxFuture;
use futures_util::stream::LocalBoxStream;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ScanningFilterPolicy {
    All = 0x00,
    Whitelisted = 0x01,
    DirectedAll = 0x02,
    DirectedWhitelisted = 0x03,
}
impl From<ScanningFilterPolicy> for u8 {
    fn from(p: ScanningFilterPolicy) -> Self {
        p as u8
    }
}
impl TryFrom<u8> for ScanningFilterPolicy {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanningFilterPolicy::All),
            1 => Ok(ScanningFilterPolicy::Whitelisted),
            2 => Ok(ScanningFilterPolicy::DirectedAll),
            3 => Ok(ScanningFilterPolicy::DirectedWhitelisted),
            _ => Err(ConversionError(())),
        }
    }
}
const INTERVAL_MIN: u16 = 0x0004;
const INTERVAL_MAX: u16 = 0x4000;
/// Range 0x0004 --> 0x4000
/// Default 0x0010 (10 ms)
/// Time = N *  0.625 ms
/// Time Range 2.5 ms --> 10.24 s
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ScanInterval(u16);
impl ScanInterval {
    pub const DEFAULT: ScanInterval = ScanInterval(0x0010);
    pub fn new(interval: u16) -> ScanInterval {
        assert!(
            interval >= INTERVAL_MIN && interval <= INTERVAL_MAX,
            "interval '{}' is out of range"
        );
        ScanInterval(interval)
    }
    pub fn as_microseconds(self) -> u32 {
        u32::from(u16::from(self)) * 625
    }
}
impl From<ScanInterval> for u16 {
    fn from(i: ScanInterval) -> u16 {
        i.0
    }
}
impl Default for ScanInterval {
    fn default() -> Self {
        Self::DEFAULT
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ScanWindow(u16);
impl ScanWindow {
    pub const DEFAULT: ScanWindow = ScanWindow(0x0010);
    pub fn new(window: u16) -> ScanWindow {
        assert!(
            window >= INTERVAL_MIN && window <= INTERVAL_MAX,
            "window '{}' is out of range"
        );
        ScanWindow(window)
    }
    pub fn as_microseconds(self) -> u32 {
        u32::from(u16::from(self)) * 625
    }
}
impl From<ScanWindow> for u16 {
    fn from(w: ScanWindow) -> u16 {
        w.0
    }
}
impl Default for ScanWindow {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum OwnAddressType {
    Public = 0x00,
    Random = 0x01,
    PrivateOrPublic = 0x02,
    PrivateOrRandom = 0x03,
}

impl From<OwnAddressType> for u8 {
    fn from(s: OwnAddressType) -> Self {
        s as u8
    }
}
impl TryFrom<u8> for OwnAddressType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OwnAddressType::Public),
            1 => Ok(OwnAddressType::Random),
            2 => Ok(OwnAddressType::PrivateOrPublic),
            3 => Ok(OwnAddressType::PrivateOrRandom),
            _ => Err(ConversionError(())),
        }
    }
}
/// Advertising scan type (Active or Passive).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ScanType {
    Passive = 0x00,
    Active = 0x01,
}
impl From<ScanType> for u8 {
    fn from(s: ScanType) -> Self {
        s as u8
    }
}
impl TryFrom<u8> for ScanType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanType::Passive),
            1 => Ok(ScanType::Active),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ScanParameters {
    pub scan_type: ScanType,
    pub scan_interval: ScanInterval,
    pub scan_window: ScanWindow,
    pub own_address_type: OwnAddressType,
    pub scanning_filter_policy: ScanningFilterPolicy,
}
impl ScanParameters {
    pub const DEFAULT: ScanParameters = ScanParameters {
        scan_type: ScanType::Passive,
        scan_interval: ScanInterval::DEFAULT,
        scan_window: ScanWindow::DEFAULT,
        own_address_type: OwnAddressType::Public,
        scanning_filter_policy: ScanningFilterPolicy::All,
    };
}
impl Default for ScanParameters {
    fn default() -> Self {
        Self::DEFAULT
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ObserverError {
    AdapterError(adapter::Error),
}
impl crate::error::Error for ObserverError {}
pub trait Observer {
    fn set_scan_parameters<'a>(
        &'a mut self,
        scan_parameters: ScanParameters,
    ) -> LocalBoxFuture<'a, Result<(), adapter::Error>>;
    fn set_scan_enable<'a>(
        &'a mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> LocalBoxFuture<'a, Result<(), adapter::Error>>;
    fn advertisement_stream<'a>(
        &'a mut self,
    ) -> LocalBoxFuture<
        'a,
        Result<
            LocalBoxStream<'a, Result<ReportInfo<StaticAdvBuffer>, adapter::Error>>,
            adapter::Error,
        >,
    >;
    /// Optimize `advertisement_stream` by avoiding setting HCI event masks. Defaults to calling
    /// `advertisement_stream()`.
    /// # Caution
    /// This function does not guarantee that the event masks won't be changed! By default, this
    /// function doesn't do anything different than `advertisement_stream()`.
    fn advertisement_stream_without_mask<'a>(
        &'a mut self,
    ) -> LocalBoxFuture<
        'a,
        Result<
            LocalBoxStream<'a, Result<ReportInfo<StaticAdvBuffer>, adapter::Error>>,
            adapter::Error,
        >,
    > {
        self.advertisement_stream()
    }
}
