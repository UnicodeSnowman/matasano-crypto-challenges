extern crate rustc_serialize as serialize;

use self::serialize::base64::{STANDARD, ToBase64};
use self::serialize::hex::FromHex;

pub fn convert_hex_to_base64() {
    // this 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    // should become SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match hex_string.from_hex() {
        Ok(result) => println!("Hex to Base64: {}", result.to_base64(STANDARD)),
        Err(err) => println!("Oh Noes: {}", err)
    }
}

fn xor(vec_a: Vec<u8>, vec_b: Vec<u8>) -> Vec<u8> {
    println!("{}", "I'm XOR'ing your bizness");
    println!("{:?}", vec_a);
    println!("{:?}", vec_b);
    vec_a
}

pub fn fixed_xor() {
// Write function that takes two equal-length buffers and produces 
// their XOR combination.
// 
// If your function works properly, then when you feed it the string:
// 1c0111001f010100061a024b53535009181c
// ... after hex decoding, and when XOR'd against:
// 
// 686974207468652062756c6c277320657965
// ... should produce:
// 
// 746865206b696420646f6e277420706c6179

    let hex_vec_a = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let hex_vec_b = "686974207468652062756c6c277320657965".from_hex().unwrap();
    xor(hex_vec_a, hex_vec_b);
}
