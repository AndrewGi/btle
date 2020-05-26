use crate::bytes::Storage;
use crate::le::advertisement::{
    AdStructureType, AdType, ConstAdStructType, UnpackableAdStructType,
};
use crate::PackError;
use core::str::Utf8Error;
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub enum LocalName<Buf> {
    Shortened(ShortenedLocalName<Buf>),
    Complete(CompleteLocalName<Buf>),
}
impl<Buf: AsRef<[u8]>> AdStructureType for LocalName<Buf> {
    fn ad_type(&self) -> AdType {
        match self {
            LocalName::Shortened(_) => ShortenedLocalName::<Buf>::AD_TYPE,
            LocalName::Complete(_) => CompleteLocalName::<Buf>::AD_TYPE,
        }
    }

    fn byte_len(&self) -> usize {
        match self {
            LocalName::Shortened(s) => s.byte_len(),
            LocalName::Complete(c) => c.byte_len(),
        }
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        match self {
            LocalName::Shortened(s) => s.pack_into(buf),
            LocalName::Complete(c) => c.pack_into(buf),
        }
    }
}
impl<Buf: Storage<u8>> UnpackableAdStructType for LocalName<Buf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        match ad_type {
            AdType::ShortenLocalName => Ok(LocalName::Shortened(ShortenedLocalName::unpack_from(
                ad_type, buf,
            )?)),
            AdType::CompleteLocalName => Ok(LocalName::Complete(CompleteLocalName::unpack_from(
                ad_type, buf,
            )?)),
            _ => Err(PackError::BadOpcode),
        }
    }
}
