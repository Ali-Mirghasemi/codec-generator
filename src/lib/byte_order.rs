use std::str::FromStr;
use serde::Deserialize;

use super::errors::{
    Error, ErrorKind,
};


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum ByteOrder {
    #[serde(alias = "big_endian", alias = "big-endian", alias = "bigendain", alias = "big")]
    BigEndian,
    #[serde(alias = "little_endian", alias = "little-endian", alias = "littleendian", alias = "little")]
    LittleEndian,
}

impl FromStr for ByteOrder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "big" | "bigendain" | 
            "big_endian" | "big-endian" => ByteOrder::BigEndian,
            "little" | "littleendian" | 
            "little_endian" | "little-endian" => ByteOrder::LittleEndian,  
            _ => return Err(Error::new(ErrorKind::InvalidByteOrder, s)),
        })
    }
}

impl Default for ByteOrder {
    fn default() -> Self {
        ByteOrder::BigEndian
    }
}

