use crate::hci::usb::Error;
use usbw::libusb::device::Device;

const WIRELESS_CONTROLLER_CLASS: u8 = 0xE0;
const SUBCLASS: u8 = 0x01;
const BLUETOOTH_PROGRAMMING_INTERFACE_PROTOCOL: u8 = 0x01;
pub fn has_bluetooth_interface(device: &Device) -> Result<bool, Error> {
    match device.active_config_descriptor() {
        Ok(config) => Ok(config
            .interfaces()
            .iter()
            .next()
            .and_then(|i| {
                i.descriptors().iter().next().map(|d| {
                    d.class_code() == WIRELESS_CONTROLLER_CLASS
                        && d.sub_class_code() == SUBCLASS
                        && d.protocol_code() == BLUETOOTH_PROGRAMMING_INTERFACE_PROTOCOL
                })
            })
            .unwrap_or(false)),
        Err(usbw::libusb::error::Error::NotFound) => Ok(false),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn bluetooth_adapters<'a>(
    i: impl Iterator<Item = Device> + 'a,
) -> impl Iterator<Item = Result<Device, Error>> + 'a {
    i.filter_map(|d| match has_bluetooth_interface(&d) {
        Ok(true) => Some(Ok(d)),
        Ok(false) => None,
        Err(e) => Some(Err(e)),
    })
}
