use btle::error::IOError;
use btle::hci::adapters::Adapter;
use btle::hci::usb;
use futures_util::StreamExt;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(main_async())?;
    Ok(())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");
    let context = usbw::libusb::context::Context::default()?;
    println!("context made");
    let device_list = context.device_list();
    for d in usb::device::bluetooth_adapters(device_list.iter()) {
        println!(
            "{:?}",
            d.and_then(|d| d.device_descriptor().map_err(usb::Error::from))
        );
    }
    println!("opening first device...");
    let mut devices = usb::device::bluetooth_adapters(device_list.iter());
    let adapter = loop {
        let device = devices.next().ok_or(IOError::NotFound)??;
        println!("using {:?}", device);
        let adapter = usb::adapter::Adapter::open(device);
        match adapter {
            Ok(adapter) => break adapter,
            Err(btle::hci::usb::Error(IOError::NotImplemented)) => (),
            Err(e) => Err(e)?,
        }
    };

    println!(
        "got adapter! '{:?}' {}",
        adapter.get_product_string(),
        adapter.device_identifier()
    );
    let mut adapter = Adapter::new(adapter);
    println!("resetting...");
    // Reset the HCI Adapter
    adapter.reset().await?;
    println!("HCI reset");
    let mut event_stream = adapter.hci_event_stream::<Box<[u8]>>();
    let mut event_stream = unsafe { core::pin::Pin::new_unchecked(&mut event_stream) };
    println!("starting loop...");
    while let Some(event) = event_stream.next().await {
        println!("got event! {:?}", event);
    }
    Ok(())
}
