#![doc(html_root_url = "https://docs.rs/geohash/")]

//! # Geohash
//!
//! Geohash algorithm implementation in Rust. It encodes/decodes a
//! longitude-latitude tuple into/from a hashed string. You can find
//! more about geohash algorithm on [Wikipedia](https://en.wikipedia.org/wiki/Geohash)
//!
//! ## Usage
//! ```rust
//! extern crate geohash;
//!
//! use std::error::Error;
//!
//! use geohash::{encode, decode, neighbor, Direction, Coord};
//!
//! fn main() -> Result<(), Box<Error>> {
//!   // encode a coordinate
//!   let c = Coord { x: 112.5584f64, y: 37.8324f64 };
//!   println!("encoding 37.8324, 112.5584: {}", encode(c, 9usize)?);
//!
//!   // decode a geohash
//!   let (c, _, _) = decode("ww8p1r4t8")?;
//!   println!("decoding ww8p1r4t8 to: {}, {}", c.y, c.x);
//!
//!   // find a neighboring hash
//!   let sw = neighbor("ww8p1r4t8", Direction::SW)?;
//!
//!   Ok(())
//! }
//! ```
//!

mod core;
mod error;
mod neighbors;

pub use crate::core::{decode, decode_bbox, encode, neighbor, neighbors};
pub use crate::error::GeohashError;
pub use crate::neighbors::{Direction, Neighbors};
pub use geo_types::{Coord, Rect};
