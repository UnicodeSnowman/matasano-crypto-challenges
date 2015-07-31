extern crate rustc_serialize as serialize;

use self::serialize::base64::{STANDARD, ToBase64};
//use self::serialize::hex::{ToHex, FromHex, FromHexError};
use self::serialize::hex::{ToHex, FromHex};
use std::iter::Zip;
use std::slice::Iter;
use std::collections::HashMap;

#[test]
fn test_convert_hex_to_base64() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let res = convert_hex_to_base64(hex_string);
    assert_eq!(res,
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

// -> Result<Vec<u8>, FromHexError> 
pub fn convert_hex_to_base64(hex_string: &str) -> String {
    // this 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    // should become SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//    match hex_string.from_hex() {
//        Ok(result) => println!("Hex to Base64: {}", result.to_base64(STANDARD)),
//        Err(err) => println!("Oh Noes: {}", err)
//    }

    // TODO can we do this without the explicit unwrap?
    hex_string.from_hex().unwrap().to_base64(STANDARD)
}

#[test]
fn test_fixed_xor() {
    let hex_string_a = "1c0111001f010100061a024b53535009181c";
    let hex_string_b = "686974207468652062756c6c277320657965";
    let res = fixed_xor(hex_string_a, hex_string_b);
    assert_eq!(res, "746865206b696420646f6e277420706c6179");
}

fn xor(vec_a: Vec<u8>, vec_b: Vec<u8>) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

pub fn fixed_xor(hex_string_a: &str, hex_string_b: &str) -> String {
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

    // TODO can we do this without the explicit unwrap?
    xor(hex_string_a.from_hex().unwrap(), 
        hex_string_b.from_hex().unwrap()).to_hex()
}

#[test]
fn test_single_bit_xor_cypher() {
    single_bit_xor_cypher();
    assert!(true);
}

fn gen_letter_map() -> HashMap<char, usize> {
    let mut letter_map: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = "zqxjkvbwpygumcfldhsirnoate".chars().collect();
    // or programmatically reverse the string so that the chars are weighted correctly...
    //let mut chars: Vec<char> = "etaonrishdlfcmugypwbvkjxqz".chars().collect();
    //chars.reverse();

    for (i, &c) in chars.iter().enumerate() {
        letter_map.insert(c, i);
    }

    letter_map
}

pub fn single_bit_xor_cypher() {
    let hex_string: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes: Vec<u8> = hex_string.from_hex().unwrap();
    let letter_map: HashMap<char, usize> = gen_letter_map();

    struct Winner {
        max: usize,
        winner: char,
        secret: String
    }

    let result: Winner = (0..255).fold(Winner { max: 0, winner: 'A', secret: "".to_string() }, |acc, i| {
        let byte = i as u8;
        let character = i as u8 as char;

        let xored_bytes: Vec<u8> = bytes.iter().map(|&a| a^byte).collect();
        let xored_string = String::from_utf8(xored_bytes);

        // can't just unwrap xored_string, some of the potential values are `Err` types,
        // so we match over it to only bother with the `Ok` values
        let score: usize = match xored_string {
            Ok(ref text) => {
                text.chars().fold(0, |acc, c| {
                    if letter_map.contains_key(&c) {
                        acc + letter_map.get(&c).unwrap()
                    } else {
                        acc
                    }
                })
            },
            Err(_) => 0
        };

        if score > acc.max {
            // we know if we get here, we have a valid string, so we can safely call unwrap
            Winner { max: score, winner: character, secret: xored_string.unwrap() }
        } else {
            acc
        }
    });

    println!("Score: {:?} Character: {:?} Secret: {:?}", result.max, result.winner, result.secret);
}

