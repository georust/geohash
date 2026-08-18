use alloc::string::String;
use core::{error::Error, fmt};

use crate::Coord;

#[derive(Debug)]
pub enum GeohashError {
    InvalidHashCharacter(char),
    InvalidCoordinateRange(Coord<f64>),
    InvalidLength(usize),
    InvalidHash(String),
}

impl fmt::Display for GeohashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeohashError::InvalidHashCharacter(c) => write!(f, "invalid hash character: {}", c),
            GeohashError::InvalidCoordinateRange(c) => {
                write!(f, "invalid coordinate range: {:?}", c)
            }
            GeohashError::InvalidLength(len) => {
                #[cfg(feature = "wide")]
                let maximum = crate::core::WIDE_LEN_RANGE.end - 1;
                #[cfg(not(feature = "wide"))]
                let maximum = crate::core::LEN_RANGE.end - 1;
                write!(
                    f,
                    "Invalid length specified: {}. Accepted values are between 1 and {}, inclusive",
                    len, maximum
                )
            }
            GeohashError::InvalidHash(msg) => write!(f, "Invalid input hash: {}", msg),
        }
    }
}

impl Error for GeohashError {}
