use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{log_debug, Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol>(
    mut instance: Instance,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create the local logger
    let logger = instance.logger.new_attribute_logger("", "identity");
    log_debug!(logger, "Mounting...");

    //
    // Create attribute
    let att_identity = instance
        .create_attribute("identity")
        .with_ro()
        .with_info("Identity string of the power supply")
        .finish_as_string()
        .await?;

    //
    // Just init
    let idn = driver.lock().await.get_idn().await?;
    log_debug!(logger, "IDN ({:?})", &idn);
    att_identity.set(idn).await?;

    //
    // End
    log_debug!(logger, "Mounting => OK");
    Ok(())
}
