use std::error::Error;
use std::fmt;

use crate::Coordinate;

#[derive(Debug)]
pub enum GeohashError {
    InvalidHashCharacter(char),
    InvalidCoordinateRange(Coordinate<f64>),
}

impl fmt::Display for GeohashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeohashError::InvalidHashCharacter(c) => write!(f, "invalid hash character: {}", c),
            GeohashError::InvalidCoordinateRange(c) => {
                write!(f, "invalid coordinate range: {:?}", c)
            }
        }
    }
}

impl Error for GeohashError {}
