use crate::hci::usb::device::{Device, DeviceIdentifier};
use crate::hci::usb::Error;
use driver_async::asyncs::time::Duration;

pub const HCI_COMMAND_ENDPOINT: u8 = 0x01;
pub const ACL_DATA_OUT_ENDPOINT: u8 = 0x02;
pub const HCI_EVENT_ENDPOINT: u8 = 0x81;
pub const ACL_DATA_IN_ENDPOINT: u8 = 0x82;

pub const INTERFACE_NUM: u8 = 0x00;

/// USB Bluetooth Adapter.
pub struct Adapter {
    pub handle: rusb::DeviceHandle<rusb::Context>,
    pub device_descriptor: rusb::DeviceDescriptor,
    _private: (),
}
impl core::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Adapter({:?})", self.device_descriptor)
    }
}
impl Adapter {
    pub fn from_handle(mut handle: rusb::DeviceHandle<rusb::Context>) -> Result<Adapter, Error> {
        handle.claim_interface(INTERFACE_NUM)?;
        Ok(Adapter::from_parts(
            handle.device().device_descriptor()?,
            handle,
        ))
    }
    pub(crate) fn from_parts(
        device_descriptor: rusb::DeviceDescriptor,
        handle: rusb::DeviceHandle<rusb::Context>,
    ) -> Adapter {
        Adapter {
            handle,
            _private: (),
            device_descriptor,
        }
    }
    /// Internal USB Device handle from `rusb`. Maybe change in the future if we use a different
    /// crate than `rusb`.
    pub fn rusb_handle_mut(&mut self) -> &mut rusb::DeviceHandle<rusb::Context> {
        &mut self.handle
    }
    pub fn rusb_handle(&self) -> &rusb::DeviceHandle<rusb::Context> {
        &self.handle
    }
    pub fn device_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            vendor_id: self.device_descriptor.vendor_id(),
            product_id: self.device_descriptor.product_id(),
        }
    }
    pub fn get_manufacturer_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.manufacturer_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn get_product_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.product_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn get_serial_number_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.manufacturer_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn write_hci_command_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        const TIMEOUT: Duration = Duration::from_secs(1);
        //TODO: Change from synchronous IO to Async IO.
        let mut index = 0;
        let size = bytes.len();
        while index < size {
            // bmRequestType = 0x20, bRequest = 0x00, wValue = 0x00, wIndex = 0x00 according to
            // Bluetooth Core Spec v5.2 Vol 4 Part B 2.2
            let amount = self
                .handle
                .write_control(0x20, 0, 0, 0, &bytes[index..], TIMEOUT)?;
            index += amount;
        }
        Ok(())
    }
    pub fn device(&self) -> Device {
        Device::new(self.handle.device())
    }
    pub fn reset(&mut self) -> Result<(), Error> {
        self.handle.reset()?;
        Ok(())
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        // We claim the interface when we make the adapter so we must release when we drop.
        let _ = self.handle.release_interface(INTERFACE_NUM);
    }
}
