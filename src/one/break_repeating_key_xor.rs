extern crate rustc_serialize as serialize;
use self::serialize::hex::{ToHex};
use self::serialize::base64::{STANDARD, ToBase64};

use std::iter::Zip;
use std::slice::Iter;

pub fn compute_hamming_distance(string_a: &str, string_b: &str) -> u32 {
    let string_a_bytes: Vec<u8> = string_a.bytes().collect();
    let string_b_bytes: Vec<u8> = string_b.bytes().collect();
    xor(&string_a_bytes, &string_b_bytes)
        .iter()
        .fold(0, |total, byte| total + byte.count_ones())
}

fn xor(vec_a: &Vec<u8>, vec_b: &Vec<u8>) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}
