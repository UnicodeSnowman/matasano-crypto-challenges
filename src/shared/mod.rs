use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::iter::Zip;
use std::slice::Iter;
use std::collections::HashMap;

pub fn open_file(path: &str) -> io::Result<String> {
    let mut file_string = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut file_string));
    Ok(file_string)
}

pub fn xor(vec_a: &[u8], vec_b: &[u8]) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

pub fn detect_ecb(ciphertext: &Vec<u8>) -> bool {
    let mut cipher_counts: HashMap<&[u8], u8> = HashMap::new();
    for chunk in ciphertext.chunks(16) {
        if cipher_counts.contains_key(chunk) {
            if let Some(count) = cipher_counts.get_mut(chunk) {
                *count += 1;
            }
        } else {
            cipher_counts.insert(chunk, 0);
        }
    }
    let counts: Vec<&u8> = cipher_counts.values().collect();
    let total: u8 = counts.iter().fold(0, |sum, val| sum + *val);
    total > 1
}
