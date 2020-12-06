use crate::le::scan;
use crate::le::scan::ScanType;
use crate::windows::WindowsError;
use crate::{
    bytes::Storage,
    bytes::ToFromBytesEndian,
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

/// Wrapper around `winrt`'s `BluetoothLEAdvertisementWatcher`.
pub struct Watcher {
    watcher: BluetoothLEAdvertisementWatcher,
    output: mpsc::Receiver<ReportInfo>,
}
impl Watcher {
    const DEFAULT_CAPACITY: usize = 16;
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
    fn u64_to_bluetooth_address(u: u64) -> BTAddress {
        BTAddress::new(&u.to_bytes_le()[..BTAddress::LEN])
    }
    fn event_args_to_report_info(
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Result<ReportInfo, WindowsError> {
        Ok(ReportInfo {
            event_type: Self::advertisement_type_to_event_type(args.advertisement_type()?),
            address_type: AddressType::PublicDevice,
            address: Self::u64_to_bluetooth_address(args.bluetooth_address()?),
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
    pub fn new() -> Result<Watcher, WindowsError> {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }
    pub fn with_capacity(capacity: usize) -> Result<Watcher, WindowsError> {
        let filter = BluetoothLEAdvertisementFilter::new()?;
        let watcher = BluetoothLEAdvertisementWatcher::create(&filter)?;
        let (mut tx, rx) = mpsc::channel(capacity);
        watcher.received(&foundation::TypedEventHandler::new(
            move |_sender, args: &BluetoothLEAdvertisementReceivedEventArgs| {
                // Unsafe just to dereference the Event Pointer.
                match tx.try_send(Self::event_args_to_report_info(args).map_err(|e| e.0)?) {
                    Ok(_) => {}
                    Err(e) => match e {
                        TrySendError::Full(_) => {}
                        TrySendError::Closed(_) => {
                            return Err(winrt::Error::new(
                                winrt::ErrorCode(0x77370002),
                                "le watcher receiver closed",
                            ))
                        }
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
    pub fn advertisement_stream(&mut self) -> AdvertisementStream<'_> {
        AdvertisementStream::new(self)
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
        Pin::new(&mut self.0.output).poll_recv(cx)
    }
}
