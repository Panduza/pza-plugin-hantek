use panduza_platform_core::{
    log_debug, log_debug_mount_end, log_debug_mount_start, spawn_on_command, BooleanAttServer,
    Container, Error,
};
use tokio::sync::Mutex;

use std::sync::Arc;

use crate::dso2c10::device::interface::DSO2C10Interface;

///
///
pub async fn mount<C: Container + 'static>(
    mut parent: C,
    channel_id: usize,
    interface: Arc<Mutex<DSO2C10Interface>>,
) -> Result<(), Error> {
    //
    //
    let att = parent
        .create_attribute("invert")
        .with_rw()
        .with_info(
            "Turn the waveform inversion of the specified channel on or off or query the switching
status of the waveform inversion of the specified channel.",
        )
        .finish_as_boolean()
        .await?;
    let logger = att.logger().clone();
    log_debug_mount_start!(logger);

    // Set the value
    att.set(
        interface
            .lock()
            .await
            .get_channel_invert(channel_id)
            .await?,
    )
    .await?;

    //
    spawn_on_command!(
        "on_command => boolean",
        parent,
        att,
        on_command(att.clone(), channel_id, interface.clone())
    );

    //
    //
    log_debug_mount_end!(logger);
    Ok(())
}

///
///
async fn on_command(
    mut att: BooleanAttServer,
    channel_id: usize,
    interface: Arc<Mutex<DSO2C10Interface>>,
) -> Result<(), Error> {
    while let Some(command) = att.pop_cmd().await {
        //
        // Log
        log_debug!(att.logger(), "invert command received '{:?}'", command);

        //
        //
        interface
            .lock()
            .await
            .set_channel_invert(channel_id, command)
            .await?;

        //
        //
        att.set(command).await?;
    }
    Ok(())
}
