extern crate rustc_serialize as serialize;

use self::serialize::base64::{STANDARD, ToBase64};
use self::serialize::hex::FromHex;

pub fn convert_hex_to_base64() {
    // this 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    // should become SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match hex_string.from_hex() {
        Ok(result) => println!("Done: {}", result.to_base64(STANDARD)),
        Err(err) => println!("Oh Noes: {}", err)
    }
}
