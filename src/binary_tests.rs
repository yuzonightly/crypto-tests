// Understand conversion from i8 to u8 and
// i8 to absolute value.
// Ex: |-8| = 8
//     |+8| = 8
pub fn i8_as_u8() {
    let a: i8 = -9;
    println!("binary: {:#010b}", a - (a << 1));
    // a - (a << 1) returns the absolute value when a is negative
    // if a is positive then a - 0 is performed.

    // let babs: u8 = (b - (((-(bnegative as i8)) & b) << 1)) as u8;
}

pub fn as_binary() {
    let a: u8 = 255;
    println!("binary: {:#010b}", a as i8);
    // println!("i8 decimal: {}", a as u8);
}

pub fn shift_precedence() {
    let mut a: u8 = 1;
    let b: u8 = 1;
    a += b << 1;
    println!("binary: {:#010b}", a);
    // println!("i8 decimal: {}", a as u8);
}
