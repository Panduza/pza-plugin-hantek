//
pub use super::SpecializedInterface;
pub use super::BooleanAccessorIndex;
pub use super::StringAccessorIndex;
pub use super::NumberAccessorIndex;
pub use super::TriggerAccessorIndex;

use serde_json::json;
use async_trait::async_trait;
use panduza_platform_core::Error;
use panduza_platform_core::Actions;
use panduza_platform_core::Instance;
use panduza_platform_core::template;
use panduza_platform_core::Container;
use std::time::Duration;
use tokio::time::sleep;

use panduza_platform_core::interface::serial::SerialSettings;
use panduza_platform_core::interface::serial::SerialEolInterface;
use panduza_platform_core::interface::usb::UsbSettings;
use panduza_platform_core::interface::usb::UsbTmcInterface;

#[derive(Default)]
///
///
pub struct Device {}

impl Device {
    /// Constructor
    ///
    pub fn new() -> Self {
        Device {}
    }
}

#[async_trait]
impl Actions for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger().clone();


        //
        // Usb settings
        let settings = instance.settings().await.ok_or(Error::BadSettings(
            "Usb Settings are required for this instance".to_string(),
        ))?;

        //
        // Compose USB settings
        let usb_settings = UsbSettings::from_json_settings(&settings);

        //
        // 
        let base = UsbTmcInterface::open(&usb_settings)?.into_arc_mutex();
        
        
        //
        //
        let interface = SpecializedInterface::new(base, logger.clone());
        
//
        //
        template::attribute::trigger::mount(
            instance.clone(),
            interface.clone(),
            TriggerAccessorIndex::SignalTrigger as usize,
            "signal trigger",
            "info",
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::SignalDuration as usize,
            "signal duration",
            "info",
            "-",
            0.0,
            5000.0,
            3,
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::DutyCycle as usize,
            "duty cycle",
            "info",
            "-",
            0.0,
            5000.0,
            3,
        )
        .await?;
    
        
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
    