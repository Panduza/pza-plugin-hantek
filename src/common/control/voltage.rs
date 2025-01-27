use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{log_debug, log_info, AttributeLogger, Error, SiAttServer};
use panduza_platform_core::{spawn_on_command, Class, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
/// control/voltage
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    mut class: Class,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Start logging
    let logger = instance.logger.new_attribute_logger("control", "voltage");
    log_debug!(logger, "Mounting...");

    //
    // Create the attribute
    let att_server = class
        .create_attribute("voltage")
        .with_rw()
        .with_info(r#"Allow to read & write the voltage limit value of the power supply"#)
        .finish_as_si("V", 0, 30, 2)
        .await?;

    //
    // Init with a first value
    let v = driver.lock().await.get_vset().await?;
    att_server.set_from_f32(v).await?;

    //
    // Execute action on each command received
    let logger_2 = logger.clone();
    let att_server_2 = att_server.clone();
    spawn_on_command!(
        "on_command => control/voltage",
        instance,
        att_server_2,
        on_command(logger_2.clone(), att_server_2.clone(), driver.clone())
    );

    //
    // End of mount
    log_debug!(logger, "Mounting => OK");
    Ok(())
}

///
/// control/voltage => triggered when command is received
///
async fn on_command<SD: AsciiCmdRespProtocol>(
    logger: AttributeLogger,
    mut att_server: SiAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command_result) = att_server.pop_cmd_as_f32().await {
        match command_result {
            Ok(v) => {
                log_info!(logger, "command received '{:?}'", v);
                driver.lock().await.set_vset(v).await?;
                att_server.set_from_f32(v).await?;
                let real_value = driver.lock().await.get_vset().await?;
                att_server.set_from_f32(real_value).await?;
            }
            Err(e) => {
                let alert = format!("warning on received command '{:?}'", e);
                logger.warn(&alert);
                att_server.send_alert(alert).await;
                let real_value = driver.lock().await.get_vset().await?;
                att_server.set_from_f32(real_value).await?;
            }
        }
    }
    Ok(())
}
