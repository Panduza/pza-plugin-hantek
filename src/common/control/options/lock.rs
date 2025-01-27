use crate::common::driver::KoradDriver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::Error;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, BooleanCodec, Device, InstanceLogger, Interface,
};
use std::sync::Arc;
use tokio::sync::Mutex;
///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol>(
    mut instance: Instance,
    mut interface: Interface,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    //
    let att_voltage = interface
        .create_attribute("lock")
        .message()
        .with_bidir_access()
        .finish_with_codec::<BooleanCodec>()
        .await;

    att_voltage.set(false).await.unwrap();

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();
    let att_voltage_for_cmd_event = att_voltage.clone();
    spawn_on_command!(
        device,
        att_voltage_for_cmd_event,
        on_command(
            logger_for_cmd_event.clone(),
            att_voltage_for_cmd_event.clone(),
        )
    );

    //
    // Function ok
    Ok(())
}

///
///
///
async fn on_command(
    logger: InstanceLogger,
    mut value_value_attr: BidirMsgAtt<BooleanCodec>,
) -> Result<(), Error> {
    while let Some(command) = value_value_attr.pop_cmd().await {
        //
        // Log
        logger.debug(format!("SI ovp command received '{:?}'", command));

        value_value_attr.set(command).await?;

        // if command.value == "0".to_string() {
        //     // connector.pico_set_bus_id(0).await?;

        // } else if command.value == "1".to_string() {
        //     // connector.pico_set_bus_id(1).await?;
        //     value_value_attr.set("1".to_string()).await?;
        // } else {
        //     logger.error(format!("BAD BUS ID !!!! {:?}", command.value));
        // }
    }
    Ok(())
}
