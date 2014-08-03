#![crate_name = "geohash"]
#![comment = "A geohash library for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]

#[cfg(test)]
use std::num::abs;

static BASE32_CODES: &'static str  = "0123456789bcdefghjkmnpqrstuvwxyz";
pub fn encode(lat: f32, lon: f32, num_chars: uint) -> String {
    let mut out: String = String::new();

    let mut bits: i8 = 0;
    let mut bits_total: i8 = 0;
    let mut hash_value: uint = 0;
    let mut max_lat = 90f32;
    let mut min_lat = -90f32;
    let mut max_lon = 180f32;
    let mut min_lon = -180f32;
    let mut mid: f32;

    let codes_vec: Vec<char> = BASE32_CODES.chars().collect();

    while out.len() < num_chars {
        if  bits_total % 2 == 0 {
            mid = (max_lon + min_lon) / 2f32;
            if lon > mid {
                hash_value = (hash_value << 1) + 1u;
                min_lon = mid;
            } else {
                hash_value = hash_value << 1;
                max_lon = mid;
            }
        } else {
            mid = (max_lat + min_lat) / 2f32;
            if lat > mid {
                hash_value = (hash_value << 1) + 1u;
                min_lat = mid;
            } else {
                hash_value = hash_value << 1;
                max_lat = mid;
            }
        }

        bits += 1;
        bits_total += 1;

        if bits == 5 {
            let code: char = codes_vec[hash_value];
            out.push_char(code);
            bits = 0;
            hash_value = 0;
        }
    }
    out
}

trait Indexable<T: Eq> {
    fn index_of(&self, item:T) -> uint;
}

impl<T: Eq> Indexable<T> for std::vec::Vec<T> {
    fn index_of(&self, item:T) -> uint {
        for c in range(0, self.len()) {
            if item == self[c] {
                return c
            }
        }
        -1
    }
}

pub fn decode_bbox(hash_str: &str) -> (f32, f32, f32, f32){
    let mut is_lon = true;
    let mut max_lat = 90f32;
    let mut min_lat = -90f32;
    let mut max_lon = 180f32;
    let mut min_lon = -180f32;
    let mut mid: f32;
    let mut hash_value: uint;

    let codes_vec: Vec<char> = BASE32_CODES.chars().collect();
    let chars: Vec<char> = hash_str.chars().collect();
    for c in chars.move_iter() {
        hash_value = codes_vec.index_of(c);

        for bs in range(0, 5) {
            let bit = (hash_value >> (4 - bs)) & 1u;
            if is_lon {
                mid = (max_lon + min_lon) / 2f32;

                if bit == 1 {
                    min_lon = mid;
                } else {
                    max_lon = mid;
                }
            } else {
                mid = (max_lat + min_lat) / 2f32;

                if bit == 1 {
                    min_lat = mid;
                } else {
                    max_lat = mid;
                }
            }
            is_lon = !is_lon;
        }
    }

    (min_lat, max_lat, min_lon, max_lon)
}

pub fn decode(hash_str: &str) -> (f32, f32, f32, f32) {
    let (lat0, lat1, lon0, lon1) = decode_bbox(hash_str);
    ((lat1+lat0)/2f32, (lon1+lon0)/2f32, (lat1-lat0)/2f32, (lon1-lon0)/2f32)
}

#[test]
fn test_encode() {
    assert_eq!(encode(37.8324f32, 112.5584f32, 9u), "ww8p1r4t8".to_string());
    assert_eq!(encode(32f32, 117f32, 3u), "wte".to_string());
}

#[test]
fn test_decode() {
    assert_eq!(decode_bbox("ww8p1r4t8"),
               (37.832367f32, 37.832409f32, 112.558365f32, 112.558411f32));
    let (lat, lon, _, _) = decode("ww8p1r4t8");
    assert_eq!(abs(lat - 37.8324f32) < 1e-4f32, true);
    assert_eq!(abs(lon - 112.5584f32) < 1e-4f32, true);
}
