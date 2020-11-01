use btle::error::IOError;
use btle::le;
use btle::le::advertisement::{AdStructureType, StaticAdvBuffer};
use btle::le::advertisement_structures::local_name::CompleteLocalName;
use btle::le::advertiser::AdvertisingInterval;
#[allow(unused_imports)]
use core::convert::{TryFrom, TryInto};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(async move {
        #[cfg(unix)]
        dump_bluez(
            std::env::args()
                .skip(1)
                .next()
                .unwrap_or("0".to_owned())
                .parse()
                .expect("invalid adapter id"),
        )
        .await;
        #[cfg(feature = "hci_usb")]
        dump_usb().await?;
        #[cfg(not(unix))]
        dump_not_supported()?;
        Ok(())
    })
}

pub fn dump_not_supported() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("no known support adapter for this platform. (When this example was written)");
    Ok(())
}
#[cfg(unix)]
pub fn dump_bluez(adapter_id: u16) -> Result<(), Box<dyn std::error::Error>> {
    use btle::error::StdError;
    let manager = btle::hci::socket::Manager::new().map_err(StdError)?;
    let socket = match manager.get_adapter_socket(btle::hci::socket::AdapterID(adapter_id)) {
        Ok(socket) => socket,
        Err(btle::hci::socket::HCISocketError::PermissionDenied) => {
            eprintln!("Permission denied error when opening the HCI socket. Maybe run as sudo?");
            return Err(btle::hci::socket::HCISocketError::PermissionDenied)
                .map_err(StdError)
                .map_err(Into::into);
        }
        Err(e) => return Err(StdError(e).into()),
    };

    let async_socket = btle::hci::socket::AsyncHCISocket::try_from(socket)?;
    let stream = btle::hci::stream::Stream::new(async_socket);
    let adapter = btle::hci::adapters::Adapter::new(stream);
    dump_adapter(adapter)
        .await
        .map_err(|e| Box::new(btle::error::StdError(e)))?;
    Result::<(), Box<dyn std::error::Error>>::Ok(())
}
#[cfg(feature = "hci_usb")]
pub async fn dump_usb() -> Result<(), btle::hci::adapter::Error> {
    use btle::hci::usb;
    println!("opening first device...");
    let device: usbw::libusb::device::Device = usb::device::bluetooth_adapters(
        usbw::libusb::context::Context::default()
            .map_err(btle::hci::usb::Error::from)?
            .device_list()
            .iter(),
    )
    .next()
    .ok_or(IOError::NotFound)??;
    println!("using {:?}", device);
    let mut adapter = usb::adapter::Adapter::open(device)?;
    adapter.reset()?;
    dump_adapter(adapter).await
}
/// Block and wait for an enter press on `StdIn`. Similar to C++ `std::cin.ignore()`.
pub fn cin_ignore() {
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("cin_ignore failed");
}
pub async fn dump_adapter<A: btle::hci::adapter::Adapter>(
    adapter: A,
) -> Result<(), btle::hci::adapter::Error> {
    let adapter = btle::hci::adapters::Adapter::new(adapter);
    let mut le = adapter.le();
    println!("resetting adapter...");
    le.adapter.reset().await?;
    println!("settings advertise parameters...");
    // Set BLE Scan parameters (when to scan, how long, etc)
    le.set_advertising_parameters(le::advertiser::AdvertisingParameters {
        // Advertise as fast as possible for testing
        interval_min: AdvertisingInterval::MIN,
        interval_max: AdvertisingInterval::MIN,
        ..Default::default()
    })
    .await?;
    // Enable scanning for advertisement packets.
    let name_struct = btle::le::advertisement_structures::local_name::LocalName::Complete(
        CompleteLocalName::new("Hello from Rust!"),
    );
    // StaticAdvBuffer is just a stack allocated Vec<u8> with enough room for a full advertisement.
    let buf = name_struct
        .pack_into_storage::<StaticAdvBuffer>()
        .map_err(btle::hci::StreamError::CommandError)?;
    println!("setting advertisement data to `{:?}`", buf.as_ref());
    le.set_advertising_data(buf.as_ref()).await?;
    println!("enabling advertising...");
    le.set_advertising_enable(true).await?;
    println!("advertising!");
    cin_ignore();
    println!("stopping...");
    le.set_advertising_enable(false).await?;
    println!("done!");
    Ok(())
}
