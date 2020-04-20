use crate::error::IOError;
use crate::hci::usb::adapter::Adapter;
use crate::hci::usb::Error;
use rusb::Context;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct DeviceIdentifier {
    pub vendor_id: u16,
    pub product_id: u16,
}
#[derive(Debug)]
pub struct Device {
    device: rusb::Device<rusb::Context>,
}
const WIRELESS_CONTROLLER_CLASS: u8 = 0xE0;
const SUBCLASS: u8 = 0x01;
const BLUETOOTH_PROGRAMMING_INTERFACE_PROTOCOL: u8 = 0x01;
impl Device {
    /// Try openning the USB Device for communication
    pub fn open(&self) -> Result<Adapter, Error> {
        if self.has_bluetooth_interface()? {
            self.device
                .open()
                .map_err(Error::from)
                .and_then(Adapter::from_handle)
        } else {
            Err(Error(IOError::NotImplemented))
        }
    }
    pub fn new(device: rusb::Device<rusb::Context>) -> Device {
        Device { device }
    }
    pub fn has_bluetooth_interface(&self) -> Result<bool, Error> {
        match self.device.active_config_descriptor() {
            Ok(config) => Ok(config
                .interfaces()
                .next()
                .and_then(|i| {
                    i.descriptors().next().map(|d| {
                        d.class_code() == WIRELESS_CONTROLLER_CLASS
                            && d.sub_class_code() == SUBCLASS
                            && d.protocol_code() == BLUETOOTH_PROGRAMMING_INTERFACE_PROTOCOL
                    })
                })
                .unwrap_or(false)),
            Err(rusb::Error::NotFound) => Ok(false),
            Err(e) => Err(Error::from(e)),
        }
    }
    pub fn rusb_device_mut(&mut self) -> &mut rusb::Device<rusb::Context> {
        &mut self.device
    }
    pub fn rusb_device(&self) -> &rusb::Device<rusb::Context> {
        &self.device
    }
}
impl From<Device> for rusb::Device<Context> {
    fn from(d: Device) -> Self {
        d.device
    }
}
pub struct DeviceList {
    device_list: rusb::DeviceList<rusb::Context>,
}
impl From<rusb::DeviceList<rusb::Context>> for DeviceList {
    fn from(device_list: rusb::DeviceList<rusb::Context>) -> Self {
        DeviceList { device_list }
    }
}
impl DeviceList {
    pub fn iter(&self) -> Devices<'_> {
        Devices {
            devices: self.device_list.iter(),
        }
    }
    pub fn bluetooth_adapters(&self) -> impl Iterator<Item = Result<Device, Error>> + '_ {
        self.iter()
            .filter_map(|d| match d.has_bluetooth_interface() {
                Ok(true) => Some(Ok(d)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
    }
}
pub struct Devices<'a> {
    devices: rusb::Devices<'a, rusb::Context>,
}
impl<'a> Iterator for Devices<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.devices.next().map(Device::new)
    }
}
