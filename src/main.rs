mod base_point;
mod binary_tests;
mod ed25519_constants;

extern crate bigint;
extern crate ed25519_compact;
extern crate ed25519_fun;
extern crate itertools;
use itertools::Itertools;
extern crate hex;
extern crate minisign;
extern crate num_bigint;

use base_point::*;
use binary_tests::*;
use ed25519_constants::*;
use ed25519_fun::*;
// TODO Make crypto module public
use minisign::crypto::ed25519;
use num_bigint::BigUint;

pub fn not_the_serializer(sk: &str) -> [u8; 32] {
    let mut sk_bytes = [0u8; 32];
    for i in 0..32 {
        sk_bytes[i] = u8::from_str_radix(&sk[i * 2..i * 2 + 2], 16).unwrap();
    }

    sk_bytes
}

fn main() {
    let message: Vec<u8> = hex::decode("18b6bec097").unwrap();
    let sk: [u8; 32] =
        not_the_serializer("b780381a65edf8b78f6945e8dbec7941ac049fd4c61040cf0c324357975a293c");

    // minisign
    let (sk1_exp, pk1): ([u8; 64], [u8; 32]) = ed25519::keypair(&sk);
    let sig1: [u8; 64] = ed25519::signature(&message, &sk1_exp, None);
    let verify1: bool = ed25519::verify(&message, &pk1, &sig1);

    // ed25519-fun
    let fun_sk = SecretKey::from_bytes(&sk).unwrap();
    let fun_keypair = Keypair::generate_public_key(fun_sk);
    let pk2: [u8; 32] = fun_keypair.public.as_bytes();
    let fun_sig: Signature = fun_keypair.sign(&message);
    let sig2: [u8; 64] = fun_sig.as_bytes();
    let verify2 = fun_keypair.verify(&message, fun_sig);
    // println!("SK: {:02x}", fun_sk.as_bytes().iter().format("|"));

    println!("{:?} {:?}", verify2, verify1);
    assert!(pk1 == pk2, "Public keys are not equal.");
    assert!(sig1 == sig2, "Signatures are not equal.");
}
