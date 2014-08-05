# geohash.rs

Geohash.rs is a Rust library for Geohash algorithm. Ported from my [node-geohash](http://github.com/sunng87/node-geohash) module.

[![Build Status](https://travis-ci.org/sunng87/geohash.rs.svg)](https://travis-ci.org/sunng87/geohash.rs)

## Usage

### Cargo

```toml

[dependencies.geohash]

git = "https://github.com/sunng87/geohash.rs.git"
```

### Rust

```rust
use geohash::{encode, decode}

fn main() {
    println!("encoding 37.8324, 112.5584: {}", encode(37.8324f32, 112.5584f32, 9u));
    let (lat, lon, _, _) = decode("ww8p1r4t8");
    println!("decoding ww8p1r4t8 to: {}, {}", lat, lon);
}
```

## License

Geohash.rs is open sourced under MIT License, of course.

