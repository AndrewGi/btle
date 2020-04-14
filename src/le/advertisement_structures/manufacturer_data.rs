use crate::le::advertisement::{
    AdStructureType, AdType, ConstAdStructType, UnpackableAdStructType,
};
use crate::{CompanyID, PackError};
use driver_async::bytes::{Storage, ToFromBytesEndian};

#[derive(Copy, Clone, Debug)]
pub struct ManufacturerSpecificData<Buf> {
    pub company_id: CompanyID,
    pub data: Buf,
}
impl<Buf> ManufacturerSpecificData<Buf> {
    pub const AD_TYPE: AdType = AdType::ManufacturerData;
    pub fn new(company_id: CompanyID, data: Buf) -> Self {
        Self { company_id, data }
    }
}
impl<Buf: AsRef<[u8]>> AdStructureType for ManufacturerSpecificData<Buf> {
    fn ad_type(&self) -> AdType {
        Self::AD_TYPE
    }

    fn byte_len(&self) -> usize {
        self.data.as_ref().len() + CompanyID::byte_len()
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len(), buf)?;
        (&mut buf[..2]).copy_from_slice(&self.company_id.to_bytes_le()[..]);
        (&mut buf[2..]).copy_from_slice(self.data.as_ref());
        Ok(())
    }
}
impl<Buf: Storage<u8>> UnpackableAdStructType for ManufacturerSpecificData<Buf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if ad_type != Self::AD_TYPE {
            Err(PackError::InvalidFields)
        } else {
            if buf.len() < CompanyID::byte_len() {
                Err(PackError::BadLength {
                    expected: CompanyID::byte_len(),
                    got: buf.len(),
                })
            } else {
                let max_len = Buf::max_len();
                if buf.len() > max_len {
                    Err(PackError::BadLength {
                        expected: max_len,
                        got: buf.len(),
                    })
                } else {
                    Ok(Self::new(
                        CompanyID::from_bytes_le(&buf[..CompanyID::byte_len()])
                            .expect("company id length checked above"),
                        Buf::from_slice(&buf[CompanyID::byte_len()..]),
                    ))
                }
            }
        }
    }
}
impl<Buf: Storage<u8>> ConstAdStructType for ManufacturerSpecificData<Buf> {
    const AD_TYPE: AdType = AdType::ManufacturerData;
}
