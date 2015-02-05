#![feature(core)]
#![cfg(test)]
#![feature(std_misc)]

extern crate geo;

use geo::Coordinate;

static BASE32_CODES: &'static [char] =
    &['0', '1', '2', '3', '4', '5', '6', '7',
      '8', '9', 'b', 'c', 'd', 'e', 'f', 'g',
      'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r',
      's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// ### Encode latitude, longitude into geohash string
///
/// Parameters:
/// * `latitude`
/// * `longitude`
/// * `num_chars`: how many characters to encode
///
/// Returns:
/// Geohash encoded `String`
pub fn encode(c: Coordinate, num_chars: usize) -> String {
    let mut out: String = String::new();

    let mut bits: i8 = 0;
    let mut bits_total: i8 = 0;
    let mut hash_value: usize = 0;
    let mut max_lat = 90f64;
    let mut min_lat = -90f64;
    let mut max_lon = 180f64;
    let mut min_lon = -180f64;
    let mut mid: f64;

    while out.len() < num_chars {
        if  bits_total % 2 == 0 {
            mid = (max_lon + min_lon) / 2f64;
            if c.x > mid {
                hash_value = (hash_value << 1) + 1us;
                min_lon = mid;
            } else {
                hash_value = hash_value << 1;
                max_lon = mid;
            }
        } else {
            mid = (max_lat + min_lat) / 2f64;
            if c.y > mid {
                hash_value = (hash_value << 1) + 1us;
                min_lat = mid;
            } else {
                hash_value = hash_value << 1;
                max_lat = mid;
            }
        }

        bits += 1;
        bits_total += 1;

        if bits == 5 {
            let code: char = BASE32_CODES[hash_value];
            out.push(code);
            bits = 0;
            hash_value = 0;
        }
    }
    out
}

trait Indexable<T: Eq> {
    fn index_of(&self, item:T) -> usize;
}

impl<'a, T: Eq> Indexable<T> for &'a [T] {
    fn index_of(&self, item:T) -> usize {
        for c in range(0, self.len()) {
            if item == self[c] {
                return c
            }
        }
        -1
    }
}

/// ### Encode latitude, longitude into geohash string
///
/// Parameters:
/// Geohash encoded `&str`
///
/// Returns:
/// A four-element tuple describs a bound box:
/// * min_lat
/// * max_lat
/// * min_lon
/// * max_lon
pub fn decode_bbox(hash_str: &str) -> (Coordinate, Coordinate){
    let mut is_lon = true;
    let mut max_lat = 90f64;
    let mut min_lat = -90f64;
    let mut max_lon = 180f64;
    let mut min_lon = -180f64;
    let mut mid: f64;
    let mut hash_value: usize;

    let chars: Vec<char> = hash_str.chars().collect();
    for c in chars.into_iter() {
        hash_value = BASE32_CODES.index_of(c);

        for bs in range(0, 5) {
            let bit = (hash_value >> (4 - bs)) & 1us;
            if is_lon {
                mid = (max_lon + min_lon) / 2f64;

                if bit == 1 {
                    min_lon = mid;
                } else {
                    max_lon = mid;
                }
            } else {
                mid = (max_lat + min_lat) / 2f64;

                if bit == 1 {
                    min_lat = mid;
                } else {
                    max_lat = mid;
                }
            }
            is_lon = !is_lon;
        }
    }

    (Coordinate{x: min_lon, y: min_lat}, Coordinate{x: max_lon, y: max_lat})
}

/// ### Encode latitude, longitude into geohash string
///
/// Parameters:
/// Geohash encoded `&str`
///
/// Returns:
/// A four-element tuple describs a bound box:
/// * latitude
/// * longitude
/// * latitude error
/// * longitude error
pub fn decode(hash_str: &str) -> (Coordinate, f64, f64) {
    let (c0, c1) = decode_bbox(hash_str);
    (Coordinate {x: (c0.x+c1.x)/2f64, y: (c0.y+c1.y)/2f64},
     (c1.y-c0.y)/2f64, (c1.x-c0.x)/2f64)
}

#[cfg(test)]
mod test {
    use ::{encode, decode};
    use geo::Coordinate;
    use std::num::Float;

    #[test]
    fn test_encode() {
        let c0 = Coordinate{x: 112.5584f64, y: 37.8324f64};
        assert_eq!(encode(c0, 9us), "ww8p1r4t8".to_string());
        let c1 = Coordinate{x: 117f64, y: 32f64};
        assert_eq!(encode(c1, 3us), "wte".to_string());
    }

    #[test]
    fn test_decode() {
        let (c, _, _) = decode("ww8p1r4t8");
        assert_eq!(Float::abs_sub(c.y, 37.8324f64) < 1e-4f64, true);
        assert_eq!(Float::abs_sub(c.x, 112.5584f64) < 1e-4f64, true);
    }
}
