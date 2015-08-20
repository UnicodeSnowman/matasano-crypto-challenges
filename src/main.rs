extern crate matasano;

use matasano::one;

fn main() {
    println!("{}", "Section One");
    println!("{}", "================");
    //one::convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    //one::fixed_xor();

    //let hex_string: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //let bytes: Vec<u8> = hex_string.from_hex().unwrap();
    //one::single_bit_xor_cypher(&bytes);
    //one::single_bit_xor_cypher(&hex_string);

    let winner = one::detect_single_character_xor();
    match winner {
        Ok(w) => println!("WINNAR {:?} {:?}", w.key, w.secret),
        Err(err) => println!("oh noes, you lose {:?}", err)
    }

    //let key: Vec<u8> = vec!(b'I', b'C', b'E');
    //let string_bytes: Vec<u8> = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal".bytes().collect();
    //let xored_string = one::repeating_key_xor(&string_bytes, &key);

//    let xored_bytes = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20690a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();
//    let xored_hex = one::repeating_key_xor(&xored_bytes, &key);
//    let bytes = xored_hex.from_hex().unwrap();
//    let xored_string = String::from_utf8(bytes).unwrap();

    let res = one::decrypto();
    println!("{:?}", res);
}
