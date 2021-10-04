extern crate geo_types;
extern crate geohash;

use geohash::{decode, encode, neighbors, Coordinate};

use csv;
use serde::Deserialize;

// struct to allow for deserialization
#[derive(Debug, Deserialize)]
struct TestCase {
    string_hash: String,
    lat: f64,
    long: f64,
}

#[test]
fn test_encode() {
    // use the testcases file to check encoding correctness
    let mut rdr =
        csv::Reader::from_path("tests/testcases.csv").expect("Failed to open file of test cases");
    let mut iter = rdr.deserialize();
    while let Some(result) = iter.next() {
        let record: TestCase = result.expect("Unable to deserialize record");
        let c = Coordinate {
            x: record.long,
            y: record.lat,
        };
        assert_eq!(encode(c, 12).unwrap(), record.string_hash);
    }
    // check that errors are thrown appropriately

    // should throw an error because the length is greater than 12
    let c1 = Coordinate {
        x: 117f64,
        y: 32f64,
    };
    assert!(encode(c1, 13).is_err());

    // should throw an error because the longitude is out of range
    let c2 = Coordinate {
        x: 190f64,
        y: -80f64,
    };
    assert!(encode(c2, 3usize).is_err());

    // should throw an error because the latitude is out of range
    let c3 = Coordinate {
        x: 100f64,
        y: -100f64,
    };
    assert!(encode(c3, 3usize).is_err());

    // should throw an error because the longitude is NAN
    let c4 = Coordinate {
        x: f64::NAN,
        y: 50f64,
    };
    assert!(encode(c4, 4usize).is_err());

    // should throw an error because the latitude is NAN
    let c5 = Coordinate {
        x: 100f64,
        y: f64::NAN,
    };
    assert!(encode(c5, 4usize).is_err());
}

fn compare_within(a: f64, b: f64, diff: f64) {
    assert!(
        (a - b).abs() < diff,
        "{:?} and {:?} should be within {:?}",
        a,
        b,
        diff
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
    // test decodes against the test cases file
    let diff = 1e-5f64;
    let mut rdr =
        csv::Reader::from_path("tests/testcases.csv").expect("Failed to open file of test cases");
    let mut iter = rdr.deserialize();
    while let Some(result) = iter.next() {
        let record: TestCase = result.expect("Unable to deserialize record");
        let c = decode(&record.string_hash).unwrap();
        compare_within(c.0.x, record.long, diff);
        compare_within(c.0.y, record.lat, diff);
    }

    // run these as well, since the test against the csv doesn't testt he error ranges
    compare_decode("ww8p1r4t8", 112.558386, 37.832386, 0.000021457, 0.000021457);
    compare_decode("9g3q", -99.31640625, 19.423828125, 0.17578125, 0.087890625);

    // check for errors being thrown appropriately

    // should throw an error since a is not a valid character
    assert!(decode("abcd").is_err());

    // should throw an error since the input is too long
    assert!(decode("ww8p1r4t8ww8p1r4t8").is_err());
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
