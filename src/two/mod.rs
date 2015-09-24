extern crate rustc_serialize as serialize;
extern crate openssl;

use ::shared::{open_file};
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{encrypt, decrypt};

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
    encrypt_cbc();
}

fn encrypt_cbc() {
    let file_string: String = open_file("assets/10.txt").unwrap();
    let file_bytes: Vec<u8> = file_string.from_base64().unwrap();

    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
    let iv: Vec<u8> = vec![0; 10];

    let vecs: Vec<Vec<u8>> = file_bytes.chunks(16).fold(vec!(iv), |acc, chunk| {
        if let Some(v) = acc.last() {
           let encrypted_chunk = encrypt(AES_128_ECB, &key, v, chunk);
           let mut v = acc.clone();
           v.push(encrypted_chunk);
           return v
        } 
        acc
    });

    let flattened: Vec<_> = vecs.iter().flat_map(|v| String::from_utf8(*v)).collect();
    //let str_res = String::from_utf8(flattened);
//    println!("{:?}", str_res);

}
