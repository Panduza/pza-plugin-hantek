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
        .create_attribute("bw_limit")
        .with_rw()
        .with_info(
            "* OFF: Turn off the 20MHz bandwidth limit, and the high-frequency components
contained in the measured signal can pass.
* ON: Turn on the bandwidth limitation, and the high-frequency components contained in
the signal under test are attenuated.
Turning on bandwidth limiting reduces waveform noise, but attenuates high-frequency
components.
",
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
            .get_channel_bw_limit(channel_id)
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
        log_debug!(att.logger(), "bw_limit command received '{:?}'", command);

        //
        //
        interface
            .lock()
            .await
            .set_channel_bw_limit(channel_id, command)
            .await?;

        //
        //
        att.set(command).await?;
    }
    Ok(())
}
