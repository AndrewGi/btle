use crate::hci;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum Error {
    BadParameter,
    StreamError(hci::StreamError),
    ErrorCode(hci::ErrorCode),
}
