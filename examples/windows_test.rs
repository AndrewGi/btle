use futures_util::StreamExt;
use btle::le::scan::Observer;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runtime::new()
        .expect("can't make async runtime");
    runtime.block_on(main_async())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");
    let mut watcher = btle::windows::ble::advertisements::observer::ReportInfoWatcher::new()?;
    watcher.set_scan_enable(true, false).await?;
    let mut stream = watcher.advertisement_stream();
    println!("waiting for next advertisement");
    loop {
        println!("{:?}", stream.next().await);
    }
}
