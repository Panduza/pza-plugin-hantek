use panduza_platform_core::{
    log_debug, log_debug_mount_end, log_debug_mount_start, spawn_on_command, Container, Error,
    SiAttServer,
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
        .create_attribute("offset")
        .with_rw()
        .with_info(
            "Manage the vertical displacement of the specified channel

* The set vertical displacement value is affected by the vertical gear and probe ratio.
* The range of legal values varies with the vertical scale and probe ratio. If 21 / 136
you set an offset outside the legal range, the offset value will be automatically set to
the closest legal value
",
        )
        .finish_as_si("V", -10.0, 10.0, 1)
        .await?;
    let logger = att.logger().clone();
    log_debug_mount_start!(logger);

    // Set the value
    att.set_from_f32(
        interface
            .lock()
            .await
            .get_channel_offset(channel_id)
            .await? as f32,
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
    mut att: SiAttServer,
    channel_id: usize,
    interface: Arc<Mutex<DSO2C10Interface>>,
) -> Result<(), Error> {
    while let Some(command) = att.pop_cmd_as_f32().await {
        match command {
            Ok(c) => {
                //
                // Log
                log_debug!(att.logger(), "offset command received '{:?}'", command);

                //
                //
                interface
                    .lock()
                    .await
                    .set_channel_offset(channel_id, c as f64)
                    .await?;

                // Set the value
                att.set_from_f32(
                    interface
                        .lock()
                        .await
                        .get_channel_offset(channel_id)
                        .await? as f32,
                )
                .await?;
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}
