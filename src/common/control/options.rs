// mod beep;
// mod lock;
// mod mode;
mod ocp;
mod ovp;
use crate::common::driver::KoradDriver;
use panduza_platform_core::{protocol::AsciiCmdRespProtocol, Class, Instance, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    instance: Instance,
    mut interface: Class,
    driver: Arc<Mutex<KoradDriver<SD>>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let itf_options = interface.create_class("options").finish();

    ovp::mount(instance.clone(), itf_options.clone(), driver.clone()).await?;
    ocp::mount(instance.clone(), itf_options.clone(), driver.clone()).await?;
    // beep::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    // lock::mount(device.clone(), itf_options.clone(), driver.clone()).await?;
    // mode::mount(device.clone(), itf_options.clone(), driver.clone()).await?;

    Ok(())
}
