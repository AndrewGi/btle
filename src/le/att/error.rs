use crate::ConversionError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ApplicationError(u8);
impl ApplicationError {
    pub const MAX_U8: u8 = 0x9F;
    pub const MAX: ApplicationError = ApplicationError(Self::MAX_U8);
    pub const MIN_U8: u8 = 0x80;
    pub const MIN: ApplicationError = ApplicationError(Self::MIN_U8);
    /// Creates a new `ApplicationError`.
    /// # Panics
    /// Panics if `error_code` isn't between the bounds of [`ApplicationError::MIN_U8'] (`0x80`) and
    /// [`ApplicationError::MAX_U8`] (`0x9F`)
    pub fn new(error_code: u8) -> ApplicationError {
        assert!(
            error_code >= Self::MIN_U8 && error_code <= Self::MAX_U8,
            "error_code `{}` out of bounds",
            error_code
        );
        ApplicationError(error_code)
    }
    /// Creates a new `ApplicationError`.
    /// If `error_code` isn't between the bounds of [`ApplicationError::MIN_U8'] (`0x80`) and
    /// [`ApplicationError::MAX_U8`] (`0x9F`), `None` will be returned.
    pub fn new_checked(error_code: u8) -> Option<ApplicationError> {
        if error_code >= Self::MIN_U8 && error_code <= Self::MAX_U8 {
            Some(ApplicationError(error_code))
        } else {
            None
        }
    }
    pub const fn inner(self) -> u8 {
        self.0
    }
}
impl From<ApplicationError> for u8 {
    fn from(a: ApplicationError) -> Self {
        a.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct CommonProfileServicesError(u8);
impl CommonProfileServicesError {
    pub const MAX_U8: u8 = 0xFF;
    pub const MAX: CommonProfileServicesError = CommonProfileServicesError(Self::MAX_U8);
    pub const MIN_U8: u8 = 0xE0;
    pub const MIN: CommonProfileServicesError = CommonProfileServicesError(Self::MIN_U8);
    /// Creates a new `CommonProfileServicesError`.
    /// # Panics
    /// Panics if `error_code` isn't between the bounds of [`CommonProfileServicesError::MIN_U8'] (`0xE0`) and
    /// [`CommonProfileServicesError::MAX_U8`] (`0xFF`)
    pub fn new(error_code: u8) -> CommonProfileServicesError {
        assert!(
            error_code >= Self::MIN_U8 && error_code <= Self::MAX_U8,
            "error_code `{}` out of bounds",
            error_code
        );
        CommonProfileServicesError(error_code)
    }
    /// Creates a new `CommonProfileServicesError`.
    /// If `error_code` isn't between the bounds of [`CommonProfileServicesError::MIN_U8'] (`0xE0`) and
    /// [`CommonProfileServicesError::MAX_U8`] (`0xFF`), `None` will be returned.
    pub fn new_checked(error_code: u8) -> Option<CommonProfileServicesError> {
        if error_code >= Self::MIN_U8 && error_code <= Self::MAX_U8 {
            Some(CommonProfileServicesError(error_code))
        } else {
            None
        }
    }
    pub const fn inner(self) -> u8 {
        self.0
    }
}
impl From<CommonProfileServicesError> for u8 {
    fn from(c: CommonProfileServicesError) -> Self {
        c.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Code {
    InvalidHandle,
    ReadNotPermitted,
    WriteNotPermitted,
    InvalidPDU,
    InsufficientAuthentication,
    RequestNotSupported,
    InvalidOffset,
    InsufficientAuthorization,
    PrepareQueueFull,
    AttributeNotFound,
    AttributeNotLong,
    InsufficientEncryptionKeySize,
    InvalidAttributeValueLength,
    UnlikelyError,
    InsufficientEncryption,
    UnsupportedGroupType,
    InsufficientResources,
    DatabaseOutOfSync,
    ValueNotAllowed,
    ApplicationError(ApplicationError),
    CommonProfileServicesError(CommonProfileServicesError),
}
impl From<Code> for u8 {
    fn from(c: Code) -> Self {
        match c {
            Code::InvalidHandle => 0x01,
            Code::ReadNotPermitted => 0x02,
            Code::WriteNotPermitted => 0x03,
            Code::InvalidPDU => 0x04,
            Code::InsufficientAuthentication => 0x05,
            Code::RequestNotSupported => 0x06,
            Code::InvalidOffset => 0x07,
            Code::InsufficientAuthorization => 0x08,
            Code::PrepareQueueFull => 0x09,
            Code::AttributeNotFound => 0x0A,
            Code::AttributeNotLong => 0x0B,
            Code::InsufficientEncryptionKeySize => 0x0C,
            Code::InvalidAttributeValueLength => 0x0D,
            Code::UnlikelyError => 0x0E,
            Code::InsufficientEncryption => 0x0F,
            Code::UnsupportedGroupType => 0x10,
            Code::InsufficientResources => 0x11,
            Code::DatabaseOutOfSync => 0x12,
            Code::ValueNotAllowed => 0x13,
            Code::ApplicationError(a) => a.inner(),
            Code::CommonProfileServicesError(c) => c.inner(),
        }
    }
}
impl core::convert::TryFrom<u8> for Code {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Code, ConversionError> {
        match value {
            0x01 => Ok(Code::InvalidHandle),
            0x02 => Ok(Code::ReadNotPermitted),
            0x03 => Ok(Code::WriteNotPermitted),
            0x04 => Ok(Code::InvalidPDU),
            0x05 => Ok(Code::InsufficientAuthentication),
            0x06 => Ok(Code::RequestNotSupported),
            0x07 => Ok(Code::InvalidOffset),
            0x08 => Ok(Code::InsufficientAuthorization),
            0x09 => Ok(Code::PrepareQueueFull),
            0x0A => Ok(Code::AttributeNotFound),
            0x0B => Ok(Code::AttributeNotLong),
            0x0C => Ok(Code::InsufficientEncryptionKeySize),
            0x0D => Ok(Code::InvalidAttributeValueLength),
            0x0E => Ok(Code::UnlikelyError),
            0x0F => Ok(Code::InsufficientEncryption),
            0x10 => Ok(Code::UnsupportedGroupType),
            0x11 => Ok(Code::InsufficientResources),
            0x12 => Ok(Code::DatabaseOutOfSync),
            0x13 => Ok(Code::ValueNotAllowed),
            ApplicationError::MIN_U8..=ApplicationError::MAX_U8 => {
                Ok(Code::ApplicationError(ApplicationError(value)))
            }
            CommonProfileServicesError::MIN_U8..=CommonProfileServicesError::MAX_U8 => Ok(
                Code::CommonProfileServicesError(CommonProfileServicesError(value)),
            ),
            _ => Err(ConversionError(())),
        }
    }
}
