use async_trait::async_trait;
use panduza_platform_core::connector::usb::tmc::Driver as UsbTmcInterface;

use panduza_platform_core::std::attribute::idn::IdnReader;
use panduza_platform_core::std::class::repl::ReplProtocol;
use panduza_platform_core::{log_info, log_trace, Error, Logger};
use tokio::sync::Mutex;

use std::sync::Arc;
use std::time::Instant;

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
        if response.len() == 1 {
            if response[0] == b'0' {
                Ok(false)
            } else if response[0] == b'1' {
                Ok(true)
            } else {
                Err(Error::InternalLogic(
                    "Cannot parse the response".to_string(),
                ))
            }
        } else if response.len() == 3 {
            let r_string = String::from_utf8(response).map_err(|_| {
                Error::InternalLogic("Cannot convert the response into string".to_string())
            })?;

            if r_string == "OFF" {
                Ok(false)
            } else if r_string == "ON" {
                Ok(true)
            } else {
                Err(Error::InternalLogic(
                    "Cannot parse the response".to_string(),
                ))
            }
        } else {
            Err(Error::InternalLogic(
                "Cannot parse the response".to_string(),
            ))
        }
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
            Err(e) => Err(Error::Generic(
                "Cannot convert response into string".to_string(),
            )),
        }
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
    pub async fn get_channel_bw_limit(&self, channel_id: usize) -> Result<bool, Error> {
        let cmd_string = format!("CHANnel{}:BWLimit?", channel_id);
        self.get_boolean_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_bw_limit(&self, channel_id: usize, value: bool) -> Result<(), Error> {
        let mut v = "0";
        if value {
            v = "1";
        }
        let cmd_string = format!("CHANnel{}:BWLimit {}", channel_id, v);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
    }

    ///
    ///
    pub async fn get_channel_coupling(&self, channel_id: usize) -> Result<String, Error> {
        let cmd_string = format!("CHANnel{}:COUPling?", channel_id);
        self.get_string_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn get_channel_display(&self, channel_id: usize) -> Result<bool, Error> {
        let cmd_string = format!("CHANnel{}:DISPlay?", channel_id);
        self.get_boolean_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_display(&self, channel_id: usize, value: bool) -> Result<(), Error> {
        let mut v = "0";
        if value {
            v = "1";
        }
        let cmd_string = format!("CHANnel{}:DISPlay {}", channel_id, v);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
    }

    ///
    ///
    pub async fn get_channel_invert(&self, channel_id: usize) -> Result<bool, Error> {
        let cmd_string = format!("CHANnel{}:INVert?", channel_id);
        self.get_boolean_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_invert(&self, channel_id: usize, value: bool) -> Result<(), Error> {
        let mut v = "0";
        if value {
            v = "1";
        }
        let cmd_string = format!("CHANnel{}:INVert {}", channel_id, v);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
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

    ///
    ///
    pub async fn get_channel_vernier(&self, channel_id: usize) -> Result<bool, Error> {
        let cmd_string = format!("CHANnel{}:VERNier?", channel_id);
        self.get_boolean_parameter(cmd_string.as_bytes()).await
    }

    ///
    ///
    pub async fn set_channel_vernier(&self, channel_id: usize, value: bool) -> Result<(), Error> {
        let mut v = "0";
        if value {
            v = "1";
        }
        let cmd_string = format!("CHANnel{}:VERNier {}", channel_id, v);
        let cmd = cmd_string.as_bytes();
        self.sub_interface.lock().await.send_command(cmd).await?;
        Ok(())
    }
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
