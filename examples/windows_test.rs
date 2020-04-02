use btle::asyncs::stream::StreamExt;
use btle::error::StdError;
use btle::windows;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(main_async())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");
    let mut watcher = windows::ble::observer::Watcher::new().map_err(StdError)?;
    let mut stream = watcher.advertisement_stream().map_err(StdError)?;
    println!("waiting for next advertisement");
    println!("{:?}", stream.next().await);
    Ok(())
}
