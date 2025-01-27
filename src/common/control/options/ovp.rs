use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{spawn_on_command, BooleanAttServer, Class, Instance, InstanceLogger};
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
    //
    let att_voltage = class
        .create_attribute("ovp")
        .with_wo()
        .with_info(
            r#"Enable/Disable the Over Voltage Protection.
        'Write Only' because the power supply does not given any read function on it"#,
        )
        .finish_as_boolean()
        .await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_voltage_2 = att_voltage.clone();
    spawn_on_command!(
        "on_command => control/options/ovp",
        instance,
        att_voltage_2,
        on_command(logger_2.clone(), att_voltage_2.clone(), driver.clone())
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
    mut value_value_attr: BooleanAttServer,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("OVP command received '{:?}'", command));

        //
        //
        driver.lock().await.set_ovp(command).await?;
    }
    Ok(())
}
