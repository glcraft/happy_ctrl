use crate::args::Args;
use btleplug::{
    Error as BtError,
    api::Manager as _,
    platform::{Adapter, Manager},
};
use log::{LevelFilter, info};
pub async fn find_first_adapter(manager: &Manager) -> Result<Option<Adapter>, BtError> {
    let adapters = manager.adapters().await?;
    info!("Selecting first adapter found");
    Ok(adapters.into_iter().nth(0))
}
pub async fn find_adapter_with(
    manager: &Manager,
    predicate: impl AsyncFn(&Adapter) -> Result<bool, BtError>,
) -> Result<Option<Adapter>, BtError> {
    info!("Selecting adapter...");
    let adapters = manager.adapters().await?;
    for adapter in adapters {
        if predicate(&adapter).await? {
            info!("Adapter found!");
            return Ok(Some(adapter));
        }
    }
    Ok(None)
}

pub fn init_logger(args: &Args) -> Result<(), log::SetLoggerError> {
    let log_level = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::max(),
    };
    simple_logger::SimpleLogger::new()
        .with_level(log_level)
        .env()
        .init()
}
