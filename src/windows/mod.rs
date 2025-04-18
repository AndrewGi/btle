use crate::uuid::UUID;
use windows::Devices::Bluetooth;
pub mod ble;
#[derive(Debug)]
pub struct WindowsError(pub windows::core::Error);
impl From<windows::core::Error> for WindowsError {
    fn from(e: windows::core::Error) -> Self {
        WindowsError(e)
    }
}
impl std::fmt::Display for WindowsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08x}: {}", self.0.code().0, self.0.message())
    }
}
impl std::error::Error for WindowsError {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BluetoothError {
    Success,
    RadioNotAvailable,
    ResourceInUse,
    DeviceNotConnected,
    OtherError,
    DisabledByPolicy,
    NotSupported,
    DisabledByUser,
    ConsentRequired,
    TransportNotSupported,
}
impl From<BluetoothError> for WindowsError {
    fn from(e: BluetoothError) -> Self {
        WindowsError(windows::core::Error::new(
            windows::core::HRESULT(0x77370000),
            format!("bluetooth error: {:?}", e).as_str(),
        ))
    }
}
impl From<Bluetooth::BluetoothError> for BluetoothError {
    fn from(e: Bluetooth::BluetoothError) -> Self {
        match e {
            Bluetooth::BluetoothError::Success => Self::Success,
            Bluetooth::BluetoothError::RadioNotAvailable => Self::RadioNotAvailable,
            Bluetooth::BluetoothError::ResourceInUse => Self::ResourceInUse,
            Bluetooth::BluetoothError::DeviceNotConnected => Self::DeviceNotConnected,
            Bluetooth::BluetoothError::OtherError => Self::OtherError,
            Bluetooth::BluetoothError::DisabledByPolicy => Self::DisabledByPolicy,
            Bluetooth::BluetoothError::NotSupported => Self::NotSupported,
            Bluetooth::BluetoothError::DisabledByUser => Self::DisabledByUser,
            Bluetooth::BluetoothError::ConsentRequired => Self::ConsentRequired,
            Bluetooth::BluetoothError::TransportNotSupported => Self::TransportNotSupported,
            _ => panic!("invalid windows BluetoothError"),
        }
    }
}
impl From<BluetoothError> for Bluetooth::BluetoothError {
    fn from(e: BluetoothError) -> Self {
        match e {
            BluetoothError::Success => Bluetooth::BluetoothError::Success,
            BluetoothError::RadioNotAvailable => Bluetooth::BluetoothError::RadioNotAvailable,
            BluetoothError::ResourceInUse => Bluetooth::BluetoothError::ResourceInUse,
            BluetoothError::DeviceNotConnected => Bluetooth::BluetoothError::DeviceNotConnected,
            BluetoothError::OtherError => Bluetooth::BluetoothError::OtherError,
            BluetoothError::DisabledByPolicy => Bluetooth::BluetoothError::DisabledByPolicy,
            BluetoothError::NotSupported => Bluetooth::BluetoothError::NotSupported,
            BluetoothError::DisabledByUser => Bluetooth::BluetoothError::DisabledByUser,
            BluetoothError::ConsentRequired => Bluetooth::BluetoothError::ConsentRequired,
            BluetoothError::TransportNotSupported => {
                Bluetooth::BluetoothError::TransportNotSupported
            }
        }
    }
}
impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for BluetoothError {}
pub fn uuid_to_guid(uuid: &UUID) -> windows::core::GUID {
    // SAFETY:
    // The struct implementation is copy and pasted from `windows::Guid`.
    // This transmute is needed to get access to the inner data. According to the C++/windows docs,
    // this is expected for going between `GUID` and `windows::Guid`
    unsafe { std::mem::transmute_copy(uuid) }
}
pub fn guid_to_uuid(guid: &windows::core::GUID) -> UUID {
    // SAFETY:
    // The struct implementation is copy and pasted from `windows::Guid`.
    // This transmute is needed to get access to the inner data. According to the C++/windows docs,
    // this is expected for going between `GUID` and `windows::Guid`
    unsafe { std::mem::transmute_copy(guid) }
}
