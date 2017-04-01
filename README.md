# Rust-Geohash

Rust-Geohash is a Rust library for Geohash algorithm. Ported from [node-geohash](http://github.com/sunng87/node-geohash) module.

[![Build Status](https://travis-ci.org/georust/rust-geohash.svg)](https://travis-ci.org/georust/rust-geohash)
[![](http://meritbadge.herokuapp.com/geohash)](https://crates.io/crates/geohash)

[Documentation](https://georust.github.io/rust-geohash/)

## Usage

```rust
extern crate geohash;

use geo::{Coordinate};
use geohash::{encode, decode, neighbor};

fn main() {
    let c = Coordinate{x: 112.5584f64, y: 37.8324f64};
    println!("encoding 37.8324, 112.5584: {}", encode(c, 9u));
    let (c, _, _) = decode("ww8p1r4t8");
    println!("decoding ww8p1r4t8 to: {}, {}", c.y, c.x);
    let sw = neighbor("ww8p1r4t8", (-1,-1));
    println!("{}", sw) // ww8p1r4mr
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
