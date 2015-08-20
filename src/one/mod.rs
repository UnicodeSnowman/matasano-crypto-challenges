extern crate rustc_serialize as serialize;
mod break_repeating_key_xor;

use self::serialize::base64::{STANDARD, FromBase64, ToBase64};
use self::serialize::hex::{ToHex, FromHex};
use std::iter::Zip;
use std::slice::Iter;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use self::break_repeating_key_xor::{compute_hamming_distance};

#[test]
fn test_convert_hex_to_base64() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let res = convert_hex_to_base64(hex_string);
    assert_eq!(res,
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

pub fn convert_hex_to_base64(hex_string: &str) -> String {
    hex_string.from_hex().unwrap().to_base64(STANDARD)
}

#[test]
fn test_fixed_xor() {
    let hex_string_a = "1c0111001f010100061a024b53535009181c";
    let hex_string_b = "686974207468652062756c6c277320657965";
    let res = fixed_xor(&hex_string_a, &hex_string_b);
    assert_eq!(res, "746865206b696420646f6e277420706c6179");
}

fn xor(vec_a: &Vec<u8>, vec_b: &Vec<u8>) -> Vec<u8> {
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
    xor(&hex_string_a.from_hex().unwrap(), 
        &hex_string_b.from_hex().unwrap()).to_hex()
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

pub struct Winner {
    max: usize,
    pub key: char, // this could be Option so we don't have to default it to A
    pub secret: String
}

pub fn single_bit_xor_cypher(bytes: &Vec<u8>) -> Winner {
    let letter_map: HashMap<char, usize> = gen_letter_map();

    let result: Winner = (0..255).fold(Winner { max: 0, key: 'A', secret: "".to_string() }, |acc, i| {
        let byte = i as u8;
        let character = i as u8 as char;

        let xored_bytes: Vec<u8> = bytes.iter().map(|&a| a^byte).collect();
        let xored_string = String::from_utf8(xored_bytes);

        // can't just unwrap xored_string, some of the potential values are `Err` types,
        // so we match over it to only bother with the `Ok` values
        let score: usize = match xored_string {
            Ok(ref text) => {
                text.chars().fold(0, |acc, c| {
                    if let Some(letter) = letter_map.get(&c) {
                        acc + letter
                    } else {
                        acc
                    }
                })
            },
            Err(_) => 0
        };

        if score > acc.max {
            // we know if we get here, we have a valid string, so we can safely call unwrap
            Winner { max: score, key: character, secret: xored_string.unwrap() }
        } else {
            acc
        }
    });

    result

}

pub fn detect_single_character_xor() -> io::Result<Winner> {
    let file_string = open_file("assets/4.txt").unwrap();
    let lines: Vec<&str> = file_string.split("\n").collect();

    let winner: Winner = lines.iter().fold(Winner { max: 0, key: 'A', secret: "".to_string() }, |acc, line| {
        let bytes: Vec<u8> = line.from_hex().unwrap();
        let new_line = single_bit_xor_cypher(&bytes);
        if new_line.max > acc.max {
            new_line
        } else {
            acc
        }
    });

    Ok(winner)
}

pub fn repeating_key_xor(bytes: &Vec<u8>, key: &Vec<u8>) -> String {
    let mut xored_bytes: Vec<u8> = Vec::new();
    let key_length = key.len();

    // map with index? I should try writing a utility map_with_index
    // using reduce... TODO
    for (i, &b) in bytes.iter().enumerate() {
        let character_byte = key[i % key_length];
        xored_bytes.push(character_byte^b);
    }

    xored_bytes.to_hex()
}

pub fn decrypto() -> String {
    let file_string: String = open_file("assets/6.txt").unwrap();
    let file_bytes: Vec<u8> = file_string.from_base64().unwrap();

    // FIXME this kinda sucks, could probably `fold` over 2..41 range
    // to get rid of these mutable vars
//    let res = (2..41).fold((f32::NAN, 2), |acc, v| {
//
//    });
    let mut score_min: f32 = 100 as f32;
    let mut keysize = 0;

    for ks in 2..41 {
        let mut sum: u32 = 0;
        let mut count: u32 = 0;
        let mut chunks = file_bytes.chunks(ks).peekable();

        while !chunks.peek().is_none() {
            let (one, two) = (chunks.next(), chunks.next());
            if one.is_some() && two.is_some() {
                let res = compute_hamming_distance(&one.unwrap().to_vec(), &two.unwrap().to_vec());
                count += 1;
                sum += res;
            }
        }

        let avg: f32 = (sum as f32) / (count as f32);
        let score = avg / (ks as f32);

        if score < score_min {
            score_min = score;
            keysize = ks;
        }
    }

    let mut transposed_blocks: HashMap<usize, Vec<u8>> = HashMap::new();

    // transpose, create another block out of first byte of each block,
    // another block out of second byte of each block, etc.
    for block in file_bytes.chunks(keysize) {
        for (i, byte) in block.iter().enumerate() {
            if transposed_blocks.contains_key(&i) {
                if let Some(vec) = transposed_blocks.get_mut(&i) {
                    vec.push(*byte);
                }
            } else {
                transposed_blocks.insert(i, vec!(*byte));
            }
        }
    }

    let key: Vec<u8> = (0..keysize).map(|i| {
        let char_key = match transposed_blocks.get(&i) {
            Some(block) => single_bit_xor_cypher(&block).key,
            None => panic!("Unable to decrypt single bit key") // better way to handle this?
        };

        char_key as u8
    }).collect();

    let bytes: Vec<u8> = repeating_key_xor(&file_bytes, &key).from_hex().unwrap();
    String::from_utf8(bytes).unwrap()
}

fn open_file(path: &str) -> io::Result<String> {
    let mut file_string = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut file_string));
    Ok(file_string)
}

