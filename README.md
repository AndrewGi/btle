# btle
Cross-platform Bluetooth Low Energy library for Rust. Supports Central, Peripheral, Broadcaster and Observer GAP roles. Also supports custom device drivers to enable platform support for custom platforms (embedded, etc).

Very much WIP.


Supported Platforms so far:
- [x] Linux (BlueZ)
- [x] HCI
- [ ] Proxy
- [ ] Windows 10
- [ ] UWP
- [ ] Windows 7
- [ ] macOS
- [ ] iOS
- [ ] Andriod

Any platforms missings drivers should still be able to compile, just without any built in way to talk to the BLE controller. 

Supported GAP Roles so far:
- [x] Observer (Receiver Advertisements)
- [ ] Broadcaster (Send Advertisements)
- [ ] Central (Initiate GATT Connection)
- [ ] Peripheral (GATT Connectable)

WIP Example (API may change later):
```rust
pub async fn dump_adapter<S: btle::hci::stream::HCIStreamable>(
    mut adapter: btle::hci::adapters::Adapter<S>,
) -> Result<(), btle::hci::adapters::Error> {
    let mut adapter = Pin::new(&mut adapter);
    let mut le = adapter.as_mut().le();
    // Set BLE Scan parameters (when to scan, how long, etc)
    le.set_scan_parameters(le::SetScanParameters::DEFAULT)
        .await?;
    // Enable scanning for advertisement packets.
    le.set_scan_enabled(true, false).await?;

    // Set the HCI filter to only allow LEMeta events.
    let mut filter = btle::hci::stream::Filter::default();
    filter.enable_type(PacketType::Event);
    filter.enable_event(EventCode::LEMeta);
    le.adapter_mut()
        .stream_pinned()
        .stream_pinned()
        .set_filter(&filter)?;
    // Create the advertisement stream from the LEAdapter.
    let mut stream: AdvertisementStream<S, Box<[ReportInfo]>> = le.advertisement_stream();
    // Pin it.
    let mut stream = Pin::new(&mut stream);
    loop {
        // Asynchronously iterate through the stream and print each advertisement report.
        while let Some(report) = StreamExt::next(&mut stream).await {
            println!("report: {:?}", &report);
        }
    }
}
```