extern crate openssl;
extern crate rustc_serialize as serialize;
extern crate rand;
use self::serialize::base64::{FromBase64};
use std::collections::HashMap;
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt};
use ::shared::{detect_ecb};

static UNKNOWN_STRING: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";

fn encrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&data)
}

fn append(data: &mut Vec<u8>, append: Vec<u8>) {
    for byte in append.into_iter() {
        data.push(byte);
    }
}

pub fn encryption_oracle(input: &Vec<u8>) -> Vec<u8> {
    let consistent_key = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    let mut data: Vec<u8> = vec!();
    let input_copy = input.clone();

    let unknown_base64_string = UNKNOWN_STRING.from_base64().unwrap();

    append(&mut data, input_copy);
    append(&mut data, unknown_base64_string);
    encrypt(&data, &consistent_key)
}

fn gen_n_bytes(n: usize) -> Vec<u8> {
    let gen_string: String = (0..n).map(|_| "A").collect();
    gen_string.bytes().collect()
}

fn detect_blocksize() -> usize {
    // send strings to the cipher, increase by a single character each time
    // if the length of the returned cipher changes, the difference between
    // the new length and the old length is the block size
    let base_length = encryption_oracle(&"A".bytes().collect()).len();
    for val in 2..64 {
        let character_string_bytes: Vec<u8> = gen_n_bytes(val);
        let encrypted = encryption_oracle(&character_string_bytes);
        let length = encrypted.len();
        if length > base_length {
            return length - base_length;
        }
    }
    base_length
}

pub fn main(msg: &Vec<u8>) {
    let encrypted = encryption_oracle(msg);
    let is_ecb = detect_ecb(&encrypted);
    let blocksize = detect_blocksize();
    let mut results: Vec<u8> = vec!();

    if is_ecb {
        let mut results_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

        for s in (1..blocksize - 1) {
            let mut ciphertext = encryption_oracle(&gen_n_bytes(blocksize - s));
            ciphertext.truncate(16);

            for i in 0..255 {
                let byte = i as u8;
                let mut vec = gen_n_bytes(blocksize - s);
                // stick our results from previous iterations
                // on our vector
                for b in results.clone() {
                    vec.push(b);
                }
                vec.push(byte);
                let mut ctext = encryption_oracle(&vec);
                ctext.truncate(16);
                results_map.insert(ctext, vec);
            }

            let answer = results_map.get(&ciphertext);
            match answer {
                Some(vec) => { 
                    if let Some(value) = vec.last() {
                        results.push(*value);
                    }
                },
                None => println!("{:?}", "Not Found")
            }
        }

        let result_string = String::from_utf8(results);
        println!("{:?}", result_string);

    }
}

