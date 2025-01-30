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
        .create_attribute("scale")
        .with_rw()
        .with_info(
            "Vertical scale of the channel, vertical units per division
* 2mV
* 5mV
* 10mV
* 20mV
* 50mV
* 100mV
* 200mV
* 500mV
* 1V
* 2V
* 5V
* 10V        
        ",
        )
        .finish_as_si("V", -10.0, 10.0, 1)
        .await?;
    let logger = att.logger().clone();
    log_debug_mount_start!(logger);

    // This values is an enum
    // 2mV
    // 5mV
    // 10mV
    // 20mV
    // 50mV
    // 100mV
    // 200mV
    // 500mV
    // 1V
    // 2V
    // 5V
    // 10V

    // Set the value
    att.set_from_f32(interface.lock().await.get_channel_scale(channel_id).await? as f32)
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
                log_debug!(att.logger(), "scale command received '{:?}'", command);

                //
                //
                interface
                    .lock()
                    .await
                    .set_channel_scale(channel_id, c as f64)
                    .await?;

                //
                // Log
                let read_back = interface.lock().await.get_channel_scale(channel_id).await?;
                log_debug!(att.logger(), "read back '{:?}'", read_back);

                // Set the value
                att.set_from_f32(read_back as f32).await?;
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}
