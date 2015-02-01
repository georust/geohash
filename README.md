# Rust-Geohash

Rust-Geohash is a Rust library for Geohash algorithm. Ported from [node-geohash](http://github.com/sunng87/node-geohash) module.

[![Build Status](https://travis-ci.org/georust/rust-geohash.svg)](https://travis-ci.org/georust/rust-geohash)

## Usage

### Cargo

```toml

[dependencies]

geohash = "*"
```

### Rust

```rust
extern crate geohash;

use geo::{Coordinate};
use geohash::{encode, decode};

fn main() {
    let c = Coordinate{x: 112.5584f64, y: 37.8324f64};
    println!("encoding 37.8324, 112.5584: {}", encode(c, 9u));
    let (c, _, _) = decode("ww8p1r4t8");
    println!("decoding ww8p1r4t8 to: {}, {}", c.y, c.x);
}
```

## License

Rust-Geohash is open sourced under MIT License.
