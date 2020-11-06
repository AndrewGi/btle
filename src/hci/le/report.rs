//! LE [`AdvertisingReport`] and [`ReportInfo`] types.
use crate::bytes::Storage;
use crate::hci::le::{MetaEvent, MetaEventCode};
use crate::le::advertisement::{RawAdvertisement, StaticAdvBuffer, MAX_ADV_LEN};
use crate::le::report::{AddressType, EventType, NumReports, ReportInfo};
use crate::{BTAddress, PackError, BT_ADDRESS_LEN, RSSI};
use core::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub struct AdvertisingReport<T: AsRef<[ReportInfo<B>]>, B: AsRef<[u8]> = StaticAdvBuffer> {
    pub reports: T,
    _marker: core::marker::PhantomData<B>,
}
impl<T: AsRef<[ReportInfo<B>]>, B: AsRef<[u8]> + Clone> IntoIterator for AdvertisingReport<T, B> {
    type Item = ReportInfo<B>;
    type IntoIter = AdvertisingReportIter<T, B>;

    fn into_iter(self) -> Self::IntoIter {
        AdvertisingReportIter::new(self)
    }
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

    fn meta_byte_len(&self) -> usize {
        AdvertisingReport::byte_len(self)
    }

    fn meta_unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        let num_reports = NumReports::try_from(*buf.get(0).ok_or(PackError::BadLength {
            expected: 1,
            got: 0,
        })?)
        .map_err(|_| PackError::bad_index(0))?;
        let reports_len = usize::from(u8::from(num_reports));
        let mut out = AdvertisingReport::new(T::with_size(reports_len));
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
            let data_index_end = data_index + 1 + usize::from(data_len);
            if usize::from(data_len) > MAX_ADV_LEN {
                return Err(PackError::bad_index(data_len_index));
            }
            let data = &buf[data_index + 1..data_index_end];
            PackError::expect_length(data_len.into(), data)?;
            out.reports.as_mut()[i] = ReportInfo {
                event_type,
                address_type,
                address,
                data: RawAdvertisement(B::from_slice(data)),
                rssi: None,
            };
            total_data_len += usize::from(data_len);
        }
        for i in 0..reports_len {
            let rssi_index = 1 + (1 + 1 + 1 + BT_ADDRESS_LEN) * reports_len + total_data_len + i;
            out.reports.as_mut()[i].rssi =
                match buf.get(rssi_index).map(|val| RSSI::maybe_rssi(*val as i8)) {
                    Some(Ok(maybe_rssi)) => maybe_rssi,
                    _ => return Err(PackError::bad_index(rssi_index)),
                }
        }
        Ok(out)
    }

    fn meta_pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
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
            let rssi_index = 1 + (1 + 1 + 1 + BT_ADDRESS_LEN) * reports_len + total_data_len + i;
            buf[rssi_index] = reports[i]
                .rssi
                .map(i8::from)
                .unwrap_or(RSSI::UNSUPPORTED_RSSI) as u8;
        }
        buf[0] = num_reports.into();
        Ok(())
    }
}

pub struct AdvertisingReportIter<Buf: AsRef<[ReportInfo<ReportBuf>]>, ReportBuf: AsRef<[u8]>> {
    pub report: AdvertisingReport<Buf, ReportBuf>,
    pub index: usize,
    _marker: core::marker::PhantomData<ReportBuf>,
}
impl<Buf: AsRef<[ReportInfo<ReportBuf>]>, ReportBuf: AsRef<[u8]>>
    AdvertisingReportIter<Buf, ReportBuf>
{
    pub fn new(report: AdvertisingReport<Buf, ReportBuf>) -> Self {
        AdvertisingReportIter {
            report,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
}
impl<Buf: AsRef<[ReportInfo<ReportBuf>]>, ReportBuf: AsRef<[u8]> + Clone> Iterator
    for AdvertisingReportIter<Buf, ReportBuf>
{
    type Item = ReportInfo<ReportBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        let report = self.report.reports.as_ref().get(self.index)?;
        self.index += 1;
        Some(report.clone())
    }
}
