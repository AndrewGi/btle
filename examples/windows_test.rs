use btle::windows;
use futures_util::StreamExt;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(main_async())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");
    let mut watcher = windows::ble::observer::Watcher::new()?;
    watcher.set_scan_enable(true)?;
    let mut stream = watcher.advertisement_stream();
    println!("waiting for next advertisement");
    loop {
        println!("{:?}", stream.next().await);
    }
}
