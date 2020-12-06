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
