use crate::windows::WindowsError;
use winrt_bluetooth_bindings::windows::devices::bluetooth::BluetoothLEDevice;

pub struct Device {
    device: BluetoothLEDevice,
}
impl Device {
    pub fn from_inner(device: BluetoothLEDevice) -> Self {
        Self { device }
    }
    pub async fn from_id(device_id: &str) -> Result<Self, WindowsError> {
        Ok(Self::from_inner(
            BluetoothLEDevice::from_id_async(device_id)?.await?,
        ))
    }
}
