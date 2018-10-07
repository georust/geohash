extern crate geo_types;
extern crate geohash;

use geohash::{decode, encode, neighbors, Coordinate};

#[test]
fn test_encode() {
    let c0 = Coordinate {
        x: 112.5584f64,
        y: 37.8324f64,
    };
    assert_eq!(encode(c0, 9usize).unwrap(), "ww8p1r4t8".to_string());
    let c1 = Coordinate {
        x: 117f64,
        y: 32f64,
    };
    assert_eq!(encode(c1, 3usize).unwrap(), "wte".to_string());

    let c2 = Coordinate {
        x: 190f64,
        y: -100f64,
    };
    assert!(encode(c2, 3usize).is_err());
}

fn compare_within(a: f64, b: f64, diff: f64) {
    assert!(
        (a - b).abs() < diff,
        format!("{:?} and {:?} should be within {:?}", a, b, diff)
    );
}

fn compare_decode(gh: &str, exp_lon: f64, exp_lat: f64, exp_lon_err: f64, exp_lat_err: f64) {
    let (coord, lon_err, lat_err) = decode(gh).unwrap();
    let diff = 1e-5f64;
    compare_within(lon_err, exp_lon_err, diff);
    compare_within(lat_err, exp_lat_err, diff);
    compare_within(coord.x, exp_lon, diff);
    compare_within(coord.y, exp_lat, diff);
}

#[test]
fn test_decode() {
    compare_decode("ww8p1r4t8", 112.558386, 37.832386, 0.000021457, 0.000021457);
    compare_decode("9g3q", -99.31640625, 19.423828125, 0.17578125, 0.087890625);

    assert!(decode("abcd").is_err());
}

#[test]
fn test_neighbor() {
    let ns = neighbors("ww8p1r4t8").unwrap();
    assert_eq!(ns.sw, "ww8p1r4mr");
    assert_eq!(ns.s, "ww8p1r4t2");
    assert_eq!(ns.se, "ww8p1r4t3");
    assert_eq!(ns.w, "ww8p1r4mx");
    assert_eq!(ns.e, "ww8p1r4t9");
    assert_eq!(ns.nw, "ww8p1r4mz");
    assert_eq!(ns.n, "ww8p1r4tb");
    assert_eq!(ns.ne, "ww8p1r4tc");
}

#[test]
fn test_neighbor_wide() {
    let ns = neighbors("9g3m").unwrap();
    assert_eq!(ns.sw, "9g3h");
    assert_eq!(ns.s, "9g3k");
    assert_eq!(ns.se, "9g3s");
    assert_eq!(ns.w, "9g3j");
    assert_eq!(ns.e, "9g3t");
    assert_eq!(ns.nw, "9g3n");
    assert_eq!(ns.n, "9g3q");
    assert_eq!(ns.ne, "9g3w");
}
