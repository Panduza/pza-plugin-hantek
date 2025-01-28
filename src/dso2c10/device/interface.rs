use async_trait::async_trait;
use panduza_platform_core::connector::usb::tmc::Driver as UsbTmcInterface;

use panduza_platform_core::std::attribute::idn::IdnReader;
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
        if response[0] == b'0' {
            Ok(false)
        } else if response[0] == b'1' {
            Ok(true)
        } else {
            Err(Error::InternalLogic(
                "Cannot parse the response".to_string(),
            ))
        }
    }

    ///
    ///
    pub async fn get_channel_display(&self, channel_id: usize) -> Result<bool, Error> {
        let cmd_string = format!("CHANnel{}:DISPlay?", channel_id);
        self.get_boolean_parameter(cmd_string.as_bytes()).await
    }

    // CHANnel<n>:BWLimit
    // CHANnel<n>:COUPling
    // CHANnel<n>:DISPlay
    // CHANnel<n>:INVert
    // CHANnel<n>:OFFSet
    // CHANnel<n>:SCALe
    // CHANnel<n>:PROBe
    // CHANnel<n>:VERNier
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
        //
        #[cfg(feature = "measure-perfs")]
        log_info!(self.logger, "ASK <=> IDN - {:.2?}", start.elapsed());

        //
        // End
        match String::from_utf8(response) {
            Ok(s) => Ok(s),
            Err(_) => Ok("Cannot convert the payload into string".to_string()),
        }
    }
}
