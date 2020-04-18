use btle::hci::usb;
use driver_async::error::StdError;

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
    let context = usb::manager::Manager::new().map_err(StdError)?;
    println!("context made");
    for d in context.devices()?.iter() {
        println!("{:?}", d);
    }
    Ok(())
}
