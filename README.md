# btle - Rust Bluetooth Low Energy Driver
[![crates.io](https://img.shields.io/crates/v/btle)](https://crates.io/crates/btle)
[![docs.rs](https://docs.rs/btle/badge.svg)](https://docs.rs/btle)
![last commit](https://img.shields.io/github/last-commit/AndrewGi/btle)

Cross-platform Bluetooth Low Energy library for Rust. Supports Central, Peripheral, Broadcaster and Observer GAP roles. Also supports custom device drivers to enable platform support for custom platforms (embedded, etc).

Very much WIP.


Supported Platforms so far:
- [x] Linux (BlueZ)
- [x] HCI
- [x] USB (using `libusb`)
- [ ] Proxy
- [x] Windows 10 / UWP
- [x] Windows 7 (must use `libusb` drivers)
- [ ] macOS
- [ ] iOS
- [ ] Android

Any platforms missings drivers should still be able to compile, just without any built in way to talk to the BLE controller. 

Supported GAP Roles so far:
- [x] Observer (Receiver Advertisements)
- [ ] Broadcaster (Send Advertisements)
- [ ] Central (Initiate GATT Connection)
- [ ] Peripheral (GATT Connectable)

WIP Example (API may change later):
```rust
pub async fn dump_adapter<A: btle::hci::adapter::Adapter>(adapter: A) -> Result<(), CLIError> {
    let adapter = btle::hci::adapters::Adapter::new(adapter);
    let mut le = adapter.le();
    println!("resetting adapter...");
    le.adapter.reset().await?;
    println!("settings scan parameters...");
    // Set BLE Scan parameters (when to scan, how long, etc)
    le.set_scan_parameters(btle::le::scan::ScanParameters::DEFAULT)
        .await?;
    // Enable scanning for advertisement packets.
    le.set_scan_enable(true, false).await?;

    println!("waiting for advertisements...");
    // Create the advertisement stream from the LEAdapter.
    let mut stream = le.advertisement_stream::<Box<[ReportInfo]>>().await?;
    // Pin it.
    let mut stream = unsafe { Pin::new_unchecked(&mut stream) };
    loop {
        // Asynchronously iterate through the stream and print each advertisement report.
        while let Some(report) = stream.next().await {
            println!("report: {:?}", &report);
        }
    }
}
```