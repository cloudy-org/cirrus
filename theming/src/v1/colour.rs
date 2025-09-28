use cirrus_error::v1::error::CError;

use crate::v1::error::Error;

#[derive(Clone)]
pub struct Colour {
    pub hex: u32
}

impl Colour {
    pub fn from_hex(hex: u32) -> Self {
        Self { hex }
    }

    pub fn as_hex_string(&self) -> String {
        format!("#{:06x}", self.hex)
    }
}

impl From<u32> for Colour {
    fn from(value: u32) -> Self {
        Self::from_hex(value)
    }
}

impl TryFrom<&str> for Colour {
    type Error = Box<dyn CError>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let formatted_hex_code = value.replace("#", "");

        let hex_code: u32 = u32::from_str_radix(&formatted_hex_code, 16)
            .map_err(|error| Error::FailedToParseHexCode(error.to_string()))?;

        Ok(Self::from_hex(hex_code))
    }
}