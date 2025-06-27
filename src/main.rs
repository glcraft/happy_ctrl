mod args;
mod device;
mod util;

use args::{Args, Command};

use btleplug::{
    api::{Central, CharPropFlags, Peripheral, bleuuid::uuid_from_u16},
    platform::Manager,
};
use clap::Parser;
use device::LedDevice;
use log::info;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Bluetooth adapter not found")]
    AdapterNotFound,
    #[error("Peripheral not found")]
    PeripheralNotFound,
    #[error("Characteristic not found")]
    CharNotFound,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    util::init_logger(&args)?;
    let manager = Manager::new().await?;
    let adapter = match args.adapter.as_ref() {
        Some(name) => {
            util::find_adapter_with(&manager, async |a| {
                Ok(a.adapter_info().await?.contains(name))
            })
            .await?
        }
        None => util::find_first_adapter(&manager).await?,
    }
    .ok_or(Error::AdapterNotFound)?;
    info!("Adapter status: {:?}", adapter.adapter_state().await?);
    let peripheral = device::LedDevice::find_peripheral(&adapter, async |peripheral| {
        let properties = peripheral.properties().await?;
        let name = properties.and_then(|p| p.local_name).unwrap_or_default();
        Ok(name.contains(&args.name))
    })
    .await?
    .ok_or(Error::PeripheralNotFound)?;

    let led_device = LedDevice::from(peripheral, |characteristic| {
        characteristic
            .properties
            .contains(CharPropFlags::WRITE_WITHOUT_RESPONSE | CharPropFlags::WRITE)
            && characteristic.service_uuid == uuid_from_u16(0xffd5)
    })
    .await?;

    match args.command {
        Command::PowerOn => led_device.power_on().await?,
        Command::PowerOff => led_device.power_off().await?,
        Command::SetRGB { r, g, b } => led_device.set_rgb(r, g, b).await?,
    };
    Ok(())
}
