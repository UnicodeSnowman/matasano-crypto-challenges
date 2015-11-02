extern crate openssl;
extern crate rand;
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt, Decrypt};
use self::openssl::crypto::rand::{rand_bytes};
use self::rand::Rng;
use std::iter::Zip;
use std::slice::Iter;

pub enum EncryptionType { CBC, ECB }

// Write a function to generate a random AES key;
// that's just 16 random bytes.'
pub fn generate_random_aes_key() -> Vec<u8> {
    rand_bytes(16)
}

fn xor(vec_a: &[u8], vec_b: &[u8]) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

fn append(data: &mut Vec<u8>, append: Vec<u8>) {
    for byte in append.into_iter() {
        data.push(byte);
    }
}

fn encrypt_cbc(data: &[u8]) -> Vec<u8> {
    let key = generate_random_aes_key();
    let iv: Vec<u8> = vec![0; 16];
 
    let (encrypted, _): (Vec<Vec<u8>>, Vec<u8>) = data.chunks(16).fold((vec!(), iv), |acc, plaintext| {
        let (ciphertexts, iv) = acc;
        let xored = xor(&plaintext, &iv);
        let ciphertext = encrypt(&xored, &key);
        let mut ciphertexts_clone = ciphertexts.clone();
        ciphertexts_clone.push(ciphertext.clone());
        (ciphertexts_clone, ciphertext)
    });

    let flattened: Vec<u8> = encrypted.into_iter().flat_map(|a| a).collect();
    flattened
}

fn encrypt_ecb(data: &[u8]) -> Vec<u8> {
    let key = generate_random_aes_key();
    encrypt(data, &key)
}

fn encrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&data)
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
        0 => Some(encrypt_cbc(&data)),
        1 => Some(encrypt_ecb(&data)),
        _ => None
    }
}

pub fn detect_mode(input: Vec<u8>) -> EncryptionType {
    EncryptionType::CBC
}
