extern crate matasano;

use matasano::one;
use matasano::two;

fn main() {
    println!("{}", "Section One");
    println!("{}", "================");
    //one::convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    //one::fixed_xor();

    //let hex_string: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //let bytes: Vec<u8> = hex_string.from_hex().unwrap();
    //one::single_bit_xor_cypher(&bytes);
    //one::single_bit_xor_cypher(&hex_string);

//    let winner = one::detect_single_character_xor();
//    match winner {
//        Ok(w) => println!("WINNAR {:?} {:?}", w.key, w.secret),
//        Err(err) => println!("oh noes, you lose {:?}", err)
//    }

    //let key: Vec<u8> = vec!(b'I', b'C', b'E');
    //let string_bytes: Vec<u8> = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal".bytes().collect();
    //let xored_string = one::repeating_key_xor(&string_bytes, &key);

    //let res = one::break_repeating_key_xor();

    println!("{}", "Section Two");
    println!("{}", "================");
//    let mut bytes = "YELLOW SUBMARINE".bytes().collect();
//    two::pad_pkcs_7(&mut bytes, 20);
//    println!("{:?}", String::from_utf8(bytes).unwrap());

    //two::an_ecb_cbc_detection_oracle::run();
    //two::byte_at_a_time_ecb_decryption_simple::run();
    //two::ecb_cut_and_paste::run();
    two::byte_at_a_time_ecb_decryption_harder::run();
}
