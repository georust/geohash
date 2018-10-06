//! Geohash algorithm implementation in Rust
//!
//!
extern crate geo_types;
#[cfg(test)]
extern crate num_traits;
#[macro_use]
extern crate failure;

mod core;
mod error;
mod neighbors;

pub use core::{decode, decode_bbox, encode, neighbors};
pub use error::GeohashError;
pub use geo_types::Coordinate;
pub use neighbors::Neighbors;
