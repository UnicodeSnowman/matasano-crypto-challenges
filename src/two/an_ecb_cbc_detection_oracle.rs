extern crate openssl;
extern crate rand;
use self::openssl::crypto::rand::{rand_bytes};
use self::rand::Rng;

// Write a function to generate a random AES key;
// that's just 16 random bytes.'
pub fn generate_random_aes_key() -> Vec<u8> {
    rand_bytes(16)
}

fn append(data: &mut Vec<u8>, append: Vec<u8>) {
    for byte in append.into_iter() {
        data.push(byte);
    }
}

fn encrypt_cbc(data: Vec<u8>) -> Vec<u8> {
    let key = generate_random_aes_key();
    vec!()
}

fn encrypt_ecb(data: Vec<u8>) -> Vec<u8> {
    let key = generate_random_aes_key();
    vec!()
}

pub fn encryption_oracle(input: Vec<u8>) -> Option<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u8> = vec!();
    let n_prepend: u16 = rng.gen_range(5, 11);
    let n_append: u16 = rng.gen_range(5, 11);
    let prepend_bytes: Vec<u8> = rand_bytes(n_prepend as usize);
    let append_bytes: Vec<u8> = rand_bytes(n_append as usize);
    let input_copy = input.clone();

    append(&mut data, prepend_bytes);
    append(&mut data, input_copy);
    append(&mut data, append_bytes);

    match rng.gen_range(0, 2) {
        0 => Some(encrypt_cbc(data)),
        1 => Some(encrypt_ecb(data)),
        _ => None
    }
}
