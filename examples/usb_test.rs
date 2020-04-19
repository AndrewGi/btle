use btle::hci::usb;
use driver_async::error::IOError;

// Using a CSR8510 A10 (PID = 0x0001, VID = 0x0A12) in my case
const USB_PRODUCT_ID: u16 = 0x0001;
const USB_VENDOR_ID: u16 = 0x0A12;

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
    let adapter = context
        .open_adapter(USB_VENDOR_ID, USB_PRODUCT_ID)?
        .ok_or(usb::Error(IOError::NotFound))?;
    println!("got adapter {:?}", adapter.device_descriptor);
    /*
    println!(
        "interface: {:?}",
        adapter
            .handle
            .device()
            .active_config_descriptor()?
            .interfaces()
            .next()
            .ok_or(usb::Error(IOError::NotFound))?
            .descriptors()
            .next()
            .ok_or(usb::Error(IOError::NotFound))?
    );
    */
    Ok(())
}
