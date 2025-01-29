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
        .create_attribute("vernier")
        .with_rw()
        .with_info(
            "Manage the fine adjustment function of the vertical scale.

The trim setting is off by default. At this time, you can only set the vertical scale in 1-
2-5 steps, that is, 500u, 1mV, 2mV, 5mV, 10mV ... 10V (probe ratio is 1X). When the
trim setting is on, you can further adjust the vertical scale within a smaller range to
improve vertical resolution. If the amplitude of the input waveform is slightly larger
than the full scale in the current scale, and the amplitude displayed by the waveform
of the next gear is slightly lower, you can use fine adjustment to improve the
waveform display amplitude to facilitate observation of signal details.
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
            .get_channel_vernier(channel_id)
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
        log_debug!(att.logger(), "vernier command received '{:?}'", command);

        //
        //
        interface
            .lock()
            .await
            .set_channel_vernier(channel_id, command)
            .await?;

        //
        //
        att.set(command).await?;
    }
    Ok(())
}
