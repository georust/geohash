#![feature(test)]

extern crate geo_types;
extern crate geohash;
extern crate test;

use geo_types::Coordinate;
use geohash::{
    decode, encode, neighbor, neighbors, old_decode, old_encode, old_neighbor, old_neighbors,
    Direction,
};
use test::Bencher;

#[bench]
fn benchmark_encode(b: &mut Bencher) {
    let x = 4.456758;
    let y = -11.11385758;

    let coordinate = Coordinate { x, y };

    b.iter(|| {
        encode(coordinate, 6).expect("The Coordinates were not possible");
    })
}

#[bench]
fn benchmark_old_encode(b: &mut Bencher) {
    let x = 4.456758;
    let y = -11.11385758;

    let coordinate = Coordinate { x, y };

    b.iter(|| {
        old_encode(coordinate, 6).expect("The Coordinates were not possible");
    })
}

#[bench]
fn benchmark_decode(b: &mut Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        decode(hash).expect("The hashstring was malformed");
    })
}

#[bench]
fn benchmark_old_decode(b: &mut Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        old_decode(hash).expect("The hashstring was malformed");
    })
}

#[bench]
fn benchmark_neighbor(b: &mut test::Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        neighbor(hash, Direction::N).expect("The hashstring was malformed");
    })
}

#[bench]
fn benchmark_old_neighbor(b: &mut test::Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        old_neighbor(hash, Direction::N).expect("The hashstring was malformed");
    })
}

#[bench]
fn benchmark_neighbors(b: &mut test::Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        neighbors(hash).expect("The hashstring was malformed");
    })
}

#[bench]
fn benchmark_old_neighbors(b: &mut test::Bencher) {
    let hash = "9q60y60rhs";

    b.iter(|| {
        old_neighbors(hash).expect("The hashstring was malformed");
    })
}
