use bigint::uint::U256;
use ed25519_fun::curve25519::field_element::FieldElement;
use ed25519_fun::curve25519::field_element::*;
use ed25519_fun::curve25519::group_element::*;
use ed25519_fun::curve25519::precomp::{precompute_double, precompute_single};
use itertools::Itertools;
use num_bigint::BigUint;
use std::ops::BitXor;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Write};

// Generates Ed25519 constants.

// * Test invert function with values predefined in
// * https://github.com/dalek-cryptography/curve25519-dalek/blob/master/src/constants.rs

/// See RFC 7748: base point.
/// FieldElement y = 4 / 5 -> 4 * 5^{-1}.
/// * Value X is wrong. This is not working atm.
/// * See module base_point.rs.
pub fn B() -> P3 {
    // m = 4
    let mut m = FieldElement([4, 0, 0, 0, 0]);

    // n = 5
    let mut n = FieldElement([5, 0, 0, 0, 0]);

    // n^{-1}
    let invert_n = n.invert();

    // m * n^{-1}
    let y = m * invert_n;

    // Encode FieldElement y
    let encoded_y = FieldElement::encode(&y);

    // Decode point with y
    let mut decoded_P3 = P3::decode(encoded_y).unwrap();

    println!(
        "X: {:020}",
        FieldElement::reduce(decoded_P3.X.0).0.iter().format("\n")
    );
    println!("Y: {:020}", decoded_P3.Y.0.iter().format("\n"));

    decoded_P3
}

/// See RFC 7748: group order.
/// L = 2^{252} + 27742317777372353535851937790883648493.
pub fn group_order() -> [u8; 32] {
    // Decimal representation
    let d = BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();

    // Hexadecimal representation
    let str_d = d.to_str_radix(16);

    // Encode FieldElement
    let mut encoded = [0u8; 32];

    // Set bit 2^{252}
    encoded[0] = 0x10;

    // Fill bits [16, 32)
    for i in 0..16 {
        encoded[i + 16] = u8::from_str_radix(&str_d[i * 2..i * 2 + 2], 16).unwrap();
    }

    // Show content of encoded.
    println!("L: {:02x}", encoded.iter().format("|"));

    encoded
}

/// See RFC 7748.
/// D = - 121665 / 121666
pub fn D() -> FieldElement {
    // m = 121665
    let m = FieldElement([0x1DB41, 0x00, 0x00, 0x00, 0x00]);

    // n = 121666
    let n = FieldElement([0x1DB42, 0x00, 0x00, 0x00, 0x00]);

    // n^{-1}
    let mut invert_n = n.invert();

    // m * n^{-1}
    let mut D = m * invert_n;

    // D = -121665 / 121666
    let zero = FieldElement([0, 0, 0, 0, 0]);
    D = zero - D;

    println!("D: {:020}", D.0.iter().format("\n"));

    D
}

/// 2 * (- 121665 / 121666) <-> 2 * D
pub fn D2() -> FieldElement {
    let d = D();

    let mut d2 = d + d;

    d2 = FieldElement::reduce(d2.0);
    println!("D2: {:020}", d2.0.iter().format("\n"));
    d2
}

/// FieldElement 0 - 1.
pub fn minus_one() -> FieldElement {
    let zero = FieldElement([0, 0, 0, 0, 0]);
    let one = FieldElement([1, 0, 0, 0, 0]);

    let minus_one = zero - one;
    println!("{:020}", minus_one.0.iter().format("\n"));

    minus_one
}

// sqrt(-1)
// * This needs attention in the future.
pub fn I() {
    // constant
    let m_one = minus_one();
    // sqrt(m_one)
}

// * For precomp_single and double use base point provided by
// * other libraries instead of calling the function B().
const base_pt: P3 = P3 {
    X: FieldElement([
        1738742601995546,
        1146398526822698,
        2070867633025821,
        562264141797630,
        587772402128613,
    ]),
    Y: FieldElement([
        1801439850948184,
        1351079888211148,
        450359962737049,
        900719925474099,
        1801439850948198,
    ]),
    Z: FieldElement([1, 0, 0, 0, 0]),
    T: FieldElement([
        1841354044333475,
        16398895984059,
        755974180946558,
        900171276175154,
        1821297809914039,
    ]),
};

// * This worked, some values are different probably because
// * we performed reduction.
// ! This may generate problems. -- This problem was fixed.
pub fn precomp_single() {
    let single: [[Precomp; 8]; 32] = precompute_single(base_pt);
    let mut concat = String::new();
    for i in 0..32 {
        concat.push_str("//Auto generated file.\n");
        concat.push_str("[\n");
        println!("[");
        for j in 0..8 {
            concat.push_str("Precomp {\nYpX: FieldElement(\n[");
            println!("Precomp {{YpX: FieldElement([");

            let ypx = format!("{}", single[i][j].YpX.0.iter().format(","));
            concat.push_str(&ypx[..]);
            println!("{}", single[i][j].YpX.0.iter().format(","));

            concat.push_str("]),\nYmX: FieldElement(\n[");
            println!("]),\nYmX: FieldElement(\n[");

            let ymx = format!("{}", single[i][j].YmX.0.iter().format(","));
            concat.push_str(&ymx[..]);
            println!("{}", single[i][j].YmX.0.iter().format(","));

            concat.push_str("]),\nXY2d: FieldElement(\n[");
            println!("]), XY2d: FieldElement([");

            let xy2d = format!("{}", single[i][j].XY2d.0.iter().format(","));
            concat.push_str(&xy2d[..]);
            println!("{}", single[i][j].XY2d.0.iter().format(","));
            concat.push_str("]),\n},\n");
            println!("]),}},");
        }
        concat.push_str("],\n");
        println!("],");
    }
    fs::write("./src/precomp_single.rs", concat.into_bytes())
        .unwrap_or_else(|_| panic!("Could not write file: {}", "precomp.rs"));
}

// Store in a file later.
pub fn precomp_double() {
    let double: [Precomp; 8] = precompute_double(base_pt);
    for i in 0..8 {
        print!("Precomp {{\nYpX: FieldElement([");
        println!("{}", double[i].YpX.0.iter().format(","));
        println!("]),");
        println!("YmX: FieldElement([");
        println!("{}", double[i].YmX.0.iter().format(","));
        println!("]), XY2d: FieldElement([");
        println!("{}", double[i].XY2d.0.iter().format(","));
        println!("]),");
        print!("}},");
    }
}

pub fn test() {
    let m = FieldElement([10, 0, 0, 0, 0]);
    let n = FieldElement([10, 0, 0, 0, 0]);

    let mut d = m * n.invert();

    d = FieldElement::reduce(d.0);

    println!("{:020}", d.0.iter().format("\n"));
}

// * From https://github.com/str4d/ed25519-java

// /* base[i][j] = (j+1) * (256^i) * P */ (2*256P 3*256P)
//   (2*256^2P 3*256^3P) (8*256^)
// (8*256^31*P)
// a[32] = a[0] + 256*a[1] + 256^2*a[2]...+ 256^31*a[31]
// a[0] = (1010|1010) = 1010 * (256P) + 1010 * (256P)
// ! private GroupElement[][] precomputeSingle() {
//     // Precomputation for single scalar multiplication.
//    ! GroupElement[][] precmp = new GroupElement[32][8];
//   !  // TODO-CR BR: check that this == base point when the method is called.
//   !  GroupElement Bi = this;
//    ! for (int i = 0; i < 32; i++) {
//    !     GroupElement Bij = Bi; // Second: 256P, oh ok, so thats what means multiplications by a huge scalar
//    !     for (int j = 0; j < 8; j++) {
//    !         final FieldElement recip = Bij.Z.invert(); //
//    !         final FieldElement x = Bij.X.multiply(recip);
//    !         final FieldElement y = Bij.Y.multiply(recip);
//    !         precmp[i][j] = precomp(this.curve, y.add(x), y.subtract(x), x.multiply(y).multiply(this.curve.get2D()));
//    !         Bij = Bij.add(Bi.toCached()).toP3();
//    !         // 1ª: Bi = P;  -> 2P....... 2ª: Bij = 2P -> 2p +         //
//         }
//         // Only every second summand is precomputed (16^2 = 256)
//         for (int k = 0; k < 8; k++) { // Bi = 2^8 = 256P = ok makes sense.
//             Bi = Bi.add(Bi.toCached()).toP3();
//         }
//     }
//     return precmp;
// }

// !private GroupElement[] precomputeDouble() {
// Precomputation for double scalar multiplication.
// P,3P,5P,7P,9P,11P,13P,15P

// recip = Z ^ (-1)
// x = X * Z^-1
// y = Y * Z^-1
// X' = y + x
// Y' = y - x
// Z' = x * y*2D
// this = P; Bi = P'
// Bi = this + (this + Bi) (first loop: Bi = P, results in Bi' = 3P)

// Basically, we're calculating multiples of a point in P3.

//     GroupElement[] dblPrecmp = new GroupElement[8];
//     GroupElement Bi = this;
//     for (int i = 0; i < 8; i++) {
//         final FieldElement recip = Bi.Z.invert();
//         final FieldElement x = Bi.X.multiply(recip);
//         final FieldElement y = Bi.Y.multiply(recip);
//         dblPrecmp[i] = precomp(this.curve, y.add(x), y.subtract(x), x.multiply(y).multiply(this.curve.get2D()));
//         // Bi = edwards(B,edwards(B,Bi))
//         Bi = this.add(this.add(Bi.toCached()).toP3().toCached()).toP3();
//     }
//     return dblPrecmp;
// }
