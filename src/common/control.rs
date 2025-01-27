mod current;
mod options;
mod voltage;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{
    protocol::AsciiCmdRespProtocol, spawn_on_command, BooleanAttServer, Error, Instance,
    InstanceLogger,
};
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
    // Start logging
    let logger = instance.logger.clone();
    logger.info("Mounting 'control' class...");

    //
    // Create attribute
    let mut itf_control = instance.create_class("control").finish();

    current::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;
    voltage::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;
    options::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .finish_as_boolean()
        .await?;

    let v = driver.lock().await.get_out().await?;
    att_oe.set(v).await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_oe_2 = att_oe.clone();
    spawn_on_command!(
        "on_command => control",
        instance,
        att_oe_2,
        on_command(logger_2.clone(), att_oe_2.clone(), driver.clone())
    );

    //
    // End of mount
    logger.info("Mounting 'control' class -> OK");
    Ok(())
}

///
///
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: InstanceLogger,
    mut att_oe: BooleanAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_oe.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI voltage command received '{:?}'", command));

        driver.lock().await.set_out(command).await?;

        let v = driver.lock().await.get_out().await?;
        att_oe.set(v).await?;
    }
    Ok(())
}
