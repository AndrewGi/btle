use crate::uuid::UUID;
use winrt_bluetooth_bindings::windows::devices::bluetooth;
pub mod ble;
#[derive(Debug)]
pub struct WindowsError(pub winrt::Error);
impl From<winrt::Error> for WindowsError {
    fn from(e: winrt::Error) -> Self {
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
        WindowsError(winrt::Error::new(
            winrt::ErrorCode(0x77370000),
            format!("bluetooth error: {:?}", e).as_str(),
        ))
    }
}
impl From<bluetooth::BluetoothError> for BluetoothError {
    fn from(e: bluetooth::BluetoothError) -> Self {
        match e {
            bluetooth::BluetoothError::Success => Self::Success,
            bluetooth::BluetoothError::RadioNotAvailable => Self::RadioNotAvailable,
            bluetooth::BluetoothError::ResourceInUse => Self::ResourceInUse,
            bluetooth::BluetoothError::DeviceNotConnected => Self::DeviceNotConnected,
            bluetooth::BluetoothError::OtherError => Self::OtherError,
            bluetooth::BluetoothError::DisabledByPolicy => Self::DisabledByPolicy,
            bluetooth::BluetoothError::NotSupported => Self::NotSupported,
            bluetooth::BluetoothError::DisabledByUser => Self::DisabledByUser,
            bluetooth::BluetoothError::ConsentRequired => Self::ConsentRequired,
            bluetooth::BluetoothError::TransportNotSupported => Self::TransportNotSupported,
            _ => panic!("invalid windows BluetoothError"),
        }
    }
}
impl From<BluetoothError> for bluetooth::BluetoothError {
    fn from(e: BluetoothError) -> Self {
        match e {
            BluetoothError::Success => bluetooth::BluetoothError::Success,
            BluetoothError::RadioNotAvailable => bluetooth::BluetoothError::RadioNotAvailable,
            BluetoothError::ResourceInUse => bluetooth::BluetoothError::ResourceInUse,
            BluetoothError::DeviceNotConnected => bluetooth::BluetoothError::DeviceNotConnected,
            BluetoothError::OtherError => bluetooth::BluetoothError::OtherError,
            BluetoothError::DisabledByPolicy => bluetooth::BluetoothError::DisabledByPolicy,
            BluetoothError::NotSupported => bluetooth::BluetoothError::NotSupported,
            BluetoothError::DisabledByUser => bluetooth::BluetoothError::DisabledByUser,
            BluetoothError::ConsentRequired => bluetooth::BluetoothError::ConsentRequired,
            BluetoothError::TransportNotSupported => {
                bluetooth::BluetoothError::TransportNotSupported
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
pub fn uuid_to_guid(uuid: &UUID) -> winrt::Guid {
    // SAFETY:
    // The struct implementation is copy and pasted from `winrt::Guid`.
    // This transmute is needed to get access to the inner data. According to the C++/WinRT docs,
    // this is expected for going between `GUID` and `winrt::Guid`
    unsafe { std::mem::transmute_copy(uuid) }
}
pub fn guid_to_uuid(guid: &winrt::Guid) -> UUID {
    // SAFETY:
    // The struct implementation is copy and pasted from `winrt::Guid`.
    // This transmute is needed to get access to the inner data. According to the C++/WinRT docs,
    // this is expected for going between `GUID` and `winrt::Guid`
    unsafe { std::mem::transmute_copy(guid) }
}
