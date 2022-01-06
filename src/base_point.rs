use bigint::uint::U256;
// TODO Make this public
use ed25519_fun::curve25519::field_element::FieldElement;
use ed25519_fun::curve25519::field_element::*;
use ed25519_fun::curve25519::group_element::*;
use itertools::Itertools;
use num_bigint::BigUint;
use std::ops::BitXor;

// TODO Make this public
use ed25519_compact::curve25519::*;

// Ed25519 base point.

pub fn base_point_v1() {
    // m = 4
    let mut m = FieldElement([4, 0, 0, 0, 0]);

    // n = 5
    let mut n = FieldElement([5, 0, 0, 0, 0]);

    // n^{-1}
    let invert_n = n.invert();
    println!("Inverted n: {:020}", invert_n.0.iter().format("\n"));

    // m * n^{-1}
    let y = m * invert_n;
    println!("m * n-1 n: {:020}", y.0.iter().format("\n"));

    // Encode FieldElement y
    let encoded_y = FieldElement::encode(&y);
    println!("Encoded y: {}", encoded_y.iter().format("\n"));

    // Decode point with y
    let mut decoded_P3 = P3::decode(encoded_y).unwrap();
    println!("P3 point: {:020}", decoded_P3.X.0.iter().format("-/-"));
    // println!("X: {:020}", FieldElement::reduce(decoded_P3.X.0).0.iter().format("\n"));
    // println!("Y: {:020}", decoded_P3.Y.0.iter().format("\n"));
}

pub fn base_point_v2() {
    // m = 4
    let mut m = Fe([4, 0, 0, 0, 0]);

    // n = 5
    let mut n = Fe([5, 0, 0, 0, 0]);

    // n^{-1}
    let invert_n = n.invert();
    println!("Inverted n: {:020}", invert_n.0.iter().format("\n"));
    // m * n^{-1}
    let y = m * invert_n;
    println!("m * n-1 n: {:020}", y.0.iter().format("\n"));

    // Encode FieldElement y
    let mut out = [0u8; 32];
    fiat_25519_to_bytes(&mut out, &y.0);
    println!("Encoded y: {:020}", out.iter().format("-/-"));

    // Decode point with y
    let mut decoded_P3 = GeP3::from_bytes_negate_vartime(&out).unwrap();
    println!("P3 point: {:020}", decoded_P3.x.0.iter().format("-/-"));

    // println!("X: {:020}", FieldElement::reduce(decoded_P3.X.0).0.iter().format("\n"));
    // println!("Y: {:020}", decoded_P3.Y.0.iter().format("\n"));
}
