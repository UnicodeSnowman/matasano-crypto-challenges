extern crate rustc_serialize as serialize;

use std::iter::Zip;
use std::slice::Iter;

pub fn compute_hamming_distance(bytes_a: &Vec<u8>, bytes_b: &Vec<u8>) -> u32 {
    xor(bytes_a, bytes_b)
        .iter()
        .fold(0, |total, byte| total + byte.count_ones())
}

fn xor(vec_a: &Vec<u8>, vec_b: &Vec<u8>) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}
