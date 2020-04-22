use btle::error::IOError;
use btle::le;
use btle::le::report::ReportInfo;
use futures_util::stream::StreamExt;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::pin::Pin;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(async move {
        #[cfg(unix)]
        dump_bluez(
            std::env::args()
                .skip(1)
                .next()
                .unwrap_or("0".to_owned())
                .parse()
                .expect("invalid adapter id"),
        )
        .await;
        #[cfg(feature = "hci_usb")]
        dump_usb().await?;
        #[cfg(not(unix))]
        dump_not_supported()?;
        Ok(())
    })
}

pub fn dump_not_supported() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("no known support adapter for this platform. (When this example was written)");
    Ok(())
}
#[cfg(unix)]
pub fn dump_bluez(adapter_id: u16) -> Result<(), Box<dyn std::error::Error>> {
    use btle::error::StdError;
    let manager = btle::hci::socket::Manager::new().map_err(StdError)?;
    let socket = match manager.get_adapter_socket(btle::hci::socket::AdapterID(adapter_id)) {
        Ok(socket) => socket,
        Err(btle::hci::socket::HCISocketError::PermissionDenied) => {
            eprintln!("Permission denied error when opening the HCI socket. Maybe run as sudo?");
            return Err(btle::hci::socket::HCISocketError::PermissionDenied)
                .map_err(StdError)
                .map_err(Into::into);
        }
        Err(e) => return Err(StdError(e).into()),
    };

    let async_socket = btle::hci::socket::AsyncHCISocket::try_from(socket)?;
    let stream = btle::hci::stream::Stream::new(async_socket);
    let adapter = btle::hci::adapters::Adapter::new(stream);
    dump_adapter(adapter)
        .await
        .map_err(|e| Box::new(btle::error::StdError(e)))?;
    Result::<(), Box<dyn std::error::Error>>::Ok(())
}
#[cfg(feature = "hci_usb")]
pub async fn dump_usb() -> Result<(), btle::hci::adapter::Error> {
    use btle::hci::usb;
    let context = usb::manager::Manager::new()?;
    println!("opening first device...");
    let device: usb::device::Device = context
        .devices()?
        .bluetooth_adapters()
        .next()
        .ok_or(IOError::NotFound)??;
    println!("using {:?}", device);
    let mut adapter = device.open()?;
    adapter.reset()?;
    dump_adapter(adapter).await
}
pub async fn dump_adapter<A: btle::hci::adapter::Adapter>(
    mut adapter: A,
) -> Result<(), btle::hci::adapter::Error> {
    let adapter = unsafe { Pin::new_unchecked(&mut adapter) };
    let adapter = btle::hci::adapters::Adapter::new(adapter);
    let mut le = adapter.le();

    le.adapter_mut().reset().await?;
    println!("settings scan parameters...");
    // Set BLE Scan parameters (when to scan, how long, etc)
    le.set_scan_parameters(le::scan::ScanParameters::DEFAULT)
        .await?;
    // Enable scanning for advertisement packets.
    le.set_scan_enable(true, false).await?;

    println!("waiting for advertisements...");
    // Create the advertisement stream from the LEAdapter.
    let mut stream = le.advertisement_stream::<Box<[ReportInfo]>>().await?;
    // Pin it.
    let mut stream = unsafe { Pin::new_unchecked(&mut stream) };
    loop {
        // Asynchronously iterate through the stream and print each advertisement report.
        while let Some(report) = stream.next().await {
            println!("report: {:?}", &report);
        }
    }
}
