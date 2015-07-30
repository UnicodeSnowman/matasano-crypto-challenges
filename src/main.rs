extern crate matasano;

use matasano::one;

fn main() {
    println!("{}", "Section One");

    println!("{}", "================");
    //one::convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    println!("{}", "================");
    //one::fixed_xor();

    println!("{}", "================");
    one::single_bit_xor_cypher();
}
