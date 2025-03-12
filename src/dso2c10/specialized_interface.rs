// Common
use bytes::Bytes;
use async_trait::async_trait;
use std::sync::Arc;
use strum_macros::FromRepr;
use tokio::sync::Mutex;

use panduza_platform_core::helper;
use panduza_platform_core::Error;
use panduza_platform_core::Logger;
use panduza_platform_core::log_info;
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
    base: Arc<Mutex<UsbTmcInterface>>,
    
    ///
    ///
    logger: Logger
}

impl SpecializedInterface {
    ///
    ///
    pub fn new(base: Arc<Mutex<UsbTmcInterface>>, logger: Logger) -> Self {
        //
        // Log
        log_info!(logger, "Create interface based on usb-tmc");

        //
        // Build the object
        Self {
            base,
            logger,
        }
    }
}



#[derive(FromRepr, Debug, PartialEq)]
pub enum BooleanAccessorIndex {
	Display,
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
    BooleanAccessorIndex::Display => Ok(helper::scpi::ScpiBoolean::from_bytes(self.base.lock().await.ask(bytes::Bytes::from("CHANnel1:DISPlay?")).await?)?.value()),_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
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
        match idx { BooleanAccessorIndex::Display => self.base.lock().await.tell(bytes::Bytes::from(format!(
                        "CHANnel1:DISPlay {}",
                        helper::scpi::ScpiBoolean::new(new_value).to_str()
                    ))).await,

		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum StringAccessorIndex {
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
    _ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}

///
    ///
    async fn set_string_at(&mut self, index: usize, value: String) -> Result<(), Error> {
        //
        // Get the index
        let idx = StringAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum NumberAccessorIndex {
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
    _ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
		}

}

///
    ///
    async fn set_number_at(&mut self, index: usize, value: f32) -> Result<(), Error> {
        //
        // Get the index
        let idx = NumberAccessorIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}


#[derive(FromRepr, Debug, PartialEq)]
pub enum TriggerAccessorIndex {
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
		_ => { Err(Error::InvalidArgument("No Action for Index".to_string())) }
}
}
}

    