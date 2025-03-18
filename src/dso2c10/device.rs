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
        let interface = SpecializedInterface::new(instance.clone(), base, logger.clone());
        
//
            //
            template::attribute::boolean::mount(
                instance.clone(),
                interface.clone(),
                BooleanAccessorIndex::Channel1Display as usize,
                "channel1_display",
                "info",
            )
            .await?;
            //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::Channel1Offset as usize,
            "channel1_offset",
            "info",
            "-",
            -100000 as f64,
            100000 as f64,
            20,
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::Channel1Scale as usize,
            "channel1_scale",
            "info",
            "-",
            -100000 as f64,
            100000 as f64,
            20,
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::Channel1Probe as usize,
            "channel1_probe",
            "info",
            "-",
            -100000 as f64,
            100000 as f64,
            20,
        )
        .await?;
    //
            //
            template::attribute::boolean::mount(
                instance.clone(),
                interface.clone(),
                BooleanAccessorIndex::Channel2Display as usize,
                "channel2_display",
                "info",
            )
            .await?;
            //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::SecPerDiv as usize,
            "sec_per_div",
            "info",
            "-",
            -100000 as f64,
            100000 as f64,
            9,
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::TimebaseOffset as usize,
            "timebase_offset",
            "info",
            "-",
            -1000 as f64,
            1000 as f64,
            9,
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
        template::attribute::r#enum::mount(
            instance.clone(),
            interface.clone(),
            StringAccessorIndex::TriggerMode as usize,
            "trigger_mode",
            "info",
            vec![ "EDGE",  "PULSe",  "TV",  "SLOPe",  "TIMeout",  "WINdow",  "PATTern",  "INTerval",  "UNDerthrow",  "UART",  "LIN",  "CAN",  "SPI",  "IIC", ]
        )
        .await?;
    //
        //
        template::attribute::trigger::mount(
            instance.clone(),
            interface.clone(),
            TriggerAccessorIndex::TriggerForce as usize,
            "trigger_force",
            "info",
        )
        .await?;
    //
        //
        template::attribute::trigger::mount(
            instance.clone(),
            interface.clone(),
            TriggerAccessorIndex::Reset as usize,
            "reset",
            "info",
        )
        .await?;
    //
        //
        template::attribute::number::mount(
            instance.clone(),
            interface.clone(),
            NumberAccessorIndex::TriggerEdgeLevel as usize,
            "trigger_edge_level",
            "info",
            "-",
            -100000 as f64,
            100000 as f64,
            2,
        )
        .await?;
    //
        //
        template::attribute::r#enum::mount(
            instance.clone(),
            interface.clone(),
            StringAccessorIndex::TriggerSweep as usize,
            "trigger_sweep",
            "info",
            vec![ "AUTO",  "NORMal",  "SINGle", ]
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
    