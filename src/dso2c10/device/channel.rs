mod bw_limit;
mod display;
mod invert;
mod offset;
mod vernier;
use tokio::sync::Mutex;

use panduza_platform_core::{
    log_debug, log_debug_mount_end, log_debug_mount_start, BooleanAttServer, Container, Error,
};
use std::sync::Arc;

use super::interface::DSO2C10Interface;

///
///
pub async fn mount<C: Container + 'static>(
    mut parent: C,
    channel_id: usize,
    interface: Arc<Mutex<DSO2C10Interface>>,
) -> Result<(), Error> {
    // CHANnel<n>:BWLimit
    // CHANnel<n>:COUPling
    // CHANnel<n>:DISPlay
    // CHANnel<n>:INVert
    // CHANnel<n>:OFFSet
    // CHANnel<n>:SCALe
    // CHANnel<n>:PROBe
    // CHANnel<n>:VERNier

    let class_chan = parent
        .create_class(format!("{}", channel_id))
        .finish()
        .await;

    let logger = class_chan.logger().clone();
    log_debug_mount_start!(logger);

    bw_limit::mount(class_chan.clone(), channel_id, interface.clone()).await?;
    display::mount(class_chan.clone(), channel_id, interface.clone()).await?;
    invert::mount(class_chan.clone(), channel_id, interface.clone()).await?;
    vernier::mount(class_chan.clone(), channel_id, interface.clone()).await?;
    offset::mount(class_chan.clone(), channel_id, interface.clone()).await?;

    //
    //
    log_debug_mount_end!(logger);
    Ok(())
}

///
///
async fn on_command(mut att: BooleanAttServer) -> Result<(), Error> {
    while let Some(command) = att.pop_cmd().await {
        //
        // Log
        log_debug!(att.logger(), "OCP command received '{:?}'", command);

        //
        //
        att.set(command).await?;
    }
    Ok(())
}
