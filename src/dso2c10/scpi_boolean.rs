use std::str::FromStr;

/// SCPI Boolean type
///
pub struct ScpiBoolean {
    /// Internal representation of the boolean value
    ///
    value: bool,
}

impl ScpiBoolean {
    /// Create a new ScpiBoolean from a bool
    ///
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    /// Create a new ScpiBoolean from a string
    ///
    pub fn from_str_case_insensitive(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "on" | "1" => Ok(Self::new(true)),
            "off" | "0" => Ok(Self::new(false)),
            _ => Err(format!("Invalid boolean value: {:?}", s)),
        }
    }

    /// Create a new ScpiBoolean from a vector of u8
    ///
    pub fn from_vec_ascii(v: &Vec<u8>) -> Result<Self, String> {
        let s =
            String::from_utf8(v.clone()).map_err(|e| format!("Invalid boolean value: {:?}", e))?;
        Self::from_str_case_insensitive(s.as_str())
    }

    /// Get the value of the ScpiBoolean
    ///
    pub fn value(&self) -> bool {
        self.value
    }

    /// Get the string representation of the Boolean
    ///
    pub fn to_str(&self) -> &'static str {
        match self.value {
            true => "ON",
            false => "OFF",
        }
    }

    /// Get the string representation of the Boolean
    ///
    pub fn to_digital_str(&self) -> &'static str {
        match self.value {
            true => "1",
            false => "0",
        }
    }
}

/// Implicit conversion from bool to ScpiBoolean
///
impl From<bool> for ScpiBoolean {
    fn from(b: bool) -> Self {
        Self { value: b }
    }
}

/// Implicit conversion from ScpiBoolean to bool
///
impl From<ScpiBoolean> for bool {
    fn from(b: ScpiBoolean) -> Self {
        b.value
    }
}

/// Explicit conversion from &str to ScpiBoolean
///
impl FromStr for ScpiBoolean {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ScpiBoolean::from_str_case_insensitive(s)
    }
}
