use crate::hci::adapters::Adapter;
use crate::hci::baseband::{EventMask, EventMaskFlags};
use crate::hci::le::mask::{MetaEventMask, SetMetaEventMask};
use crate::hci::le::MetaEventCode;
use crate::{
    bytes::Storage,
    hci::{
        adapter,
        event::{EventCode, EventPacket},
        le::{self, random::RAND_LEN, report::AdvertisingReport, MetaEvent, RawMetaEvent},
        StreamError,
    },
    le::{
        advertisement::{StaticAdvBuffer, MAX_ADV_LEN},
        advertiser::AdvertisingParameters,
        report::ReportInfo,
        scan::ScanParameters,
    },
    Stream,
};
use core::convert::TryFrom;
use core::ops::{Deref, DerefMut};
use futures_util::StreamExt;

pub struct LEAdapter<A: adapter::Adapter, S: Deref<Target = A> + DerefMut> {
    adapter: Adapter<A, S>,
}
impl<A: adapter::Adapter, S: Deref<Target = A> + DerefMut> LEAdapter<A, S> {
    pub fn new(adapter: Adapter<A, S>) -> Self {
        Self { adapter }
    }
    pub fn adapter_mut(&mut self) -> Adapter<A, &'_ mut A> {
        self.adapter.as_mut()
    }
    /// Read the advertising channel TX power in dBm. See [`le::advertise::TxPowerLevel`] for more.
    pub async fn get_advertising_tx_power(
        &mut self,
    ) -> Result<le::advertise::TxPowerLevel, adapter::Error> {
        let r = self
            .adapter
            .hci_send_command(le::commands::ReadAdvertisingChannelTxPower {})
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
        self.adapter
            .hci_send_command(le::commands::SetScanEnable {
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
        self.adapter
            .hci_send_command(le::scan::SetScanParameters(scan_parameters))
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
        self.adapter
            .hci_send_command(le::commands::SetAdvertisingEnable { is_enabled })
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
        self.adapter
            .hci_send_command(le::commands::SetAdvertisingParameters(parameters))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Get `RAND_LEN` (8) bytes from the HCI Controller.
    pub async fn get_rand(&mut self) -> Result<[u8; RAND_LEN], adapter::Error> {
        let r = self.adapter.hci_send_command(le::commands::Rand {}).await?;
        r.params.status.error()?;
        Ok(r.params.random_bytes)
    }
    pub async fn set_meta_event_mask(&mut self, mask: MetaEventMask) -> Result<(), adapter::Error> {
        self.adapter
            .hci_send_command(SetMetaEventMask(mask))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Set advertising data (0-31 bytes).
    /// # Errors
    /// Returns `adapter::Error::BadParameter` if `data.len() > MAX_ADV_LEN` (31).
    pub async fn set_advertising_data(&mut self, data: &[u8]) -> Result<(), adapter::Error> {
        if data.len() > MAX_ADV_LEN {
            return Err(adapter::Error::BadParameter);
        }
        self.adapter
            .hci_send_command(le::commands::SetAdvertisingData::new(data))
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn meta_event_stream<'a, 'b: 'a, Buf: Storage<u8> + 'b>(
        &'a mut self,
    ) -> Result<impl Stream<Item = Result<RawMetaEvent<Buf>, adapter::Error>> + 'a, adapter::Error>
    {
        let mut mask = EventMask::zeroed();
        mask.enable_event(EventMaskFlags::LEMetaEvent);
        self.adapter.set_event_mask(mask).await?;
        Ok(self.adapter.hci_event_stream().filter_map(
            |p: Result<EventPacket<Buf>, adapter::Error>| async move {
                let event = match p {
                    Ok(event) => event,
                    Err(e) => return Some(Err(e)),
                };
                // Ignore all non-LEMeta HCI Events
                if event.event_code == EventCode::LEMeta {
                    let meta_event = RawMetaEvent::try_from(event.as_ref())
                        .map_err(|e| adapter::Error::StreamError(StreamError::EventError(e)));
                    Some(meta_event.map(|e| e.to_owned()))
                } else {
                    None
                }
            },
        ))
    }
    pub async fn advertising_report_stream<
        'a,
        'b: 'a,
        Buf: Storage<ReportInfo<StaticAdvBuffer>> + 'b,
    >(
        &'a mut self,
    ) -> Result<
        impl Stream<Item = Result<AdvertisingReport<Buf>, adapter::Error>> + 'a,
        adapter::Error,
    > {
        let mut mask = MetaEventMask::zeroed();
        mask.enable_event(MetaEventCode::AdvertisingReport);
        self.set_meta_event_mask(mask).await?;
        Ok(self.meta_event_stream().await?.filter_map(
            |meta_event: Result<RawMetaEvent<Box<[u8]>>, adapter::Error>| async move {
                // We expect only AdvertisingReport Meta events to get through because the HCI
                // filter should be set for that. Otherwise if a non-`AdvertisingReport`
                // packet gets through, this will return `PackError::BadOpcode` because
                // an LEMeta event with an Event Code of anything but `AdvertisingReport` got
                // through.
                Some(meta_event.and_then(|event| {
                    AdvertisingReport::meta_unpack_packet(event.as_ref().as_ref())
                        .map_err(|e| adapter::Error::StreamError(StreamError::EventError(e)))
                }))
            },
        ))
    }
    pub async fn advertisement_stream<
        'a,
        'b: 'a,
        Buf: Storage<ReportInfo<StaticAdvBuffer>> + 'b,
    >(
        &'a mut self,
    ) -> Result<
        impl Stream<Item = Result<ReportInfo<StaticAdvBuffer>, adapter::Error>> + 'a,
        adapter::Error,
    > {
        Ok(self
            .advertising_report_stream::<Buf>()
            .await?
            .map(
                |r: Result<AdvertisingReport<Buf>, adapter::Error>| match r {
                    Ok(report) => futures_util::future::Either::Left(
                        futures_util::stream::iter(report).map(Ok),
                    ),
                    Err(err) => futures_util::future::Either::Right(futures_util::stream::once(
                        async move { Err(err) },
                    )),
                },
            )
            .flatten())
    }
}
/*
impl<A: adapter::Adapter, S: Deref<Target = A> + DerefMut> Advertiser for LEAdapter<A, S> {
    fn set_advertising_enable(
        &mut self,
        is_enabled: bool,
    ) -> BoxFuture<Result<(), adapter::Error>> {
        Box::pin(LEAdapter::set_advertising_enable(self, is_enabled))
    }

    fn set_advertising_parameters(
        &mut self,
        advertisement_parameters: AdvertisingParameters,
    ) -> BoxFuture<Result<(), adapter::Error>> {
        Box::pin(LEAdapter::set_advertising_parameters(
            self,
            advertisement_parameters,
        ))
    }

    fn set_advertising_data<'s, 'b: 's>(
        &'b mut self,
        data: &'s [u8],
    ) -> BoxFuture<'s, Result<(), adapter::Error>> {
        Box::pin(LEAdapter::set_advertising_data(self, data))
    }
}
*/
