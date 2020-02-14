use crate::advertisement::RawAdvertisement;
pub trait ScannerSink {
    fn consume_advertisement(&mut self, advertisement: &RawAdvertisement);
}
pub trait Scanner<Sink: ScannerSink> {
    fn take_sink(&mut self, sink: Sink);
}

pub enum AdvertiserError {}
pub trait Advertiser {
    fn advertise(&mut self, advertisement: &RawAdvertisement) -> Result<(), AdvertiserError>;
}
