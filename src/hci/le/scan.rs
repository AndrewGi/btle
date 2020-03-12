//! LE [`SetScanEnable`], [`SetScanParameters`], and other primitive scan types.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::StatusReturn;
use crate::hci::le::LEControllerOpcode;
use crate::hci::Opcode;
use crate::{ConversionError, PackError};
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetScanEnable {
    pub is_enabled: bool,
    pub filter_duplicates: bool,
}
impl SetScanEnable {
    pub const BYTE_LEN: usize = 2;
}
impl Command for SetScanEnable {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanEnable.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        buf[1] = self.filter_duplicates.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let is_enabled = match buf[0] {
            0 => false,
            1 => true,
            _ => return Err(PackError::bad_index(0)),
        };
        let filter_duplicates = match buf[1] {
            0 => false,
            1 => true,
            _ => return Err(PackError::bad_index(1)),
        };
        Ok(Self {
            is_enabled,
            filter_duplicates,
        })
    }
}

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
pub struct SetScanParameters {
    pub scan_type: ScanType,
    pub scan_interval: ScanInterval,
    pub scan_window: ScanWindow,
    pub own_address_type: OwnAddressType,
    pub scanning_filter_policy: ScanningFilterPolicy,
}
impl SetScanParameters {
    pub const DEFAULT: SetScanParameters = SetScanParameters {
        scan_type: ScanType::Passive,
        scan_interval: ScanInterval::DEFAULT,
        scan_window: ScanWindow::DEFAULT,
        own_address_type: OwnAddressType::Public,
        scanning_filter_policy: ScanningFilterPolicy::All,
    };
}
impl Default for SetScanParameters {
    fn default() -> Self {
        Self::DEFAULT
    }
}
pub const SET_SCAN_PARAMETERS_LEN: usize = 7;
impl Command for SetScanParameters {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanParameters.into()
    }

    fn byte_len(&self) -> usize {
        SET_SCAN_PARAMETERS_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(SET_SCAN_PARAMETERS_LEN, buf)?;
        if self.scan_window.0 > self.scan_interval.0 {
            // The scan window should always be less than or equal to the scan interval.
            return Err(PackError::InvalidFields);
        }
        buf[0] = self.scan_type.into();
        buf[1..3].copy_from_slice(&self.scan_interval.0.to_bytes_le()[..]);
        buf[3..5].copy_from_slice(&self.scan_window.0.to_bytes_le()[..]);
        buf[5] = self.own_address_type.into();
        buf[6] = self.scanning_filter_policy.into();
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
