use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{log_info, spawn_on_command, Class, Instance, InstanceLogger};
use panduza_platform_core::{BooleanAttServer, SiAttServer};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    mut class: Class,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create interface
    let mut c_interface = class.create_class("voltage").finish();

    //
    //
    let att_voltage = c_interface
        .create_attribute("value")
        .with_ro()
        .with_info(r#"Hold values returned from the ampermeter"#)
        .finish_as_si("V", 0, 30, 2)
        .await?;

    //
    // Init with a first value
    att_voltage.set_from_f32(0.0).await?;

    //
    //
    let att_trigger = c_interface
        .create_attribute("trigger")
        .with_rw()
        .with_info(
            r#"Trigger a measure in the ampermeter, the data is published on the 'value' topic"#,
        )
        .finish_as_boolean()
        .await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_trigger_2 = att_trigger.clone();
    spawn_on_command!(
        "on_command => measure/voltage/trigger",
        instance,
        att_trigger_2,
        on_command(
            logger_2.clone(),
            att_trigger_2.clone(),
            att_voltage.clone(),
            driver.clone()
        )
    );

    //
    // Function ok
    Ok(())
}

///
///
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: InstanceLogger,
    mut att_trigger: BooleanAttServer,
    att_voltage: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_trigger.pop_cmd().await {
        //
        // Log
        log_info!(
            logger,
            "'measure/voltage/trigger' - command received '{:?}'",
            command
        );

        if command == true {
            let v = driver.lock().await.get_vout().await?;
            att_voltage.set_from_f32(v).await?;
            att_trigger.set(true).await?;
        }
    }
    Ok(())
}
