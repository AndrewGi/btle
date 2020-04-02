use crate::asyncs::future::ready;
use crate::asyncs::sync::mpsc::TrySendError;
use crate::bytes::Storage;
use crate::error::IOError;
use crate::le::adapter::Error;
use crate::le::advertisement::{
    AdType, RawAdStructureBuffer, RawAdvertisement, StaticAdvBuffer, StaticAdvStructBuf,
};
use crate::le::report::{AddressType, EventType, ReportInfo};
use crate::le::scan::{Observer, ScanParameters};
use crate::{asyncs, bytes::ToFromBytesEndian, BTAddress, BoxFuture, BoxStream, RSSI};
use core::convert::{TryFrom, TryInto};
use core::ops::Deref;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::Stream;
use winrt::windows::devices::bluetooth::advertisement::{
    BluetoothLEAdvertisementDataSection, BluetoothLEAdvertisementFilter,
    BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementType,
    BluetoothLEAdvertisementWatcher,
};
use winrt::windows::storage::streams::DataReader;
use winrt::{ComPtr, RtDefaultConstructible};

/// Wrapper around `winrt`'s `BluetoothLEAdvertisementWatcher`.
pub struct Watcher {
    watcher: ComPtr<BluetoothLEAdvertisementWatcher>,
    output: asyncs::sync::mpsc::Receiver<ReportInfo>,
}
impl Watcher {
    const DEFAULT_CAPACITY: usize = 16;
    fn data_section_to_raw_ad_struct(
        data_sec: &BluetoothLEAdvertisementDataSection,
    ) -> Result<RawAdStructureBuffer, winrt::Error> {
        let ad_type = AdType::try_from(data_sec.get_data_type()?).expect("bad advertisement part");
        let reader = DataReader::from_buffer(&data_sec.get_data()?.expect("missing data"))?
            .expect("reader should exist from IBuffer");
        let len: u32 = reader.deref().deref().get_unconsumed_buffer_length()?;
        let mut buf = StaticAdvStructBuf::with_size(len as usize);
        reader.read_bytes(buf.as_mut())?;
        Ok(RawAdStructureBuffer::new(ad_type, buf))
    }
    fn advertisement_type_to_event_type(t: BluetoothLEAdvertisementType) -> EventType {
        match t {
            BluetoothLEAdvertisementType::ConnectableUndirected => EventType::AdvInd,
            BluetoothLEAdvertisementType::ConnectableDirected => EventType::AdvDirectInd,
            BluetoothLEAdvertisementType::ScannableUndirected => EventType::AdvScanInd,
            BluetoothLEAdvertisementType::NonConnectableUndirected => EventType::AdvNonconnInd,
            BluetoothLEAdvertisementType::ScanResponse => EventType::ScanRsp,
            _ => unreachable!("all bluetooth le advertisements types"),
        }
    }
    fn u64_to_bluetooth_address(u: u64) -> BTAddress {
        BTAddress::new(&u.to_bytes_le()[..BTAddress::LEN])
    }
    fn event_args_to_report_info(
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<ReportInfo, winrt::Error> {
        Ok(ReportInfo {
            event_type: Self::advertisement_type_to_event_type(args.get_advertisement_type()?),
            address_type: AddressType::PublicDevice,
            address: Self::u64_to_bluetooth_address(args.get_bluetooth_address()?),
            data: {
                let mut out = RawAdvertisement::default();
                for data_sec in args
                    .get_advertisement()?
                    .expect("missing advertisement")
                    .get_data_sections()?
                    .expect("there should be data sections")
                    .into_iter()
                {
                    if let Some(data_sec) = &data_sec {
                        out.insert(&Self::data_section_to_raw_ad_struct(&data_sec)?)
                            .map_err(|_| winrt::Error::OutOfMemory)?;
                    }
                }
                out
            },
            rssi: Some(RSSI::new(
                args.get_raw_signal_strength_in_dbm()?
                    .try_into()
                    .expect("invalid rssi"),
            )),
        })
    }
    pub fn new() -> Result<Watcher, IOError> {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }
    pub fn with_capacity(capacity: usize) -> Result<Watcher, IOError> {
        let filter = BluetoothLEAdvertisementFilter::new();
        let watcher = BluetoothLEAdvertisementWatcher::create(&filter)?;
        let (mut tx, rx) = asyncs::sync::mpsc::channel(capacity);
        watcher.add_received(&winrt::windows::foundation::TypedEventHandler::new(
            move |_sender, args: *mut BluetoothLEAdvertisementReceivedEventArgs| {
                // Unsafe just to dereference the Event Pointer.
                let args = unsafe { &*args };
                match tx.try_send(Self::event_args_to_report_info(args)?) {
                    Ok(_) => {}
                    Err(e) => match e {
                        TrySendError::Full(_) => {}
                        TrySendError::Closed(_) => return Err(winrt::Error::ObjectClosed),
                    },
                }
                Ok(())
            },
        ))?;
        Ok(Watcher {
            watcher,
            output: rx,
        })
    }
    pub fn set_scan_enable(&mut self, is_enabled: bool) -> Result<(), IOError> {
        if is_enabled {
            self.watcher.start()?;
        } else {
            self.watcher.stop()?;
        }
        Ok(())
    }
    pub fn advertisement_stream(&mut self) -> AdvertisementStream<'_> {
        AdvertisementStream::new(self)
    }
}
impl Observer for Watcher {
    fn set_scan_parameters<'a>(
        &'a mut self,
        _scan_parameters: ScanParameters,
    ) -> BoxFuture<'a, Result<(), Error>> {
        unimplemented!()
    }

    fn set_scan_enable<'a>(
        &'a mut self,
        is_enabled: bool,
        _filter_duplicates: bool,
    ) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(ready(
            self.set_scan_enable(is_enabled).map_err(Error::IOError),
        ))
    }

    fn advertisement_stream<'a>(
        &'a mut self,
    ) -> BoxStream<'a, Result<ReportInfo<StaticAdvBuffer>, Error>> {
        unimplemented!()
    }
}
pub struct AdvertisementStream<'a>(&'a mut Watcher);
impl<'a> AdvertisementStream<'a> {
    pub fn new(watcher: &'a mut Watcher) -> Self {
        Self(watcher)
    }
}
impl<'a> Stream for AdvertisementStream<'a> {
    type Item = ReportInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0.output).poll_next(cx)
    }
}
