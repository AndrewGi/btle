use btle::error::IOError;
use btle::hci::adapters::Adapter;
use btle::hci::usb;
use btle::le;
use btle::le::scan::ScanParameters;
use futures_util::StreamExt;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runetime::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(main_async())?;
    Ok(())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");
    let context = usbw::libusb::context::Context::default()?;
    let context = context.start_async();
    println!("context made");
    let device_list = context.context_ref().device_list();
    for d in usb::device::bluetooth_adapters(device_list.iter()) {
        println!(
            "{:?}",
            d.and_then(|d| d.device_descriptor().map_err(usb::Error::from))
        );
    }
    println!("opening first device...");
    let mut devices = usb::device::bluetooth_adapters(device_list.iter());
    let handle = loop {
        let device = devices.next().ok_or(IOError::NotFound)??;
        if usb::device::has_bluetooth_interface(&device).unwrap_or(false) {
            println!("using {:?}", device);
            match device.open() {
                Ok(handle) => break handle,
                Err(usbw::libusb::error::Error::NotSupported) => (),
                Err(e) => Err(e)?,
            };
        }
    };
    let handle = context.make_async_device(handle);
    let adapter = usb::adapter::Adapter::open(handle)?;
    println!(
        "got adapter! '{:?}' {}",
        adapter.get_product_string().await,
        adapter.device_identifier()
    );
    let mut adapter = Adapter::new(adapter);
    println!("resetting...");
    // Reset the HCI Adapter
    adapter.reset().await?;
    println!("HCI reset");
    let mut le = adapter.le();
    le.set_scan_parameters(le::scan::ScanParameters::DEFAULT)
        .await?;
    le.set_scan_enable(true, false).await?;
    let mut event_stream = le
        .advertising_report_stream::<Box<[le::report::ReportInfo]>>()
        .await?;
    let mut event_stream = unsafe { core::pin::Pin::new_unchecked(&mut event_stream) };
    println!("starting loop...");
    while let Some(event) = event_stream.next().await {
        println!("got event! {:?}", event);
    }
    Ok(())
}
