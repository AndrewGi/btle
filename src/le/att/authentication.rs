pub const SIGNATURE_LEN: usize = 12;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct Signature(pub [u8; SIGNATURE_LEN]);
impl Signature {
    pub const ZEROED: Signature = Signature([0_u8; SIGNATURE_LEN]);
    pub const BYTE_LEN: usize = SIGNATURE_LEN;
}
impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}
