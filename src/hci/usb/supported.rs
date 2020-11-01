//! List of Known supported USB Bluetooth HCI adapters. There are many more not listed here.

use usbw::device::{DeviceIdentifier, ProductID, VendorID};

pub static KNOWN_DEVICES: [DeviceIdentifier; 12] = [
    DeviceIdentifier {
        vendor_id: VendorID(0x0CF3),
        product_id: ProductID(0xE300),
    }, // Qualcomm Atheros QCA61x4
    DeviceIdentifier {
        vendor_id: VendorID(0x0a5c),
        product_id: ProductID(0x21e8),
    }, // Broadcom BCM20702A0
    DeviceIdentifier {
        vendor_id: VendorID(0x19ff),
        product_id: ProductID(0x0239),
    }, // Broadcom BCM20702A0
    DeviceIdentifier {
        vendor_id: VendorID(0x0a12),
        product_id: ProductID(0x0001),
    }, // CSR8510 A10
    DeviceIdentifier {
        vendor_id: VendorID(0x0b05),
        product_id: ProductID(0x17cb),
    }, // ASUS BT400
    DeviceIdentifier {
        vendor_id: VendorID(0x8087),
        product_id: ProductID(0x07da),
    }, // Intel 6235
    DeviceIdentifier {
        vendor_id: VendorID(0x8087),
        product_id: ProductID(0x07dc),
    }, // Intel 7260
    DeviceIdentifier {
        vendor_id: VendorID(0x8087),
        product_id: ProductID(0x0a2a),
    }, // Intel 7265
    DeviceIdentifier {
        vendor_id: VendorID(0x8087),
        product_id: ProductID(0x0a2b),
    }, // Intel 8265
    DeviceIdentifier {
        vendor_id: VendorID(0x0489),
        product_id: ProductID(0xe07a),
    }, // Broadcom BCM20702A1
    DeviceIdentifier {
        vendor_id: VendorID(0x0a5c),
        product_id: ProductID(0x6412),
    }, // Broadcom BCM2045A0
    DeviceIdentifier {
        vendor_id: VendorID(0x050D),
        product_id: ProductID(0x065A),
    }, // Belkin BCM20702A0
];
