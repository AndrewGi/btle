use crate::advertisement::MAX_ADV_LEN;
use crate::bytes::{StaticBuf, Storage};
use crate::hci::le::{MetaEvent, MetaEventCode};
use crate::{BTAddress, ConversionError, PackError, BT_ADDRESS_LEN, RSSI};
use std::convert::TryFrom;

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
pub struct ReportInfo<T: AsRef<[u8]>> {
    pub event_type: EventType,
    pub address_type: AddressType,
    pub address: BTAddress,
    pub data: T,
    pub rssi: Option<RSSI>,
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
            data: T::default(),
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
pub type StaticAdvBuffer = StaticBuf<u8, [u8; MAX_ADV_LEN]>;
pub struct AdvertisingReport<T: AsRef<[ReportInfo<B>]>, B: AsRef<[u8]> = StaticAdvBuffer> {
    pub reports: T,
    pub _marker: core::marker::PhantomData<B>,
}
impl<T: AsRef<[ReportInfo<B>]>, B: AsRef<[u8]>> AdvertisingReport<T, B> {
    pub const SUBEVENT_CODE: MetaEventCode = MetaEventCode::AdvertisingReport;
    pub fn new(reports: T) -> Self {
        Self {
            reports,
            _marker: core::marker::PhantomData,
        }
    }
    pub fn byte_len(&self) -> usize {
        self.reports
            .as_ref()
            .iter()
            .fold(0usize, |size, report| size + report.byte_len())
            + 1
    }
}

impl<T: Storage<ReportInfo<B>>, B: Storage<u8> + Default + Copy> MetaEvent
    for AdvertisingReport<T, B>
{
    const META_CODE: MetaEventCode = Self::SUBEVENT_CODE;

    fn byte_len(&self) -> usize {
        AdvertisingReport::byte_len(self)
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        let num_reports = NumReports::try_from(*buf.get(0).ok_or(PackError::BadLength {
            expected: 1,
            got: 0,
        })?)
        .map_err(|_| PackError::bad_index(0))?;
        let mut out = AdvertisingReport::new(T::with_size(num_reports.0.into()));
        let reports_len = usize::from(num_reports.0);
        let mut total_data_len = 0usize;
        for i in 0..reports_len {
            let event_type_index = i + 1;
            let address_type_index = event_type_index + reports_len;
            let address_index = address_type_index + reports_len;
            let data_len_index = address_index + BT_ADDRESS_LEN * reports_len;
            let data_index = data_len_index + total_data_len;
            let event_type = match buf.get(event_type_index).map(|e| EventType::try_from(*e)) {
                Some(Ok(t)) => t,
                _ => return Err(PackError::bad_index(event_type_index)),
            };
            let address_type = match buf
                .get(address_type_index)
                .map(|e| AddressType::try_from(*e))
            {
                Some(Ok(t)) => t,
                _ => return Err(PackError::bad_index(address_type_index)),
            };

            let address =
                BTAddress::unpack_from(&buf[address_index..address_index + BT_ADDRESS_LEN])?;
            let data_len = buf
                .get(data_len_index)
                .map(|e| *e)
                .ok_or(PackError::bad_index(data_len_index))?;
            let data_index_end = data_index + usize::from(data_len);
            if usize::from(data_len) > MAX_ADV_LEN {
                return Err(PackError::bad_index(data_len_index));
            }
            PackError::expect_length(data_index_end, buf)?;
            let data = &buf[data_index..data_index_end];
            out.reports.as_mut()[i] = ReportInfo {
                event_type,
                address_type,
                address,
                data: B::from_slice(data),
                rssi: None,
            };
            total_data_len += usize::from(data_len);
        }
        for i in 0..reports_len {
            let rssi_index = total_data_len + i;
            out.reports.as_mut()[i].rssi =
                match buf.get(rssi_index).map(|val| RSSI::maybe_rssi(*val as i8)) {
                    Some(Ok(maybe_rssi)) => maybe_rssi,
                    _ => return Err(PackError::bad_index(rssi_index)),
                }
        }
        Ok(out)
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        let reports = self.reports.as_ref();
        let reports_len = reports.len();
        let num_reports =
            NumReports::try_from(reports_len).map_err(|_| PackError::InvalidFields)?;
        let full = self.byte_len();
        PackError::expect_length(full, buf)?;
        let mut total_data_len = 0usize;
        for i in 0..reports_len {
            let report = &reports[i];
            let data = report.data.as_ref();
            let data_len = data.len();
            if data_len > MAX_ADV_LEN {
                return Err(PackError::InvalidFields);
            }
            let event_type_index = i + 1;
            let address_type_index = event_type_index + reports_len;
            let address_index = address_type_index + reports_len;
            let data_len_index = address_index + BT_ADDRESS_LEN * reports_len;
            let data_index = data_len_index + total_data_len;
            let data_index_end = data_index + usize::from(data_len);
            buf[event_type_index] = report.event_type.into();
            buf[address_type_index] = report.address_type.into();
            report
                .address
                .pack_into(&mut buf[address_type_index..address_type_index + BT_ADDRESS_LEN])?;
            buf[data_index..data_index_end].copy_from_slice(data);
            total_data_len += data_len;
        }
        for i in 0..reports_len {
            let rssi_index = total_data_len + i;
            buf[rssi_index] = reports[i]
                .rssi
                .map(i8::from)
                .unwrap_or(RSSI::UNSUPPORTED_RSSI) as u8;
        }
        buf[0] = num_reports.into();
        Ok(())
    }
}
