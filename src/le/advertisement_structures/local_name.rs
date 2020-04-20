use crate::bytes::Storage;
use crate::le::advertisement::{
    AdStructureType, AdType, ConstAdStructType, UnpackableAdStructType,
};
use crate::PackError;
use core::str::Utf8Error;

pub struct ShortenedLocalName<Buf> {
    pub name: Buf,
}
impl<Buf> ShortenedLocalName<Buf> {
    pub const AD_TYPE: AdType = AdType::ShortenLocalName;
    pub fn new(name: Buf) -> Self {
        ShortenedLocalName { name }
    }
}
impl<Buf: AsRef<[u8]>> ShortenedLocalName<Buf> {
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.name.as_ref())
    }
}
impl<Buf: AsRef<[u8]>> AdStructureType for ShortenedLocalName<Buf> {
    fn ad_type(&self) -> AdType {
        Self::AD_TYPE
    }

    fn byte_len(&self) -> usize {
        self.name.as_ref().len()
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len(), buf)?;
        buf.copy_from_slice(self.name.as_ref());
        Ok(())
    }
}
impl<Buf: Storage<u8>> UnpackableAdStructType for ShortenedLocalName<Buf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if ad_type != Self::AD_TYPE {
            Err(PackError::InvalidFields)
        } else {
            let max_len = Buf::max_len();
            if buf.len() > max_len {
                Err(PackError::BadLength {
                    expected: max_len,
                    got: buf.len(),
                })
            } else {
                Ok(Self::new(Buf::from_slice(buf)))
            }
        }
    }
}
impl<Buf: Storage<u8>> ConstAdStructType for ShortenedLocalName<Buf> {
    const AD_TYPE: AdType = AdType::ShortenLocalName;
}
pub struct CompleteLocalName<Buf> {
    pub name: Buf,
}
impl<Buf> CompleteLocalName<Buf> {
    pub const AD_TYPE: AdType = AdType::CompleteLocalName;
    pub fn new(name: Buf) -> Self {
        CompleteLocalName { name }
    }
}
impl<Buf: AsRef<[u8]>> CompleteLocalName<Buf> {
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.name.as_ref())
    }
}
impl<Buf: AsRef<[u8]>> AdStructureType for CompleteLocalName<Buf> {
    fn ad_type(&self) -> AdType {
        Self::AD_TYPE
    }

    fn byte_len(&self) -> usize {
        self.name.as_ref().len()
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len(), buf)?;
        buf.copy_from_slice(self.name.as_ref());
        Ok(())
    }
}
impl<Buf: Storage<u8>> UnpackableAdStructType for CompleteLocalName<Buf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if ad_type != Self::AD_TYPE {
            Err(PackError::InvalidFields)
        } else {
            let max_len = Buf::max_len();
            if buf.len() > max_len {
                Err(PackError::BadLength {
                    expected: max_len,
                    got: buf.len(),
                })
            } else {
                Ok(Self::new(Buf::from_slice(buf)))
            }
        }
    }
}
impl<Buf: Storage<u8>> ConstAdStructType for CompleteLocalName<Buf> {
    const AD_TYPE: AdType = AdType::CompleteLocalName;
}
pub enum LocalName<Buf> {
    Shortened(ShortenedLocalName<Buf>),
    Complete(CompleteLocalName<Buf>),
}
