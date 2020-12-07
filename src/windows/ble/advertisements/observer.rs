use crate::le::scan;
use crate::le::scan::ScanType;
use crate::windows::WindowsError;
use crate::{
    bytes::Storage,
    le::advertisement::{AdType, RawAdStructureBuffer, RawAdvertisement, StaticAdvStructBuf},
    le::report::{AddressType, EventType, ReportInfo},
    BTAddress, RSSI,
};
use core::{
    convert::{TryFrom, TryInto},
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::Stream;
use std::marker::PhantomData;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use winrt_bluetooth_bindings::windows::{
    devices::bluetooth::advertisement::{
        BluetoothLEAdvertisementDataSection, BluetoothLEAdvertisementFilter,
        BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementType,
        BluetoothLEAdvertisementWatcher, BluetoothLEScanningMode,
    },
    foundation,
    storage::streams::DataReader,
};
pub trait RawWatcherCallback: 'static {
    fn callback(
        &mut self,
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<(), WindowsError>;
}
impl<T> RawWatcherCallback for T
where
    T: FnMut(&BluetoothLEAdvertisementReceivedEventArgs) -> Result<(), WindowsError> + 'static,
{
    fn callback(
        &mut self,
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<(), WindowsError> {
        (self)(args)
    }
}
/// Wrapper around `winrt`'s `BluetoothLEAdvertisementWatcher`.
pub struct RawWatcher<Callback> {
    watcher: BluetoothLEAdvertisementWatcher,
    _marker: PhantomData<Callback>,
}
impl<Callback: RawWatcherCallback> RawWatcher<Callback> {
    pub fn new(mut callback: Callback) -> Result<Self, WindowsError> {
        let filter = BluetoothLEAdvertisementFilter::new()?;
        let watcher = BluetoothLEAdvertisementWatcher::create(&filter)?;
        watcher.received(&foundation::TypedEventHandler::new(
            move |_sender, args: &BluetoothLEAdvertisementReceivedEventArgs| {
                callback.callback(args).map_err(|e| e.0)
            },
        ))?;
        Ok(RawWatcher {
            watcher,
            _marker: PhantomData::default(),
        })
    }

    pub fn set_scan_enable(&mut self, is_enabled: bool) -> Result<(), WindowsError> {
        if is_enabled {
            self.watcher.start()?;
        } else {
            self.watcher.stop()?;
        }
        Ok(())
    }
    pub fn set_scanning_mode(&mut self, scanning_mode: scan::ScanType) -> Result<(), WindowsError> {
        let mode = match scanning_mode {
            ScanType::Passive => BluetoothLEScanningMode::Passive,
            ScanType::Active => BluetoothLEScanningMode::Active,
        };
        self.watcher.set_scanning_mode(mode)?;
        Ok(())
    }
}
struct ReportInfoCallback {
    sender: mpsc::Sender<ReportInfo>,
}
impl ReportInfoCallback {
    pub fn from_sender(sender: mpsc::Sender<ReportInfo>) -> Self {
        ReportInfoCallback { sender }
    }
    fn data_section_to_raw_ad_struct(
        data_sec: &BluetoothLEAdvertisementDataSection,
    ) -> Result<RawAdStructureBuffer, WindowsError> {
        let ad_type = AdType::try_from(data_sec.data_type()?).expect("bad advertisement part");
        let reader = DataReader::from_buffer(&data_sec.data()?)?;
        let len: u32 = reader.unconsumed_buffer_length()?;
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
    fn callback(
        &mut self,
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<(), WindowsError> {
        match self
            .sender
            .try_send(Self::event_args_to_report_info(args).map_err(|e| e.0)?)
        {
            Ok(_) => Ok(()),
            Err(e) => match e {
                TrySendError::Full(_) => Ok(()),
                TrySendError::Closed(_) => Err(winrt::Error::new(
                    winrt::ErrorCode(0x77370002),
                    "le watcher receiver closed",
                )
                .into()),
            },
        }
    }

    fn event_args_to_report_info(
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<ReportInfo, WindowsError> {
        Ok(ReportInfo {
            event_type: Self::advertisement_type_to_event_type(args.advertisement_type()?),
            address_type: AddressType::PublicDevice,
            address: BTAddress::from_u64(args.bluetooth_address()?),
            data: {
                let mut out = RawAdvertisement::default();
                for data_sec in args.advertisement()?.data_sections()?.into_iter() {
                    out.insert(&Self::data_section_to_raw_ad_struct(&data_sec)?)
                        .map_err(|_| {
                            winrt::Error::new(
                                winrt::ErrorCode(0x77370001),
                                "unable to convert data section to raw ad struct",
                            )
                        })?;
                }
                out
            },
            rssi: Some(RSSI::new(
                args.raw_signal_strength_in_dbm()?
                    .try_into()
                    .expect("invalid rssi"),
            )),
        })
    }
}
impl RawWatcherCallback for ReportInfoCallback {
    fn callback(
        &mut self,
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<(), WindowsError> {
        self.callback(args)
    }
}
pub struct ReportInfoWatcher {
    watcher: RawWatcher<ReportInfoCallback>,
    rx: mpsc::Receiver<ReportInfo>,
}
impl ReportInfoWatcher {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new() -> Result<Self, WindowsError> {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }
    pub fn with_capacity(capacity: usize) -> Result<Self, WindowsError> {
        let (tx, rx) = mpsc::channel(capacity);
        let watcher = RawWatcher::new(ReportInfoCallback::from_sender(tx))?;
        Ok(Self { watcher, rx })
    }
    pub fn advertisement_stream(&mut self) -> AdvertisementStream<'_> {
        AdvertisementStream::new(self)
    }
}

pub struct AdvertisementStream<'a>(&'a mut ReportInfoWatcher);
impl<'a> AdvertisementStream<'a> {
    pub fn new(watcher: &'a mut ReportInfoWatcher) -> Self {
        Self(watcher)
    }
}
impl<'a> Stream for AdvertisementStream<'a> {
    type Item = ReportInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0.rx).poll_recv(cx)
    }
}
