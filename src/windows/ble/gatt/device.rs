use crate::windows::ble::gatt::service::ServicesResult;
use crate::windows::WindowsError;
use crate::BTAddress;
use windows::Devices::Bluetooth::BluetoothLEDevice;

pub struct Device {
    device: BluetoothLEDevice,
}
impl Device {
    pub fn device_id(&self) -> Result<String, WindowsError> {
        Ok(self.device.DeviceId()?.to_string_lossy())
    }
    pub fn name(&self) -> Result<String, WindowsError> {
        Ok(self.device.Name()?.to_string_lossy())
    }
    pub fn from_inner(device: BluetoothLEDevice) -> Self {
        Self { device }
    }
    pub async fn from_device_id(device_id: &str) -> Result<Self, WindowsError> {
        Ok(Self::from_inner(
            BluetoothLEDevice::FromIdAsync(&windows::core::HSTRING::from(device_id))?.await?,
        ))
    }
    pub async fn from_bluetooth_address(address: BTAddress) -> Result<Self, WindowsError> {
        // TODO: causes a null pointer some how?
        Ok(Self::from_inner(
            BluetoothLEDevice::FromBluetoothAddressAsync(address.to_u64())?.await?,
        ))
    }
    pub fn bluetooth_address(&self) -> Result<BTAddress, WindowsError> {
        let addr: u64 = self.device.BluetoothAddress()?;
        Ok(BTAddress::from_u64(addr))
    }
    pub async fn gatt_services(&self) -> Result<ServicesResult, WindowsError> {
        Ok(ServicesResult::from_inner(
            self.device.GetGattServicesAsync()?.await?,
        ))
    }
}
