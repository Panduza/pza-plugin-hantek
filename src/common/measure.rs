mod ampermeter;
mod voltmeter;

use crate::common::driver::KoradDriver;
use panduza_platform_core::{protocol::AsciiCmdRespProtocol, Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let itf_measure = instance.create_class("measure").finish();

    ampermeter::mount(instance.clone(), itf_measure.clone(), driver.clone()).await?;
    voltmeter::mount(instance.clone(), itf_measure.clone(), driver.clone()).await?;

    Ok(())
}
