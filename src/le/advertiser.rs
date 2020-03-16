//! Generic BLE Advertiser (WIP)
use crate::error::Error;
use crate::le::advertisement::OutgoingAdvertisement;
use crate::BoxFuture;
use core::pin::Pin;

use crate::le::adapter;
use futures_sink::Sink;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AdvertiserError {
    AdapterError(adapter::Error),
}
impl Error for AdvertiserError {}
pub type AdapterFutureResult<'a, T, E> = BoxFuture<'a, Result<T, E>>;

pub struct AdvertisementParameters {}
pub trait Advertiser: Sink<OutgoingAdvertisement, Error = AdvertiserError> {
    fn set_parameters<'a>(
        self: Pin<&'a mut Self>,
        advertisement_parameters: AdvertisementParameters,
    ) -> AdapterFutureResult<'a, (), AdvertiserError>;
}
