use crate::windows::ble::gatt::services::ServicesResult;
use crate::windows::WindowsError;
use crate::BTAddress;
use winrt_bluetooth_bindings::windows::devices::bluetooth::BluetoothLEDevice;

pub struct Device {
    device: BluetoothLEDevice,
}
impl Device {
    pub fn device_id(&self) -> Result<String, WindowsError> {
        Ok(self.device.device_id()?.into())
    }
    pub fn name(&self) -> Result<String, WindowsError> {
        Ok(self.device.name()?.into())
    }
    pub fn from_inner(device: BluetoothLEDevice) -> Self {
        Self { device }
    }
    pub async fn from_device_id(device_id: &str) -> Result<Self, WindowsError> {
        Ok(Self::from_inner(
            BluetoothLEDevice::from_id_async(device_id)?.await?,
        ))
    }
    pub async fn from_bluetooth_address(address: BTAddress) -> Result<Self, WindowsError> {
        // TODO: causes a null pointer some how?
        Ok(Self::from_inner(
            BluetoothLEDevice::from_bluetooth_address_async(address.to_u64())?.await?,
        ))
    }
    pub fn bluetooth_address(&self) -> Result<BTAddress, WindowsError> {
        let addr: u64 = self.device.bluetooth_address()?;
        Ok(BTAddress::from_u64(addr))
    }
    pub async fn gatt_services(&self) -> Result<ServicesResult, WindowsError> {
        Ok(ServicesResult::from_inner(
            self.device.get_gatt_services_async()?.await?,
        ))
    }
}
