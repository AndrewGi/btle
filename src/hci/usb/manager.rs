use crate::hci::usb::adapter::Adapter;
use crate::hci::usb::{device, Error};
use rusb::UsbContext;

#[derive(Clone, Eq, PartialEq)]
pub struct Manager {
    context: rusb::Context,
}

impl Manager {
    pub fn new() -> Result<Manager, Error> {
        Ok(Manager {
            context: rusb::Context::new()?,
        })
    }
    /// Intern usb context used. Currently `rusb` but could change in the future for different
    /// platforms. For advance use.
    pub fn usb_context(&self) -> &rusb::Context {
        &self.context
    }
    /// Intern usb context used. Currently `rusb` but could change in the future for different
    /// platforms. For advance use.
    pub fn usb_context_mut(&mut self) -> &rusb::Context {
        &self.context
    }
    pub fn devices(&self) -> Result<device::DeviceList, Error> {
        self.context
            .devices()
            .map(device::DeviceList::from)
            .map_err(Error::from)
    }
    pub fn open_adapter(&self, vendor_id: u16, product_id: u16) -> Result<Option<Adapter>, Error> {
        self.context
            .open_device_with_vid_pid(vendor_id, product_id)
            .map(Adapter::new)
            .transpose()
    }
}
