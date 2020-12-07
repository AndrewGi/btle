use btle::uuid::UUID;
use btle::{windows::ble::gatt, BTAddress};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Builder::new()
        .enable_all()
        .build()
        .expect("can't make async runtime");
    runtime.block_on(main_async())
}
async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = UUID([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ]);
    let provider = gatt::services::ServiceProvider::new(&uuid).await?;
    Ok(())
}
