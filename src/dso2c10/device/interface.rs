use async_trait::async_trait;
use panduza_platform_core::connector::usb::tmc::Driver as UsbTmcInterface;

use panduza_platform_core::std::attribute::boolean::BooleanAccessorModel;
use panduza_platform_core::std::attribute::idn::IdnReader;
use panduza_platform_core::std::attribute::r#enum::StringAccessorModel;
use panduza_platform_core::std::class::repl::ReplProtocol;
use panduza_platform_core::{log_info, log_trace, Error, Logger};
use strum_macros::FromRepr;
use tokio::sync::Mutex;

use std::sync::Arc;
use std::time::Instant;

use crate::dso2c10::ScpiBoolean;

#[derive(Clone)]
/// Interface to communicate with the DSO2C10 device
///
pub struct DSO2C10Interface {
    /// Lower level interface (USB TMC)
    ///
    sub_interface: Arc<Mutex<UsbTmcInterface>>,

    /// Logger for the driver
    ///
    logger: Logger,
}

///
///
impl DSO2C10Interface {
    ///
    ///
    pub fn new(sub_interface: Arc<Mutex<UsbTmcInterface>>, logger: Logger) -> Self {
        //
        // Log
        log_info!(logger, "Create DSO2C10Interface with USBTMC sub_interface",);

        //
        // Build the object
        DSO2C10Interface {
            sub_interface,
            logger,
        }
    }

    /// Generic way to get boolean parameter from the device
    ///
    pub async fn get_boolean_parameter(&self, cmd: &[u8]) -> Result<bool, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let mut response: Vec<u8> = Vec::new();
        self.sub_interface
            .lock()
            .await
            .execute_command(cmd, &mut response)
            .await?;

        //
        // Log
        log_trace!(
            self.logger,
            "ASK <=> {:?} - {:?} - {:.2?}",
            cmd,
            response,
            start.elapsed()
        );

        //
        // End
        let v = ScpiBoolean::from_vec_ascii(&response).map_err(|e| Error::DeserializeError(e))?;
        Ok(v.into())
    }

    ///
    ///
    pub async fn set_boolean_parameter(&self, cmd: &str, value: bool) -> Result<(), Error> {
        let cmd_string = format!("{} {}", cmd, ScpiBoolean::new(value).to_digital_str());
        self.sub_interface
            .lock()
            .await
            .send_command(cmd_string.as_bytes())
            .await
    }

    /// Generic way to get string parameter from the device
    ///
    pub async fn get_string_parameter(&self, cmd: &[u8]) -> Result<String, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let mut response: Vec<u8> = Vec::new();
        self.sub_interface
            .lock()
            .await
            .execute_command(cmd, &mut response)
            .await?;

        //
        // Log
        log_trace!(
            self.logger,
            "ASK <=> {:?} - {:?} - {:.2?}",
            cmd,
            response,
            start.elapsed()
        );

        //
        // End
        match String::from_utf8(response) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::DeserializeError(
                "Cannot convert response into string".to_string(),
            )),
        }
    }

    ///
    ///
    pub async fn set_string_parameter(&self, cmd: &str, value: &String) -> Result<(), Error> {
        let cmd_string = format!("{} {}", cmd, value);
        self.sub_interface
            .lock()
            .await
            .send_command(cmd_string.as_bytes())
            .await
    }

    /// Generic way to get string parameter from the device
    ///
    pub async fn get_float_parameter(&self, cmd: &[u8]) -> Result<f64, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let mut response: Vec<u8> = Vec::new();
        self.sub_interface
            .lock()
            .await
            .execute_command(cmd, &mut response)
            .await?;

        //
        // Log
        log_trace!(
            self.logger,
            "ASK <=> {:?} - {:?} - {:.2?}",
            cmd,
            response,
            start.elapsed()
        );

        //
        // End
        match String::from_utf8(response) {
            Ok(s) => Ok(s
                .parse::<f64>()
                .map_err(|_| Error::Generic("Cannot convert response into float".to_string()))?),
            Err(e) => Err(Error::Generic(
                "Cannot convert response into string".to_string(),
            )),
        }
    }

    ///
    ///
    pub async fn get_channel_coupling(&self, channel_id: usize) -> Result<String, Error> {
        let cmd_string = format!("CHANnel{}:COUPling?", channel_id);
        self.get_string_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn get_channel_offset(&self, channel_id: usize) -> Result<f64, Error> {
        let cmd_string = format!("CHANnel{}:OFFSet?", channel_id);
        self.get_float_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_offset(&self, channel_id: usize, value: f64) -> Result<(), Error> {
        // let mut v = "0";
        // if value {
        //     v = "1";
        // }
        let cmd_string = format!("CHANnel{}:OFFSet {}", channel_id, value);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
    }

    ///
    ///
    pub async fn get_channel_scale(&self, channel_id: usize) -> Result<f64, Error> {
        let cmd_string = format!("CHANnel{}:SCALe?", channel_id);
        self.get_float_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_scale(&self, channel_id: usize, value: f64) -> Result<(), Error> {
        // let mut v = "0";
        // if value {
        //     v = "1";
        // }
        let cmd_string = format!("CHANnel{}:SCALe {}", channel_id, value);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
    }

    // CHANnel<n>:PROBe
}

#[async_trait]
impl ReplProtocol for DSO2C10Interface {
    ///
    ///
    async fn eval(&mut self, command: String) -> Result<String, Error> {
        self.sub_interface.lock().await.eval(command).await
    }
}

#[async_trait]
/// Implement IDN Protocol
///
impl IdnReader for DSO2C10Interface {
    async fn read_idn(&mut self) -> Result<String, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let mut response: Vec<u8> = Vec::new();
        let cmd = "*IDN?".as_bytes();
        self.sub_interface
            .lock()
            .await
            .execute_command(cmd, &mut response)
            .await?;

        //
        // Log
        log_trace!(
            self.logger,
            "ASK <=> {:?} - {:?} - {:.2?}",
            cmd,
            response,
            start.elapsed()
        );

        //
        // End
        match String::from_utf8(response) {
            Ok(s) => Ok(s),
            Err(_) => Ok("Cannot convert the payload into string".to_string()),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(FromRepr, Debug, PartialEq)]
pub enum BooleanIndex {
    //
    Channel1BwLimit,
    Channel1Display,
    Channel1Invert,
    Channel1Vernier,
    //
    Channel2BwLimit,
    Channel2Display,
    Channel2Invert,
    Channel2Vernier,
    //
    TimebaseWindowEnable,
    //
    MeasureEnable,
    MeasureADisplay,
    MeasureGateEnable,
}

#[async_trait]
///
///
impl BooleanAccessorModel for DSO2C10Interface {
    ///
    ///
    async fn get_boolean_at(&mut self, index: usize) -> Result<bool, Error> {
        //
        // Get the index
        let idx = BooleanIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
            //
            BooleanIndex::Channel1BwLimit => self.get_boolean_parameter(b"CHANnel1:BWLimit?").await,
            BooleanIndex::Channel1Display => self.get_boolean_parameter(b"CHANnel1:DISPlay?").await,
            BooleanIndex::Channel1Invert => self.get_boolean_parameter(b"CHANnel1:INVert?").await,
            BooleanIndex::Channel1Vernier => self.get_boolean_parameter(b"CHANnel1:VERNier?").await,
            //
            BooleanIndex::Channel2BwLimit => self.get_boolean_parameter(b"CHANnel2:BWLimit?").await,
            BooleanIndex::Channel2Display => self.get_boolean_parameter(b"CHANnel2:DISPlay?").await,
            BooleanIndex::Channel2Invert => self.get_boolean_parameter(b"CHANnel2:INVert?").await,
            BooleanIndex::Channel2Vernier => self.get_boolean_parameter(b"CHANnel2:VERNier?").await,
            //
            BooleanIndex::TimebaseWindowEnable => {
                self.get_boolean_parameter(b"TIMebase:WINDow:ENABle?").await
            }
            //
            BooleanIndex::MeasureEnable => self.get_boolean_parameter(b"MEASure:ENABle?").await,
            BooleanIndex::MeasureADisplay => self.get_boolean_parameter(b"MEASure:ADISplay?").await,
            BooleanIndex::MeasureGateEnable => {
                self.get_boolean_parameter(b"MEASure:GATE:ENABle?").await
            }
        }
    }

    ///
    ///
    async fn set_boolean_at(&mut self, index: usize, value: bool) -> Result<(), Error> {
        //
        // Get the index
        let idx = BooleanIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
            //
            BooleanIndex::Channel1BwLimit => {
                self.set_boolean_parameter("CHANnel1:BWLimit", value).await
            }
            BooleanIndex::Channel1Display => {
                self.set_boolean_parameter("CHANnel1:DISPlay", value).await
            }
            BooleanIndex::Channel1Invert => {
                self.set_boolean_parameter("CHANnel1:INVert", value).await
            }
            BooleanIndex::Channel1Vernier => {
                self.set_boolean_parameter("CHANnel1:VERNier", value).await
            }
            //
            BooleanIndex::Channel2BwLimit => {
                self.set_boolean_parameter("CHANnel2:BWLimit", value).await
            }
            BooleanIndex::Channel2Display => {
                self.set_boolean_parameter("CHANnel2:DISPlay", value).await
            }
            BooleanIndex::Channel2Invert => {
                self.set_boolean_parameter("CHANnel2:INVert", value).await
            }
            BooleanIndex::Channel2Vernier => {
                self.set_boolean_parameter("CHANnel2:VERNier", value).await
            }
            //
            BooleanIndex::TimebaseWindowEnable => {
                self.set_boolean_parameter("TIMebase:WINDow:ENABle", value)
                    .await
            }
            //
            BooleanIndex::MeasureEnable => {
                self.set_boolean_parameter("MEASure:ENABle", value).await
            }
            BooleanIndex::MeasureADisplay => {
                self.set_boolean_parameter("MEASure:ADISplay", value).await
            }
            BooleanIndex::MeasureGateEnable => {
                self.set_boolean_parameter("MEASure:GATE:ENABle", value)
                    .await
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(FromRepr, Debug, PartialEq)]
pub enum StringIndex {
    //
    Channel1Coupling,
    Channel1Scale,
    Channel1Probe,
}

#[async_trait]
///
///
impl StringAccessorModel for DSO2C10Interface {
    ///
    ///
    async fn get_string_at(&mut self, index: usize) -> Result<String, Error> {
        //
        // Get the index
        let idx = StringIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
            StringIndex::Channel1Coupling => self.get_string_parameter(b"CHANnel1:COUPling?").await,
            StringIndex::Channel1Scale => {
                let f = self.get_float_parameter(b"CHANnel1:SCALe?").await?;
                // println!("f: {}", f);
                match f {
                    0.1 => Ok("100mV".to_string()),
                    0.2 => Ok("200mV".to_string()),
                    0.5 => Ok("500mV".to_string()),
                    1.0 => Ok("1V".to_string()),
                    2.0 => Ok("2V".to_string()),
                    5.0 => Ok("5V".to_string()),
                    10.0 => Ok("10V".to_string()),
                    _ => Ok(f.to_string()),
                }
            }
            StringIndex::Channel1Probe => {
                let f = self.get_float_parameter(b"CHANnel1:PROBe?").await?;
                match f {
                    1.0 => Ok("1".to_string()),
                    10.0 => Ok("10".to_string()),
                    100.0 => Ok("100".to_string()),
                    1000.0 => Ok("1000".to_string()),
                    _ => Ok(f.to_string()),
                }
            }
        }
    }

    ///
    ///
    async fn set_string_at(&mut self, index: usize, value: &String) -> Result<(), Error> {
        //
        // Get the index
        let idx = StringIndex::from_repr(index)
            .ok_or(Error::InvalidArgument("Invalid Index".to_string()))?;

        //
        // Perform the request
        match idx {
            StringIndex::Channel1Coupling => {
                self.set_string_parameter("CHANnel1:COUPling", value).await
            }
            StringIndex::Channel1Scale => self.set_string_parameter("CHANnel1:SCALe", value).await,
            StringIndex::Channel1Probe => self.set_string_parameter("CHANnel1:PROBe", value).await,
        }
    }
}
