//
pub use super::SpecializedInterface;
pub use super::BooleanAccessorIndex;
pub use super::StringAccessorIndex;
pub use super::NumberAccessorIndex;
pub use super::TriggerAccessorIndex;
pub use super::VectorF32AccessorIndex;

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
    //
    let settings = instance.settings().await.or(Some(json!({}))).unwrap();;

    //
    //
    let usb_settings = UsbSettings::new().set_vid(0x49f).set_pid(20574);

    //
    // 
    let base = UsbTmcInterface::open(&usb_settings)?.into_arc_mutex();
    
        
        //
        //
        let interface = SpecializedInterface::new(base, logger.clone());
        
//
            //
            template::attribute::boolean::mount(
                instance.clone(),
                interface.clone(),
                BooleanAccessorIndex::Display as usize,
                "display",
                "info",
            )
            .await?;
            //
            //
            template::class::boolean_acquisitor::mount(
                instance.clone(),
                interface.clone(),
                BooleanAccessorIndex::Triggered as usize,
                "triggered",
            )
            .await?;
            //
            //
            template::class::vectorf32_acquisitor::mount(
                instance.clone(),
                interface.clone(),
                VectorF32AccessorIndex::Samples as usize,
                "samples",
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
    