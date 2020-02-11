# btle
Cross-platform Bluetooth Low Energy library for Rust. Supports Central, Peripheral, Broadcaster and Observer GAP roles. Also supports custom device drivers to enable platform support for custom platforms (embedded, etc).

Very much WIP.


Supported Platforms so far:
- [x] Linux (BlueZ)
- [x] HCI
- [x] Proxy
- [ ] Windows 10
- [ ] UWP
- [ ] Windows 7
- [ ] macOS
- [ ] iOS
- [ ] Andriod

Any platforms missings drivers should still be able to compile, just without any built in way to talk to the BLE controller. 

Supported GAP Roles so far:
- [x] Observer (Receiver Advertisements)
- [x] Broadcaster (Send Advertisements)
- [ ] Central (Initiate GATT Connection)
- [ ] Peripheral (GATT Connectable)
