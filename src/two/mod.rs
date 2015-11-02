extern crate rustc_serialize as serialize;
extern crate openssl;

pub mod an_ecb_cbc_detection_oracle;

use ::shared::{open_file};
use self::serialize::base64::{FromBase64};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt, Decrypt};
use std::slice::Iter;
use std::iter::Zip;

pub fn pad_pkcs_7(block: &mut Vec<u8>, block_size: u8) {
    // "YELLOW SUBMARINE" padded to 20 bytes is...
    // "YELLOW SUBMARINE\x04\x04\x04\x04"

    let length = block.len() as u8;
    let padding_length = block_size - (length % block_size);

    for _ in (0..padding_length) {
        block.push(padding_length as u8);
    }
}

pub fn cbc_mode() {
    decrypt_cbc();
}

fn xor(vec_a: &[u8], vec_b: &[u8]) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

fn encrypt_plaintext(key: &Vec<u8>, plaintext: &[u8], v: &Vec<u8>) -> Vec<u8> {
    let xored = xor(plaintext, v);
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, &key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&xored[..])
}

fn decrypt_cbc() {
    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
    let iv: Vec<u8> = vec![0; 16];
    let iv2: [u8; 16] = [0; 16];
    let secret_message_bytes: Vec<u8> = "Bacon ipsum dolr amet short loin".bytes().collect();

    let encrypted: Vec<Vec<u8>> = secret_message_bytes.chunks(16).fold(vec!(), |acc, plaintext| {
        if let Some(v) = acc.last() {
            let cipher_text = encrypt_plaintext(&key, plaintext, &v);
            let mut acc_copy = acc.clone();
            acc_copy.push(cipher_text);
            return acc_copy;
        } else {
            let cipher_text = encrypt_plaintext(&key, plaintext, &iv);
            vec!(cipher_text)
        }
    });

    println!("{:?}", encrypted);

    let file_string: String = open_file("assets/10.txt").unwrap();
    let file_bytes: Vec<u8> = file_string.from_base64().unwrap();

    // decrypted `encrypted` from above
    //let decrypted: (Vec<Vec<u8>>, &[u8]) = encrypted.iter().fold((vec!(), &iv2), |acc, ciphertext| {
    let decrypted: (Vec<Vec<u8>>, &[u8]) = file_bytes.chunks(16).fold((vec!(), &iv2), |acc, ciphertext| {
        let (results, iv) = acc;
        let decrypter = Crypter::new(AES_128_ECB);
        decrypter.init(Decrypt, &key, &vec![0]);
        decrypter.pad(false);
        let decrypted = decrypter.update(&ciphertext);
        let plaintext = xor(&decrypted, iv);
        let mut plaintext_results = results.clone();
        plaintext_results.push(plaintext);
        (plaintext_results, ciphertext)
    });

    let (plaintext, _) = decrypted;
    let flattened: Vec<u8> = plaintext.into_iter().flat_map(|a| a).collect();
    println!("{:?}", String::from_utf8(flattened));
}

