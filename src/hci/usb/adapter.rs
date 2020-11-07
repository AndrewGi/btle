use crate::bytes::Storage;
use crate::error::IOError;
use crate::hci;
use crate::hci::command::CommandPacket;
use crate::hci::event::{EventCode, EventPacket, StaticHCIBuffer};
use crate::hci::usb::device::has_bluetooth_interface;
use crate::hci::usb::Error;
use core::convert::TryFrom;
use core::time::Duration;
use futures_util::future::LocalBoxFuture;
use usbw::device::DeviceIdentifier;
use usbw::libusb::async_device::AsyncDevice;
use usbw::libusb::device_descriptor::DeviceDescriptor;

/// Yield the task back to the executor. Just returns `Poll::Pending` once and calls
/// `.waker_by_ref()` to put the task back onto the queue. Workaround for blocking futures
pub async fn yield_now() {
    struct YieldNow {
        yielded: bool,
    }

    impl core::future::Future for YieldNow {
        type Output = ();

        fn poll(
            mut self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
        ) -> core::task::Poll<()> {
            if self.yielded {
                return core::task::Poll::Ready(());
            }

            self.yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    }

    YieldNow { yielded: false }.await
}

pub const HCI_COMMAND_ENDPOINT: u8 = 0x01;
pub const ACL_DATA_OUT_ENDPOINT: u8 = 0x02;
pub const HCI_EVENT_ENDPOINT: u8 = 0x81;
pub const ACL_DATA_IN_ENDPOINT: u8 = 0x82;
pub const HCI_COMMAND_REQUEST_TYPE: u8 = 0x20;
pub const INTERFACE_NUM: u8 = 0x00;

/// USB Bluetooth HCI Adapter.
/// # WARNING
/// Currently using the `rusb` crate (a `libusb` wrapper) for USB communication but it !!DOES NOT!!
/// implement async/await yet. All read/write function will be blocking!
/// TODO: Add asynchronous USB support
pub struct Adapter {
    handle: AsyncDevice,
    device_descriptor: DeviceDescriptor,
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
    pub fn open(device_handle: AsyncDevice) -> Result<Adapter, Error> {
        if has_bluetooth_interface(&device_handle.handle_ref().device())? {
            Self::from_handle(device_handle)
        } else {
            Err(Error(IOError::NotImplemented))
        }
    }
    pub fn from_handle(mut handle: AsyncDevice) -> Result<Adapter, Error> {
        handle.handle_mut().reset()?;
        handle.handle_mut().claim_interface(INTERFACE_NUM)?;
        Ok(Adapter::from_parts(
            handle.device().device_descriptor()?,
            handle,
        ))
    }
    pub(crate) fn from_parts(device_descriptor: DeviceDescriptor, handle: AsyncDevice) -> Adapter {
        Adapter {
            handle,
            _private: (),
            device_descriptor,
        }
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
            Some(index) => Ok(Some(
                self.handle
                    .handle_ref()
                    .read_string_descriptor_ascii(index)?,
            )),
            None => Ok(None),
        }
    }
    pub fn get_product_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.product_string_index() {
            Some(index) => Ok(Some(
                self.handle
                    .handle_ref()
                    .read_string_descriptor_ascii(index)?,
            )),
            None => Ok(None),
        }
    }
    pub fn get_serial_number_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.manufacturer_string_index() {
            Some(index) => Ok(Some(
                self.handle
                    .handle_ref()
                    .read_string_descriptor_ascii(index)?,
            )),
            None => Ok(None),
        }
    }
    pub async fn write_hci_command_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let mut index = 0;
        let size = bytes.len();
        // TODO: Fix probably infinite loop
        while index < size {
            // bmRequestType = 0x20, bRequest = 0x00, wValue = 0x00, wIndex = 0x00 according to
            // Bluetooth Core Spec v5.2 Vol 4 Part B 2.2
            let amount = self
                .handle
                .control_write(0x20, 0, 0, 0, &bytes[index..], Self::TIMEOUT)
                .await?;
            if amount == 0 {
                return Err(Error(IOError::TimedOut));
            }
            index += amount;
        }
        Ok(())
    }
    pub async fn read_some_event_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .interrupt_read(HCI_EVENT_ENDPOINT, buf, Self::TIMEOUT)
            .await?)
    }
    pub async fn read_some_acl_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .bulk_read(ACL_DATA_IN_ENDPOINT, buf, Self::TIMEOUT)
            .await?)
    }
    pub async fn read_event_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let mut index = 0;
        let size = buf.len();
        // TODO: Fix probably infinite loop
        while index < size {
            let amount = match self.read_some_event_bytes(&mut buf[index..]).await {
                Ok(a) => a,
                Err(Error(IOError::TimedOut)) => 0,
                Err(e) => return Err(e),
            };
            index += amount;
        }
        Ok(())
    }
    pub async fn read_event_packet<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, hci::adapter::Error> {
        let mut header = [0u8; 2];
        self.read_event_bytes(&mut header[..]).await?;
        let len = header[1];
        let mut buf = Buf::with_size(len.into());
        // Even if the event code is wrong, still read so we don't leave data in buffer
        self.read_event_bytes(buf.as_mut()).await?;
        let event_code =
            EventCode::try_from(header[0]).map_err(|_| hci::StreamError::BadEventCode)?;
        Ok(EventPacket {
            event_code,
            parameters: buf,
        })
    }
    pub fn reset(&mut self) -> Result<(), Error> {
        self.handle.handle_ref().reset()?;
        Ok(())
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        // We claim the interface when we make the adapter so we must release when we drop.
        let _ = self.handle.handle_mut().release_interface(INTERFACE_NUM);
    }
}

impl hci::adapter::Adapter for Adapter {
    fn write_command<'s, 'p: 's>(
        &'s mut self,
        packet: CommandPacket<&'p [u8]>,
    ) -> LocalBoxFuture<'s, Result<(), hci::adapter::Error>> {
        let packed = packet.to_raw_packet::<StaticHCIBuffer>();
        Box::pin(async move {
            self.write_hci_command_bytes(packed.buf.as_ref())
                .await
                .map_err(hci::adapter::Error::from)
        })
    }

    fn read_event<'s, 'p: 's, S: Storage<u8> + 'p>(
        &'s mut self,
    ) -> LocalBoxFuture<'s, Result<EventPacket<S>, hci::adapter::Error>> {
        Box::pin(async move {
            self.read_event_packet()
                .await
                .map_err(hci::adapter::Error::from)
        })
    }
}
