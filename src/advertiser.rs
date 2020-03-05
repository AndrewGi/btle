use crate::advertisement::{IncomingAdvertisement, OutgoingAdvertisement};
use crate::BoxFuture;
use futures_core::Stream;
use futures_sink::Sink;
use std::fmt::Formatter;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AdvertiserError {}
impl core::fmt::Display for AdvertiserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", *self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for AdvertiserError {}
pub type AdapterFutureResult<'a, T> = BoxFuture<'a, Result<T, AdvertiserError>>;

pub struct AdvertisementParameters {}
pub trait Advertiser: Sink<OutgoingAdvertisement, Error = AdvertiserError> {
    fn set_parameters(
        &self,
        advertisement_parameters: AdvertisementParameters,
    ) -> AdapterFutureResult<()>;
}
pub trait Observer: Stream<Item = IncomingAdvertisement> {}
