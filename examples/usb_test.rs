use btle::error::IOError;
use btle::hci::usb;

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
    loop {
        let packet = adapter.read_event_packet::<Box<[u8]>>()?;
        println!("got event! {:?}", packet);
    }
}
