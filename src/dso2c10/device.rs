mod channel;
mod interface;

use interface::DSO2C10Interface;

use async_trait::async_trait;
use panduza_platform_core::connector::usb::tmc::Driver as UsbTmcDriver;
use panduza_platform_core::connector::usb::Settings as UsbSettings;
use panduza_platform_core::{log_debug, Container, DriverOperations, Error, Instance};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Default)]
///
/// Device to control PicoHA SSB Board
///
pub struct Device {}

impl Device {}

#[async_trait]
impl DriverOperations for Device {
    ///
    /// Mount the device instance
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger.clone();

        //
        // Usb settings
        let settings = instance.settings().await.ok_or(Error::BadSettings(
            "Usb Settings are required for this instance".to_string(),
        ))?;

        //
        // Compose USB settings
        let usb_settings = UsbSettings::from_json_settings(&settings);
        log_debug!(logger, "Try to open SCPI interface on {:?}", &usb_settings);

        //
        // Mount the driver
        let driver = UsbTmcDriver::open(&usb_settings)?.into_arc_mutex();

        let interface: Arc<Mutex<DSO2C10Interface>> =
            Arc::new(Mutex::new(DSO2C10Interface::new(driver, logger.clone())));

        panduza_platform_core::std::class::repl::mount("repl", instance.clone(), interface.clone())
            .await?;

        panduza_platform_core::std::attribute::idn::mount(instance.clone(), interface.clone())
            .await?;

        let class_channels = instance.create_class("channel").finish().await;
        for i in 1..2 {
            channel::mount(class_channels.clone(), i, interface.clone()).await?;
        }

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
