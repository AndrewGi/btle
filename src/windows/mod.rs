use crate::uuid::UUID;

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
