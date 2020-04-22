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
    let context = usb::manager::Manager::new()?;
    println!("context made");
    for d in context.devices()?.bluetooth_adapters() {
        println!("{:?}", d);
    }
    println!("opening first device...");
    let device: usb::device::Device = context
        .devices()?
        .bluetooth_adapters()
        .next()
        .ok_or(IOError::NotFound)??;
    println!("using {:?}", device);
    let mut adapter = device.open()?;
    println!("got adapter! {:?}", adapter);
    let mut adapter = Adapter::pin(&mut adapter);
    // Read the HCI Capabilities (always first event)
    adapter.hci_read_event::<Box<[u8]>>().await?;
    // Reset the HCI Adapter
    adapter.reset().await?;
    let mut event_stream = adapter.hci_event_stream::<Box<[u8]>>();
    let mut event_stream = unsafe { core::pin::Pin::new_unchecked(&mut event_stream) };
    while let Some(event) = event_stream.next().await {
        println!("got event! {:?}", event);
    }
    Ok(())
}
