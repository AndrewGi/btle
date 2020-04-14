use crate::le::adapter::Error;
use crate::le::advertisement::StaticAdvBuffer;
use crate::le::advertiser::{Advertiser, AdvertisingParameters};
use crate::{
    hci::{
        adapters,
        event::{Event, EventPacket, StaticEventBuffer},
        le,
        le::random::RAND_LEN,
        packet::RawPacket,
        stream::HCIStreamable,
        StreamError,
    },
    le::{
        adapter,
        advertisement::MAX_ADV_LEN,
        report::ReportInfo,
        scan::{Observer, ScanParameters},
    },
    BoxFuture, BoxStream,
};
use core::convert::TryFrom;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use driver_async::bytes::Storage;

pub struct LEAdapter<'a, S: HCIStreamable> {
    adapter: Pin<&'a mut adapters::Adapter<S>>,
}
impl<'a, S: HCIStreamable> LEAdapter<'a, S> {
    pub fn new(adapter: Pin<&'a mut adapters::Adapter<S>>) -> Self {
        Self { adapter }
    }
    pub fn adapter_mut(&mut self) -> Pin<&mut adapters::Adapter<S>> {
        self.adapter.as_mut()
    }
    pub fn adapter_ref(&self) -> Pin<&adapters::Adapter<S>> {
        self.adapter.as_ref()
    }
    /// Read the advertising channel TX power in dBm. See [`le::advertise::TxPowerLevel`] for more.
    pub async fn get_advertising_tx_power(
        &mut self,
    ) -> Result<le::advertise::TxPowerLevel, adapter::Error> {
        let r = self
            .adapter_mut()
            .send_command(le::commands::ReadAdvertisingChannelTxPower {})
            .await?;
        r.params.status.error()?;
        Ok(r.params.power_level)
    }
    /// Set advertisement scanning enable/disable. [`LEAdapter::set_scan_parameters`] should be
    /// called first to set any scanning parameters (how long, what type of advertisements, etc).
    pub async fn set_scan_enable(
        &mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> Result<(), adapter::Error> {
        self.adapter_mut()
            .send_command(le::commands::SetScanEnable {
                is_enabled,
                filter_duplicates,
            })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Set advertisement scanning parameters. See [`le::commands::SetScanParameters`] for more.
    pub async fn set_scan_parameters(
        &mut self,
        scan_parameters: ScanParameters,
    ) -> Result<(), adapter::Error> {
        self.adapter_mut()
            .send_command(le::scan::SetScanParameters(scan_parameters))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Enable or disable advertising. Make sure to set advertising parameters
    /// ([`LEAdapter::set_advertising_parameters`]) and advertising data
    /// ([`LEAdapter::set_advertising_data`]) before calling this function.
    pub async fn set_advertising_enable(&mut self, is_enabled: bool) -> Result<(), adapter::Error> {
        self.adapter_mut()
            .send_command(le::commands::SetAdvertisingEnable { is_enabled })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Set advertising parameters. See [`le::commands::SetAdvertisingParameters`] for more.
    pub async fn set_advertising_parameters(
        &mut self,
        parameters: AdvertisingParameters,
    ) -> Result<(), adapter::Error> {
        self.adapter_mut()
            .send_command(le::commands::SetAdvertisingParameters(parameters))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Get `RAND_LEN` (8) bytes from the HCI Controller.
    pub async fn get_rand(&mut self) -> Result<[u8; RAND_LEN], adapter::Error> {
        let r = self
            .adapter_mut()
            .send_command(le::commands::Rand {})
            .await?;
        r.params.status.error()?;
        Ok(r.params.random_bytes)
    }
    /// Set advertising data (0-31 bytes).
    /// # Errors
    /// Returns `adapter::Error::BadParameter` if `data.len() > MAX_ADV_LEN` (31).
    pub async fn set_advertising_data(&mut self, data: &[u8]) -> Result<(), adapter::Error> {
        if data.len() > MAX_ADV_LEN {
            return Err(adapter::Error::BadParameter);
        }
        self.adapter_mut()
            .send_command(le::commands::SetAdvertisingData::new(data))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Create an BLE Advertisement Stream that returns
    /// `le::report::ReportInfo<le::report::StaticAdvBuffer>>`. Make sure you set scan parameters
    /// and a `Filter` before calling this.
    pub fn advertisement_stream<'b, Buf: Storage<ReportInfo<StaticAdvBuffer>>>(
        &'b mut self,
    ) -> AdvertisementStream<'b, 'a, S, Buf> {
        AdvertisementStream::new(self)
    }
}
/// BLE Advertisement Stream. Returns advertising reports [`ReportInfo'] that contain
/// advertisement type [`EventType`], address type [`AddressType`], bluetooth address [`BTAddress`],
/// data (0-31 bytes) and maybe (`Option`) RSSI [`RSSI`].
///
/// [`ReportInfo`]: btle::hci::le::report::ReportInfo;
/// [`EventType`]: btle::hci::le::report::EventType;
/// [`AddressType`]: btle::hci::le::report::AddressType;
/// [`BTAddress`]: btle::BTAddress;
/// [`RSSI`]: btle::RSSI;
pub struct AdvertisementStream<
    'a,
    'b: 'a,
    S: HCIStreamable,
    Buf: Storage<ReportInfo<ReportBuf>>,
    ReportBuf: Storage<u8> + Copy + Default = StaticAdvBuffer,
    PacketBuf: Storage<u8> = StaticEventBuffer,
> {
    adapter: &'a mut LEAdapter<'b, S>,
    last_report: Option<(le::events::AdvertisingReport<Buf, ReportBuf>, usize)>,
    marker_: core::marker::PhantomData<PacketBuf>,
}
impl<
        'a,
        'b: 'a,
        S: HCIStreamable,
        Buf: Storage<ReportInfo<ReportBuf>>,
        ReportBuf: Storage<u8> + Copy + Default,
        PacketBuf: Storage<u8>,
    > AdvertisementStream<'a, 'b, S, Buf, ReportBuf, PacketBuf>
{
    pub fn new(adapter: &'a mut LEAdapter<'b, S>) -> Self {
        Self {
            adapter,
            last_report: None,
            marker_: core::marker::PhantomData,
        }
    }
}
impl<
        'a,
        'b: 'a,
        S: HCIStreamable,
        Buf: Storage<ReportInfo<ReportBuf>>,
        ReportBuf: Storage<u8> + Copy + Default,
        PacketBuf: Storage<u8>,
    > driver_async::asyncs::stream::Stream
    for AdvertisementStream<'a, 'b, S, Buf, ReportBuf, PacketBuf>
{
    type Item = Result<ReportInfo<ReportBuf>, adapter::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        if let Some((ref reports, index)) = &mut this.last_report {
            if let Some(report) = reports.reports.as_ref().get(*index) {
                *index += 1;
                return Poll::Ready(Some(Ok(*report)));
            } else {
                this.last_report = None;
            }
        }
        let packet: RawPacket<PacketBuf> =
            match unsafe { Pin::new_unchecked(&mut this.adapter.adapter.as_mut().read_packet()) }
                .poll(cx)
            {
                Poll::Ready(r) => match r {
                    Ok(p) => p,
                    Err(e) => return Poll::Ready(Some(Err(e.into()))),
                },
                Poll::Pending => return Poll::Pending,
            };

        let reports = match EventPacket::try_from(packet.as_ref())
            .map(|p| le::events::AdvertisingReport::unpack_event_packet(&p))
        {
            Ok(Ok(reports)) => reports,
            Ok(Err(e)) | Err(e) => {
                return Poll::Ready(Some(Err(adapter::Error::StreamError(
                    StreamError::CommandError(e),
                ))))
            }
        };
        self.last_report = Some((reports, 0));
        self.poll_next(cx)
    }
}

impl<'a, S: HCIStreamable> Observer for LEAdapter<'a, S> {
    fn set_scan_parameters(
        &mut self,
        scan_parameters: ScanParameters,
    ) -> BoxFuture<Result<(), adapter::Error>> {
        Box::pin(LEAdapter::set_scan_parameters(self, scan_parameters))
    }

    fn set_scan_enable(
        &mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> BoxFuture<Result<(), adapter::Error>> {
        Box::pin(LEAdapter::set_scan_enable(
            self,
            is_enabled,
            filter_duplicates,
        ))
    }

    fn advertisement_stream(&mut self) -> BoxStream<Result<ReportInfo, adapter::Error>> {
        Box::pin(LEAdapter::advertisement_stream::<Box<[ReportInfo]>>(self))
    }
}

impl<'a, S: HCIStreamable> Advertiser for LEAdapter<'a, S> {
    fn set_advertising_enable(&mut self, is_enabled: bool) -> BoxFuture<Result<(), Error>> {
        Box::pin(LEAdapter::set_advertising_enable(self, is_enabled))
    }

    fn set_advertising_parameters(
        &mut self,
        advertisement_parameters: AdvertisingParameters,
    ) -> BoxFuture<Result<(), Error>> {
        Box::pin(LEAdapter::set_advertising_parameters(
            self,
            advertisement_parameters,
        ))
    }

    fn set_advertising_data<'s, 'b: 's>(
        &'b mut self,
        data: &'s [u8],
    ) -> BoxFuture<'s, Result<(), Error>> {
        Box::pin(LEAdapter::set_advertising_data(self, data))
    }
}
