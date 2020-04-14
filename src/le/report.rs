use crate::le::advertisement::{RawAdvertisement, StaticAdvBuffer};
use crate::{BTAddress, BT_ADDRESS_LEN, RSSI};
use core::convert::TryFrom;
use core::fmt::Formatter;
use driver_async::ConversionError;

pub struct NumReports(u8);
impl NumReports {
    pub const NUM_REPORTS_MIN: u8 = 0x01;
    pub const NUM_REPORTS_MAX: u8 = 0x19;
    pub fn new(num_reports: u8) -> NumReports {
        if let Ok(r) = NumReports::try_from(num_reports) {
            r
        } else {
            panic!("invalid number of reports: {}", num_reports);
        }
    }
}
impl From<NumReports> for u8 {
    fn from(n: NumReports) -> Self {
        n.0
    }
}
impl TryFrom<u8> for NumReports {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= NumReports::NUM_REPORTS_MAX && value >= NumReports::NUM_REPORTS_MIN {
            Ok(NumReports(value))
        } else {
            Err(ConversionError(()))
        }
    }
}
impl TryFrom<usize> for NumReports {
    type Error = ConversionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u8::try_from(value).map(NumReports::try_from) {
            Ok(Ok(v)) => Ok(v),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum EventType {
    AdvInd = 0x00,
    AdvDirectInd = 0x01,
    AdvScanInd = 0x02,
    AdvNonconnInd = 0x03,
    ScanRsp = 0x04,
}
impl EventType {
    pub fn as_str(self) -> &'static str {
        match self {
            EventType::AdvInd => "ADV_IND",
            EventType::AdvDirectInd => "ADV_DIRECT_IND",
            EventType::AdvScanInd => "ADV_SCAN_IND",
            EventType::AdvNonconnInd => "ADV_NONNCONN_IND",
            EventType::ScanRsp => "SCAN_RSP",
        }
    }
}
impl core::fmt::Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl TryFrom<u8> for EventType {
    type Error = ConversionError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(EventType::AdvInd),
            0x01 => Ok(EventType::AdvDirectInd),
            0x02 => Ok(EventType::AdvScanInd),
            0x03 => Ok(EventType::AdvNonconnInd),
            0x04 => Ok(EventType::ScanRsp),
            _ => Err(ConversionError(())),
        }
    }
}
impl From<EventType> for u8 {
    fn from(e: EventType) -> Self {
        e as u8
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum AddressType {
    PublicDevice = 0x00,
    RandomDevice = 0x01,
    PublicIdentity = 0x02,
    RandomIdentity = 0x03,
}
impl TryFrom<u8> for AddressType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(AddressType::PublicDevice),
            0x01 => Ok(AddressType::RandomDevice),
            0x02 => Ok(AddressType::PublicIdentity),
            0x03 => Ok(AddressType::RandomIdentity),
            _ => Err(ConversionError(())),
        }
    }
}
impl From<AddressType> for u8 {
    fn from(a: AddressType) -> Self {
        a as u8
    }
}
/// BLE Advertising report from scanning for advertisements that contains advertisement type [`EventType`],
/// address type [`AddressType`], bluetooth address [`BTAddress`], data (0-31 bytes) and
/// maybe (`Option`) RSSI [`RSSI`].
///
/// # Important
/// `T` is the byte buffer that stores the advertisement data (0-31 bytes) which means `T` should
/// always be able to hold 31 bytes if you are using `unpack_from`.
pub struct ReportInfo<T = StaticAdvBuffer> {
    /// Advertisement Type.
    pub event_type: EventType,
    /// Bluetooth Address type.
    pub address_type: AddressType,
    /// Bluetooth Address associated with the Advertisement.
    pub address: BTAddress,
    /// Advertisement data (0-31 bytes).
    pub data: RawAdvertisement<T>,
    /// RSSI (-127dBm to +20dBm) or `None` if RSSI readings are unsupported by the adapter.
    pub rssi: Option<RSSI>,
}
impl<T: AsRef<[u8]>> core::fmt::Debug for ReportInfo<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ReportInfo")
            .field("event_type", &self.event_type)
            .field("address_type", &self.address_type)
            .field("address", &self.address)
            .field("rssi", &self.rssi)
            .field("data", &self.data.as_ref())
            .finish()
    }
}
impl<T: AsRef<[u8]> + Copy> Copy for ReportInfo<T> {}
impl<T: AsRef<[u8]> + Clone> Clone for ReportInfo<T> {
    fn clone(&self) -> Self {
        Self {
            event_type: self.event_type,
            address_type: self.address_type,
            address: self.address,
            data: self.data.clone(),
            rssi: self.rssi,
        }
    }
}
impl<T: AsRef<[u8]> + Default> Default for ReportInfo<T> {
    fn default() -> Self {
        Self {
            event_type: EventType::AdvInd,
            address_type: AddressType::PublicDevice,
            address: BTAddress::ZEROED,
            data: RawAdvertisement(T::default()),
            rssi: None,
        }
    }
}
impl<T: AsRef<[u8]>> ReportInfo<T> {
    pub fn byte_len(&self) -> usize {
        // event_type (1) + address_type (1) + address (6) + data (data.len()) + rssi (1)
        1 + 1 + BT_ADDRESS_LEN + self.data.as_ref().len() + 1
    }
}
