extern crate openssl;
extern crate rand;
use std::collections::HashMap;
use self::openssl::crypto::rand::{rand_bytes};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Decrypt,Encrypt};
use self::rand::Rng;

pub fn random_aes_key() -> Vec<u8> {
    rand_bytes(16)
}

pub fn encrypt_ecb(data: &[u8]) -> Vec<u8> {
    let key = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    encrypt(data, &key)
}

pub fn decrypt_ecb(data: &[u8]) -> Vec<u8> {
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

pub fn profile_for(email_str: &str) -> String {
    let mut encoded_string = String::new();
    let mut rng = rand::thread_rng();
    let uid: u32 = rng.gen_range(1, 1000);

    encoded_string.push_str("email=");
    encoded_string.push_str(&email_str); // TODO encode '&' and '=' chars
    encoded_string.push_str("&uid=");
    //encoded_string.push_str(&uid.to_string());
    encoded_string.push_str("10");
    encoded_string.push_str("&role=user");
    encoded_string
}

pub fn k_equals_v_parse(string: &str) -> HashMap<&str, &str> {
    let res: Vec<&str> = string.split("&").collect();
    let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();

    for val in string.split("&") {
        let pair: Vec<&str> = val.split("=").collect();
        key_value_pairs.insert(pair[0], pair[1]);
        // TODO should be able to pattern match on this, but can't in this
        // version of rust (1.3.0). update this when upgrading version
//        match val.split("=").collect()[..] {
//            [key, value] => println!("{:?}", "key and value"),
//            _ => println!("{:?}", "Unknown")
//        }
    }

    key_value_pairs
}

