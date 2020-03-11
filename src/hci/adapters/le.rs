use crate::bytes::Storage;
use crate::hci::event::{Event, EventPacket};
use crate::hci::le;
use crate::hci::le::random::RAND_LEN;
use crate::hci::packet::RawPacket;
use crate::hci::stream::HCIStreamable;
use crate::hci::{adapters, stream};
use core::convert::TryFrom;
use core::future::Future;
use core::pin::Pin;
use futures_task::{Context, Poll};

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
    pub async fn set_scan_enabled(
        &mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetScanEnable {
                is_enabled,
                filter_duplicates,
            })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn set_scan_parameters(
        &mut self,
        scan_parameters: le::SetScanParameters,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(scan_parameters)
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn set_advertising_enabled(
        &mut self,
        is_enabled: bool,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetAdvertisingEnable { is_enabled })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Get `RAND_LEN` (8) bytes from the HCI Controller.
    pub async fn get_rand(&mut self) -> Result<[u8; RAND_LEN], adapters::Error> {
        let r = self.adapter_mut().send_command(le::Rand {}).await?;
        r.params.status.error()?;
        Ok(r.params.random_bytes)
    }
    /// Create an BLE Advertisement Stream that returns
    /// `le::report::ReportInfo<le::report::StaticAdvBuffer>>`. Make sure you set scan parameters
    /// and a `Filter` before calling this.
    pub fn advertisement_stream<
        't,
        Buf: Storage<le::report::ReportInfo<le::report::StaticAdvBuffer>>,
    >(
        &'t mut self,
    ) -> AdvertisementStream<'t, 'a, S, Buf> {
        AdvertisementStream::new(self)
    }
}
pub struct AdvertisementStream<
    'a,
    'b: 'a,
    S: HCIStreamable,
    Buf: Storage<le::report::ReportInfo<PacketBuf>>,
    PacketBuf: Storage<u8> + Copy + Default = le::report::StaticAdvBuffer,
> {
    adapter: &'a mut LEAdapter<'b, S>,
    last_report: Option<(le::AdvertisingReport<Buf, PacketBuf>, usize)>,
    marker_: core::marker::PhantomData<PacketBuf>,
}
impl<
        'a,
        'b: 'a,
        S: HCIStreamable,
        Buf: Storage<le::report::ReportInfo<PacketBuf>>,
        PacketBuf: Storage<u8> + Copy + Default,
    > AdvertisementStream<'a, 'b, S, Buf, PacketBuf>
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
        Buf: Storage<le::report::ReportInfo<PacketBuf>>,
        PacketBuf: Storage<u8> + Copy + Default,
    > futures_core::Stream for AdvertisementStream<'a, 'b, S, Buf, PacketBuf>
{
    type Item = Result<le::report::ReportInfo<PacketBuf>, adapters::Error>;

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
            .map(|p| le::AdvertisingReport::unpack_event_packet(&p))
        {
            Ok(Ok(reports)) => reports,
            Ok(Err(e)) | Err(e) => {
                return Poll::Ready(Some(Err(adapters::Error::StreamError(
                    stream::Error::CommandError(e),
                ))))
            }
        };
        self.last_report = Some((reports, 0));
        self.poll_next(cx)
    }
}
