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
impl Device {
    /// Try openning the USB Device for communication
    pub fn open(&self) -> Result<Adapter, Error> {
        self.device
            .open()
            .map_err(Error::from)
            .and_then(Adapter::new)
    }
    pub fn new(device: rusb::Device<rusb::Context>) -> Device {
        Device { device }
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
