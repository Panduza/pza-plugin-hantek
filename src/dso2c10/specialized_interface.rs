// Common
use bytes::Bytes;
use async_trait::async_trait;
use std::sync::Arc;
use strum_macros::FromRepr;
use tokio::sync::Mutex;
use std::collections::HashMap;

use panduza_platform_core::Instance;
use panduza_platform_core::helper;
use panduza_platform_core::Error;
use panduza_platform_core::Logger;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::protocol::BytesDialogProtocol;

use panduza_platform_core::model::TriggerAccessorModel;
use panduza_platform_core::model::BooleanAccessorModel;
use panduza_platform_core::model::StringAccessorModel;
use panduza_platform_core::model::NumberAccessorModel;
use panduza_platform_core::model::VectorF32AccessorModel;

// Base Interface
use panduza_platform_core::interface::usb::UsbTmcInterface;

#[derive(Clone)]
///
///
pub struct SpecializedInterface {
    ///
    ///
    instance: Instance,

    ///
    ///
    base: Arc<Mutex<UsbTmcInterface>>,
    
    ///
    ///
    logger: Logger
}

impl SpecializedInterface {
    ///
    ///
    pub fn new(instance: Instance, base: Arc<Mutex<UsbTmcInterface>>, logger: Logger) -> Self {
        //
        // Log
        log_info!(logger, "Create interface based on usb-tmc");

        //
        // Build the object
        Self {
            instance,
            base,
            logger,
        }
    }
}



#[derive(FromRepr, Debug, PartialEq)]
pub enum BooleanAccessorIndex {
	Channel1Display,
	Channel2Display,
	Triggered,
}

#[async_trait]
///
///
impl BooleanAccessorModel for SpecializedInterface {
    ///
    ///
    async fn get_boolean_at(&mut self, index: usize) -> Result<bool, Error> {
        //
        // Get the index
        let idx = BooleanAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;
    
        //
        // Perform the request
        match idx {
    BooleanAccessorIndex::Channel1Display => Ok(helper::scpi::ScpiBoolean::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel1:DISPlay?")).await?)?.value()),BooleanAccessorIndex::Channel2Display => Ok(helper::scpi::ScpiBoolean::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel2:DISPlay?")).await?)?.value()),BooleanAccessorIndex::Triggered => Ok(helper::scpi::ScpiBoolean::from_bytes_and_map(self.base.lock().await.ask(bytes::Bytes::from("TRIGger:STATus?")).await?, HashMap::from([("TRIGed", true), ("NOTRIG", false), ]))?.value()),_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
		}

}

    ///
    ///
    async fn set_boolean_at(&mut self, index: usize, new_value: bool) -> Result<(), Error> {
        //
        // Get the index
        let idx = BooleanAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx { BooleanAccessorIndex::Channel1Display => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "CHANnel1:DISPlay {}",
                            helper::scpi::ScpiBoolean::new(new_value).to_str()
                        ))).await,
BooleanAccessorIndex::Channel2Display => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "CHANnel2:DISPlay {}",
                            helper::scpi::ScpiBoolean::new(new_value).to_str()
                        ))).await,

		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum StringAccessorIndex {
	TriggerMode,
	TriggerSweep,
}

#[async_trait]
///
///
impl StringAccessorModel for SpecializedInterface {
    ///
    ///
    async fn get_string_at(&mut self, index: usize) -> Result<String, Error> {
        //
        // Get the index
        let idx = StringAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;
    
        //
        // Perform the request
        match idx {
    StringAccessorIndex::TriggerMode => Ok(String::from_utf8(self.base.lock().await.ask(bytes::Bytes::from("TRIGger:MODE?")).await?.to_vec()).unwrap()),StringAccessorIndex::TriggerSweep => Ok(String::from_utf8(self.base.lock().await.ask(bytes::Bytes::from("TRIGger:SWEep?")).await?.to_vec()).unwrap()),_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}

///
    ///
    async fn set_string_at(&mut self, index: usize, new_value: String) -> Result<(), Error> {
        //
        // Get the index
        let idx = StringAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx  { StringAccessorIndex::TriggerMode => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "TRIGger:MODE {}",
                            new_value
                        ))).await,
StringAccessorIndex::TriggerSweep => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "TRIGger:SWEep {}",
                            new_value
                        ))).await,

		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum NumberAccessorIndex {
	Channel1Offset,
	Channel1Scale,
	Channel1Probe,
	SecPerDiv,
	TimebaseOffset,
	TriggerEdgeLevel,
}

#[async_trait]
///
///
impl NumberAccessorModel for SpecializedInterface {
    ///
    ///
    async fn get_number_at(&mut self, index: usize) -> Result<f32, Error> {
        //
        // Get the index
        let idx = NumberAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;
    
        //
        // Perform the request
        match idx {
    NumberAccessorIndex::Channel1Offset => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel1:OFFSet?")).await?)?.value()),
NumberAccessorIndex::Channel1Scale => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel1:SCALe?")).await?)?.value()),
NumberAccessorIndex::Channel1Probe => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel1:PROBe?")).await?)?.value()),
NumberAccessorIndex::SecPerDiv => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("TIMebase:SCALe?")).await?)?.value()),
NumberAccessorIndex::TimebaseOffset => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("TIMebase:POSition?")).await?)?.value()),
NumberAccessorIndex::TriggerEdgeLevel => Ok(helper::scpi::ScpiNumber::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("TRIGger:EDGe:LEVel?")).await?)?.value()),
_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
		}

}

///
    ///
    async fn set_number_at(&mut self, index: usize, new_value: f32) -> Result<(), Error> {
        //
        // Get the index
        let idx = NumberAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx { NumberAccessorIndex::Channel1Offset => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "CHANnel1:OFFSet {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,
NumberAccessorIndex::Channel1Scale => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "CHANnel1:SCALe {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,
NumberAccessorIndex::Channel1Probe => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "CHANnel1:PROBe {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,
NumberAccessorIndex::SecPerDiv => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "TIMebase:SCALe {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,
NumberAccessorIndex::TimebaseOffset => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "TIMebase:POSition {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,
NumberAccessorIndex::TriggerEdgeLevel => self.base.lock().await.tell(bytes::Bytes::from(format!(
                            "TRIGger:EDGe:LEVel {}",
                            helper::scpi::ScpiNumber::new(new_value).to_str()
                        ))).await,

		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum TriggerAccessorIndex {
	TriggerForce,
	Reset,
}

#[async_trait]
///
///
impl TriggerAccessorModel for SpecializedInterface {
    
    

    ///
    ///
    async fn trigger_at(&mut self, index: usize) -> Result<(), Error> {
        //
        // Get the index
        let idx = TriggerAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
TriggerAccessorIndex::TriggerForce => self.base.lock().await.tell(bytes::Bytes::from("TRIGger:FORCe")).await,
TriggerAccessorIndex::Reset => {
                self.base.lock().await.tell(bytes::Bytes::from("*RST")).await.unwrap();
                self.instance.trigger_reset_signal();
                Ok(())
            },
		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum VectorF32AccessorIndex {
	Samples,
}

#[async_trait]
///
///
impl VectorF32AccessorModel for SpecializedInterface {
    
    

    ///
    ///
    async fn get_vectorf32_at(&mut self, index: usize) -> Result<Vec<f32>, Error> {
        //
        // Get the index
        let idx = VectorF32AccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
VectorF32AccessorIndex::Samples => {
                // let p = self
                //     .base
                //     .lock()
                //     .await
                //     .ask(bytes::Bytes::from("WAVeform:SOURce?"))
                //     .await?;

                // let p = self
                //     .base
                //     .lock()
                //     .await
                //     .ask(bytes::Bytes::from("WAVeform:FORMat?"))
                //     .await?;

                // self.base
                //     .lock()
                //     .await
                //     .tell(bytes::Bytes::from("DIGitize CHANnel1"))
                //     .await?;

                let chan1_off = helper::scpi::ScpiNumber::from_bytes(
                    self.base
                        .lock()
                        .await
                        .ask(bytes::Bytes::from("CHANnel1:OFFSet?"))
                        .await?,
                )?
                .value();
                let chan1_scale = helper::scpi::ScpiNumber::from_bytes(
                    self.base
                        .lock()
                        .await
                        .ask(bytes::Bytes::from("CHANnel1:SCALe?"))
                        .await?,
                )?
                .value();

                // Header
                let mut data = Bytes::new();
                for i in 0..6 {
                    data = self
                        .base
                        .lock()
                        .await
                        // .ask(bytes::Bytes::from("WAVeform:DATA:ALL?"))
                        .ask(bytes::Bytes::from("PRIVate:WAVeform:DATA:ALL?"))
                        .await?;

                    if data.len() > 128 {
                        break;
                    }
                }

                let total = data.len();
                if total <= 29 {
                    println!("BAD DATA RECEIVED{:?}", total);
                    // panic!("division by zero");
                    return Ok(vec![0.0]);
                }
                // let sub = total - 29;

                let mut previous = 0.0;
                let mut result = Vec::<f32>::new();
                for value in &data[29..] {
                    if *value >= 200 {
                        // not on the screen
                        result.push(previous);
                    } else {
                        // println!("{:?} - {:?} - {:?}", chan1_off, chan1_probe, chan1_scale);
                        let v_float = *value as f32;
                        // println!("v_float - {:?}", v_float);
                        let v_float_ratio = v_float / 25.0;
                        // println!("v_float_ratio - {:?}", v_float_ratio);
                        let final_value = (v_float_ratio * chan1_scale) - chan1_off;
                        // if final_value > 10.0 {
                        //     println!("{:?} => {:?}", value, final_value);
                        // }

                        result.push(final_value);
                        previous = final_value;
                    }
                }

                Ok(result)
            },
		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}

    async fn set_vectorf32_at(&mut self, index: usize, value: Vec<f32>) -> Result<(), Error> { Ok(()) }

}

    