use btle::asyncs::stream::StreamExt;
use btle::hci::adapters::le::AdvertisementStream;
use btle::hci::event::EventCode;
use btle::hci::packet::PacketType;
use btle::le;
use btle::le::report::ReportInfo;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::pin::Pin;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    dump_bluez(
        std::env::args()
            .skip(1)
            .next()
            .unwrap_or("0".to_owned())
            .parse()
            .expect("invalid adapter id"),
    )?;
    #[cfg(not(unix))]
    dump_not_supported()?;
    Ok(())
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
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(async move {
        let async_socket = btle::hci::socket::AsyncHCISocket::try_from(socket)?;
        let stream = btle::hci::stream::Stream::new(async_socket);
        let adapter = btle::hci::adapters::Adapter::new(stream);
        dump_adapter(adapter)
            .await
            .map_err(|e| Box::new(btle::error::StdError(e)))?;
        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
pub async fn dump_adapter<S: btle::hci::stream::HCIStreamable>(
    mut adapter: btle::hci::adapters::Adapter<S>,
) -> Result<(), btle::le::adapter::Error> {
    let mut adapter = Pin::new(&mut adapter);
    let mut le = adapter.as_mut().le();
    // Set BLE Scan parameters (when to scan, how long, etc)
    le.set_scan_parameters(le::scan::ScanParameters::DEFAULT)
        .await?;
    // Enable scanning for advertisement packets.
    le.set_scan_enable(true, false).await?;

    // Set the HCI filter to only allow LEMeta events.
    let mut filter = btle::hci::stream::Filter::default();
    filter.enable_type(PacketType::Event);
    filter.enable_event(EventCode::LEMeta);
    le.adapter_mut()
        .stream_pinned()
        .stream_pinned()
        .set_filter(&filter)?;
    // Create the advertisement stream from the LEAdapter.
    let mut stream: AdvertisementStream<S, Box<[ReportInfo]>> = le.advertisement_stream();
    // Pin it.
    let mut stream = Pin::new(&mut stream);
    loop {
        // Asynchronously iterate through the stream and print each advertisement report.
        while let Some(report) = StreamExt::next(&mut stream).await {
            println!("report: {:?}", &report);
        }
    }
}
