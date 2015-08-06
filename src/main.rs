extern crate matasano;

use matasano::one;

fn main() {
    println!("{}", "Section One");

    println!("{}", "================");
    //one::convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    println!("{}", "================");
    //one::fixed_xor();

    println!("{}", "================");
    //let hex_string: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //one::single_bit_xor_cypher(&hex_string);

    println!("{}", "================");
    let winner = one::detect_single_character_xor();
    match winner {
        Ok(w) => println!("WINNAR {:?}", w.secret),
        Err(err) => println!("oh noes, you lose {:?}", err)
    }

    println!("{}", "================");
    let xored_string = one::repeating_key_xor("Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal");
    println!("{:?}", xored_string);
}
