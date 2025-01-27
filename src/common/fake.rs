use async_trait::async_trait;
use panduza_platform_core::{protocol::AsciiCmdRespProtocol, Error};

///
/// Fake Driver for power supply
///
pub struct Driver {
    output_enable: bool,
    iset: String,
    vset: String,
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open() -> Result<Self, Error> {
        Ok(Self {
            output_enable: false,
            iset: "0".to_string(),
            vset: "0".to_string(),
        })
    }
}

#[async_trait]
impl AsciiCmdRespProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error> {
        println!("Fake Send {:?}", command);

        if *command == "OUT1".to_string() {
            self.output_enable = true
        } else if *command == "OUT0".to_string() {
            self.output_enable = false
        } else if command.starts_with("VSET1:") {
            self.vset = command.replace("VSET1:", "")
        } else if command.starts_with("ISET1:") {
            self.iset = command.replace("ISET1:", "")
        } else {
            println!("Fake behaviour not supported {:?} > do nothing", command);
        }

        Ok(())
    }

    ///
    ///
    ///
    async fn ask(&mut self, command: &String) -> Result<String, Error> {
        //
        //
        println!("Fake Ask {:?}", command);
        let mut response = "1".to_string();

        //
        //
        if *command == "*IDN?".to_string() {
            response = "Fake Device !!!".to_string();
        } else if *command == "VOUT1".to_string() {
            response = "2.0".to_string();
        } else if *command == "ISET1?".to_string() {
            response = self.iset.clone();
        } else if *command == "VSET1?".to_string() {
            response = self.vset.clone();
        } else if *command == "STATUS?".to_string() {
            let mut status: u8 = 0;
            if self.output_enable {
                status = status | (1 << 6);
            }
            response = format!("{}", status as char);
        } else {
            println!("Fake behaviour not supported {:?} > do nothing", command);
        }

        println!("Fake Ask - return {:?}", response);
        return Ok(response);
    }
}
