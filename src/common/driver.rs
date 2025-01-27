use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{log_trace, Error, InstanceLogger};
use std::time::Instant;

///
///
///
pub struct KoradDriver<SD> {
    /// Physical level driver
    ///
    driver: SD,

    /// Logger for the driver
    ///
    logger: InstanceLogger,
}

impl<SD: AsciiCmdRespProtocol> KoradDriver<SD> {
    ///
    /// Create a new driver
    ///
    pub fn new(driver: SD, logger: InstanceLogger) -> Self {
        Self {
            driver: driver,
            logger: logger,
        }
    }

    ///
    /// Get identity string
    ///
    pub async fn get_idn(&mut self) -> Result<String, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = "*IDN?".to_string();
        let response = self.driver.ask(&cmd).await?;

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
        Ok(response)
    }

    ///
    /// Control current getter
    ///
    pub async fn get_iset(&mut self) -> Result<f32, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = "ISET1?".to_string();
        let response = self.driver.ask(&cmd).await?;

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
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_iset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("ISET1:{:.3}", value);
        self.driver.send(&cmd).await
    }

    ///
    /// Control current getter
    ///
    pub async fn get_vset(&mut self) -> Result<f32, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        // Perform request
        let cmd = "VSET1?".to_string();
        let response = self.driver.ask(&cmd).await?;

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
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    /// Control current getter
    ///
    pub async fn set_vset(&mut self, value: f32) -> Result<(), Error> {
        let cmd = format!("VSET1:{:.2}", value);
        self.driver.send(&cmd).await
    }

    ///
    ///
    ///
    pub async fn get_iout(&mut self) -> Result<f32, Error> {
        let cmd = "IOUT1?".to_string();
        let response = self.driver.ask(&cmd).await?;
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    ///
    ///
    pub async fn get_vout(&mut self) -> Result<f32, Error> {
        let cmd = "VOUT1?".to_string();
        let response = self.driver.ask(&cmd).await?;
        let value = response
            .parse::<f32>()
            .map_err(|e| Error::Generic(format!("{:?}", e)))?;
        Ok(value)
    }

    ///
    ///
    ///
    pub async fn set_out(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = "OUT1".to_string();
                self.driver.send(&cmd).await
            }
            false => {
                let cmd = "OUT0".to_string();
                self.driver.send(&cmd).await
            }
        }
    }

    ///
    ///
    ///
    pub async fn get_out(&mut self) -> Result<bool, Error> {
        //
        // Measure perfs
        let start = Instant::now();

        //
        //
        let cmd = "STATUS?".to_string();
        let response = self.driver.ask(&cmd).await?;

        //
        // Log
        log_trace!(
            self.logger,
            "ASK <=> {:?} - {:?} (as bytes/{:?}) - {:.2?}",
            cmd,
            response,
            response.as_bytes(),
            start.elapsed()
        );

        let byte = response.as_bytes()[0];
        if (byte & (1 << 6)) == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    ///
    ///
    ///
    pub async fn set_beep(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = "BEEP1".to_string();
                self.driver.send(&cmd).await
            }
            false => {
                let cmd = "BEEP0".to_string();
                self.driver.send(&cmd).await
            }
        }
    }

    ///
    ///
    ///
    pub async fn get_beep(&mut self) -> Result<bool, Error> {
        let cmd = "STATUS?".to_string();
        let response = self.driver.ask(&cmd).await?;
        let byte = response.as_bytes()[0];
        if (byte & (1 << 4)) == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    ///
    ///
    ///
    pub async fn set_ocp(&mut self, value: bool) -> Result<(), Error> {
        //
        // Prepare command
        let mut cmd = "OCP0".to_string();
        if value {
            cmd = "OCP1".to_string();
        }

        //
        // Send
        log_trace!(self.logger, "SEND => {:?}", cmd);
        self.driver.send(&cmd).await
    }

    ///
    ///
    ///
    pub async fn set_ovp(&mut self, value: bool) -> Result<(), Error> {
        match value {
            true => {
                let cmd = "OVP1".to_string();
                self.driver.send(&cmd).await
            }
            false => {
                let cmd = "OVP0".to_string();
                self.driver.send(&cmd).await
            }
        }
    }
}
