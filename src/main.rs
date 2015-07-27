extern crate matasano;

use matasano::one;

fn main() {
    println!("{}", "Section One");
    println!("{}", "================");
    one::convert_hex_to_base64();
    println!("{}", "================");
    one::fixed_xor();
}
