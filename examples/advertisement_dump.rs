use btle::hci::adapters::le::AdvertisementStream;
use btle::hci::event::EventCode;
use btle::hci::le;
use btle::hci::le::report::ReportInfo;
use btle::hci::packet::PacketType;
use futures_util::StreamExt;
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
    use btle::error::STDError;
    let manager = btle::hci::socket::Manager::new().map_err(STDError)?;
    let socket = manager
        .get_adapter_socket(btle::hci::socket::AdapterID(adapter_id))
        .map_err(STDError)?;
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
            .map_err(|e| Box::new(btle::error::STDError(e)))?;
        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
pub async fn dump_adapter<S: btle::hci::stream::HCIStreamable>(
    mut adapter: btle::hci::adapters::Adapter<S>,
) -> Result<(), btle::hci::adapters::Error> {
    let mut adapter = unsafe { Pin::new_unchecked(&mut adapter) };
    //adapter.as_mut().le().set_scan_enabled(false, false).await?;
    let mut le = adapter.as_mut().le();
    le.set_scan_parameters(le::SetScanParameters::DEFAULT)
        .await?;

    le.set_scan_enabled(true, false).await?;

    let mut filter = btle::hci::stream::Filter::default();
    filter.enable_type(PacketType::Event);
    filter.enable_event(EventCode::LEMeta);
    le.adapter_mut()
        .stream_pinned()
        .stream_pinned()
        .set_filter(&filter)?;
    let mut stream: AdvertisementStream<S, Box<[ReportInfo]>> = le.advertisement_stream();
    let mut stream = Pin::new(&mut stream);
    loop {
        while let Some(report) = StreamExt::next(&mut stream).await {
            println!("report: {:?}", &report);
        }
    }
}
