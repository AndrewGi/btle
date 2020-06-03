use super::authentication;
use crate::le::att::Opcode;
use crate::uuid;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct Handle(u16);
impl Handle {
    pub const fn new(handle: u16) -> Handle {
        Handle(handle)
    }
    pub const fn inner(self) -> u16 {
        self.0
    }
}
impl From<Handle> for u16 {
    fn from(h: Handle) -> Self {
        h.0
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum TypeUUID {
    UUID128(uuid::UUID),
    UUID32(uuid::UUID32),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash)]
pub struct Value<B>(pub B);
impl<B> Value<B> {
    pub const fn new(bytes: B) -> Value<B> {
        Value(bytes)
    }
}
impl<B: AsRef<[u8]>> Value<B> {
    pub fn len(&self) -> usize {
        self.0.as_ref().len()
    }
}
impl<B: AsRef<[u8]>> AsRef<[u8]> for Value<B> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl<B: AsMut<[u8]>> AsMut<[u8]> for Value<B> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum PDUType {
    Command,
    Request,
    Response,
    Notification,
    Indication,
    Confirmation,
}
impl PDUType {
    pub fn suffix(self) -> &'static str {
        match self {
            PDUType::Command => "CMD",
            PDUType::Request => "REQ",
            PDUType::Response => "RSP",
            PDUType::Notification => "NTF",
            PDUType::Indication => "IND",
            PDUType::Confirmation => "CFM",
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct RawPDU<B> {
    pub opcode: Opcode,
    pub parameters: B,
    pub authentication_signature: Option<authentication::Signature>,
}
impl<B> RawPDU<B> {
    pub const fn new(
        opcode: Opcode,
        parameters: B,
        authentication_signature: Option<authentication::Signature>,
    ) -> RawPDU<B> {
        RawPDU {
            opcode,
            parameters,
            authentication_signature,
        }
    }
}
impl<B: AsRef<[u8]>> RawPDU<B> {
    pub fn byte_len(&self) -> usize {
        Opcode::BYTE_LEN + self.parameters.as_ref().len() + self.signature_len()
    }
    pub fn signature_len(&self) -> usize {
        self.authentication_signature
            .map_or(0, |_| authentication::SIGNATURE_LEN)
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct PackedPDU<B>(pub B);
