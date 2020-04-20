use crate::hci;
use crate::hci::command::Command;
use crate::hci::event::{Event, EventCode, EventPacket, ReturnParameters};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::stream::HCI_EVENT_READ_TRIES;
use crate::hci::usb::device::{Device, DeviceIdentifier};
use crate::hci::usb::Error;
use crate::hci::{Opcode, StreamError};
use driver_async::asyncs::time::Duration;
use driver_async::bytes::Storage;
use driver_async::error::IOError;
use hci::event::CommandComplete;
use std::convert::TryFrom;

pub const HCI_COMMAND_ENDPOINT: u8 = 0x01;
pub const ACL_DATA_OUT_ENDPOINT: u8 = 0x02;
pub const HCI_EVENT_ENDPOINT: u8 = 0x81;
pub const ACL_DATA_IN_ENDPOINT: u8 = 0x82;

pub const INTERFACE_NUM: u8 = 0x00;

/// USB Bluetooth Adapter.
pub struct Adapter {
    handle: rusb::DeviceHandle<rusb::Context>,
    device_descriptor: rusb::DeviceDescriptor,
    _private: (),
}
impl core::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Adapter({:?})", self.device_descriptor)
    }
}
impl Adapter {
    /// Timeout for USB transfers (1s). If expired, it'll return `IOError::TimedOut`. Hoping to just
    /// be a temporary solution until I get Async USB working.
    pub const TIMEOUT: Duration = Duration::from_secs(1);
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
        //TODO: Change from synchronous IO to Async IO.
        let mut index = 0;
        let size = bytes.len();
        // TODO: Fix probably infinite loop
        while index < size {
            // bmRequestType = 0x20, bRequest = 0x00, wValue = 0x00, wIndex = 0x00 according to
            // Bluetooth Core Spec v5.2 Vol 4 Part B 2.2
            let amount =
                self.handle
                    .write_control(0x20, 0, 0, 0, &bytes[index..], Self::TIMEOUT)?;
            if amount == 0 {
                return Err(Error(IOError::TimedOut));
            }
            index += amount;
        }
        Ok(())
    }
    pub fn read_some_event_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .read_interrupt(HCI_EVENT_ENDPOINT, buf, Self::TIMEOUT)?)
    }
    pub fn read_some_acl_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .read_bulk(ACL_DATA_IN_ENDPOINT, buf, Self::TIMEOUT)?)
    }
    pub fn read_event_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        // TODO: Change from synchronous IO to Async IO.
        let mut index = 0;
        let size = buf.len();
        // TODO: Fix probably infinite loop
        while index < size {
            let amount = self.read_some_event_bytes(&mut buf[index..])?;
            index += amount;
        }
        Ok(())
    }
    pub fn read_event_packet<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, hci::adapter::Error> {
        let mut header = [0u8; 2];
        self.read_event_bytes(&mut header[..])?;
        let event_code =
            EventCode::try_from(header[0]).map_err(|_| hci::StreamError::BadEventCode)?;
        let len = header[1];
        let mut buf = Buf::with_size(len.into());
        self.read_event_bytes(buf.as_mut())?;
        Ok(EventPacket {
            event_code,
            parameters: buf,
        })
    }
    fn send_hci_command<Cmd: Command>(
        &mut self,
        command: Cmd,
    ) -> Result<Cmd::Return, hci::adapter::Error> {
        const BUF_LEN: usize = 0xFF + 2 + 1;
        let mut buf = [0_u8; BUF_LEN];
        // Pack Command
        let len = command
            .pack_full(&mut buf[..])
            .map_err(StreamError::CommandError)?;
        self.write_hci_command_bytes(&buf[..len])?;
        for _try_i in 0..HCI_EVENT_READ_TRIES {
            // Reuse `buf` to read the RawPacket
            let mut header = [0u8; 2];
            self.read_event_bytes(&mut header[..])?;
            let event_code =
                EventCode::try_from(header[0]).map_err(|_| hci::StreamError::BadEventCode)?;
            let len = usize::from(header[1]);
            self.read_event_bytes(buf[..len].as_mut())?;
            let event = EventPacket::new(event_code, &buf[..len]);
            if event.event_code() == CommandComplete::<Cmd::Return>::CODE {
                if Opcode::unpack(&event.parameters().as_ref()[1..3])
                    .map_err(|_| StreamError::BadOpcode)?
                    == Cmd::opcode()
                {
                    return Ok(Cmd::Return::unpack_from(event.parameters())
                        .map_err(StreamError::CommandError)?);
                }
            }
        }
        Err(hci::adapter::Error::StreamError(StreamError::StreamFailed))
    }
    pub fn write_packet(&mut self, packet: RawPacket<&[u8]>) -> Result<(), Error> {
        // TODO: change this API to safer error handling
        match packet.packet_type {
            PacketType::Command => self.write_hci_command_bytes(packet.buf),
            PacketType::ACLData => unimplemented!(),
            PacketType::SCOData => unimplemented!(),
            PacketType::Event => panic!("can't write an event packet"),
            PacketType::Vendor => unimplemented!(),
        }
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
