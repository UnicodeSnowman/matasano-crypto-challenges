extern crate openssl;
extern crate rand;
use std::collections::HashMap;
use self::openssl::crypto::rand::{rand_bytes};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Decrypt,Encrypt};
use self::rand::Rng;

fn random_aes_key() -> Vec<u8> {
    rand_bytes(16)
}

fn encrypt_ecb(data: &[u8]) -> Vec<u8> {
    let key = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    encrypt(data, &key)
}

fn decrypt_ecb(data: &[u8]) -> Vec<u8> {
    let key = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    decrypt(data, &key)
}

fn encrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&data)
}

fn decrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let decrypter = Crypter::new(AES_128_ECB);
    decrypter.init(Decrypt, key, &vec![0]);
    decrypter.pad(false);
    decrypter.update(&data)
}

fn profile_for(email_str: &str) -> Vec<u8> {
    let mut encoded_string = String::new();
    let mut rng = rand::thread_rng();
    let uid: u32 = rng.gen_range(1, 1000);

    encoded_string.push_str("email=");
    encoded_string.push_str(&email_str); // TODO encode '&' and '=' chars
    encoded_string.push_str("&uid=");
    //encoded_string.push_str(&uid.to_string());
    encoded_string.push_str("10");
    encoded_string.push_str("&role=user");

    let encoded_bytes: Vec<u8> = encoded_string.bytes().collect();
    encrypt_ecb(&encoded_bytes)
}

fn k_equals_v_parse(string: &str) -> HashMap<&str, &str> {
    let res: Vec<&str> = string.split("&").collect();
    let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();

    for val in string.split("&") {
        let pair: Vec<&str> = val.split("=").collect();
        key_value_pairs.insert(pair[1], pair[1]);
        // TODO should be able to pattern match on this, but can't in this
        // version of rust (1.3.0). update this when upgrading version
//        match val.split("=").collect()[..] {
//            [key, value] => println!("{:?}", "key and value"),
//            _ => println!("{:?}", "Unknown")
//        }
    }

    key_value_pairs
}

pub fn run() {
    let mut result_vec: Vec<u8> = vec!();
    // we need to finagle ourselves a proper input which will give us our desired
    // result. 
    // email= is 6 bytes long, so we fill it in to make the first section 16 bytes
    // middle section is "admin" plus padding to get to 16, which we will eventually
    // copy/paste at the end
    // "role=" needs to be the last set of bytes, so we need an extra 3 bytes after
    // our padding admin string because "com" + &uid=10&role= is 16 bytes long
    let encrypted_profile = profile_for("capli@cap.admin\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00com");

    // take the first section, before our padded admin string
    for byte in &encrypted_profile[0..16] {
        result_vec.push(*byte);
    }

    // take the next section (com&uid=10& ... etc)
    for byte in &encrypted_profile[32..48] {
        result_vec.push(*byte);
    }

    // take our padded admin string and stick it at the end
    for byte in &encrypted_profile[16..32] {
        result_vec.push(*byte);
    }

    // result_vec is now our exploited copy-pasted encrypted profile.
    // to check our answer...
    let result = decrypt_ecb(&result_vec);
    println!("profile {:?}", String::from_utf8(result).unwrap());
}

