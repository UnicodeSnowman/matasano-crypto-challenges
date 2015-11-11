extern crate openssl;
extern crate rustc_serialize as serialize;
extern crate rand;
use self::serialize::base64::{FromBase64};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt};
use self::openssl::crypto::rand::{rand_bytes};
use self::rand::Rng;
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
    let CONSISTENT_KEY = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    let mut rng = rand::thread_rng();
    let mut data: Vec<u8> = vec!();
    let input_copy = input.clone();

    let unknown_base64_string = UNKNOWN_STRING.from_base64().unwrap();

    append(&mut data, input_copy);
    append(&mut data, unknown_base64_string);
    encrypt(&data, &CONSISTENT_KEY)
}

fn detect_blocksize() -> usize {
    // send strings to the cipher, increase by a single character each time
    // if the length of the returned cipher changes, the difference between
    // the new length and the old length is the block size
    let base_length = encryption_oracle(&"A".bytes().collect()).len();
    for val in 2..64 {
        let character_string: String = (0..val).map(|_| "A").collect();
        let encrypted = encryption_oracle(&character_string.bytes().collect());
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
    println!("TEST: {:?}", blocksize);
}

