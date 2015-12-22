extern crate openssl;
extern crate rustc_serialize as serialize;
extern crate rand;
use self::serialize::base64::{FromBase64};
use std::collections::HashMap;
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt};
use ::shared::{detect_ecb};

static UNKNOWN_STRING: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";

fn encrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&data)
}

fn append(data: &mut Vec<u8>, append: Vec<u8>) {
    for byte in append.into_iter() {
        data.push(byte);
    }
}

fn encryption_oracle(input: &Vec<u8>) -> Vec<u8> {
    let consistent_key = vec!(110, 203, 52, 7, 87, 32, 203, 144, 10, 157, 241, 177, 0, 95, 189, 94);
    let mut data: Vec<u8> = vec!();
    let input_copy = input.clone();

    let unknown_base64_string = UNKNOWN_STRING.from_base64().unwrap();

    append(&mut data, input_copy);
    append(&mut data, unknown_base64_string);
    encrypt(&data, &consistent_key)
}

fn gen_n_bytes(n: usize) -> Vec<u8> {
    let gen_string: String = (0..n).map(|_| "A").collect();
    gen_string.bytes().collect()
}

fn detect_blocksize() -> usize {
    // send strings to the cipher, increase by a single character each time
    // if the length of the returned cipher changes, the difference between
    // the new length and the old length is the block size
    let base_length = encryption_oracle(&"A".bytes().collect()).len();
    for val in 2..64 {
        let character_string_bytes: Vec<u8> = gen_n_bytes(val);
        let encrypted = encryption_oracle(&character_string_bytes);
        let length = encrypted.len();
        if length > base_length {
            return length - base_length;
        }
    }
    base_length
}

pub fn run() {
    let msg: Vec<u8> =
        "There’s a kind of music that reminds me of you It’s all clear expensive drinks and shiny shirts And the click of heels as they descend from the taxi like the first foot on the moon Oh and it glows with ache And if it hits me right on it’s almost too much to take And it’s got right angle razor thin lines That curve and swerve like perfect sines As we dress to the nines In an attempt to leave it all behind In a search of the moment between the seconds Where everything is just fine That silver thread embedded deep within our spines And I used to be kind of weird about this A fear of dependence on a guilty gilt-edged hedged transcendence that makes us liars And tense with fear of looking down and seeing that nothing really suspends us But it was never just another Saturday night Not with you in attendance So throw your hands in the air And wave them like you just don’t care It’s on a whim; it’s on a dare To shrug away what we can’t bear And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So it’s a deep blue see-through membrane That protects us, it connects us A pulsing cellophane Party-train skein that helps us and envelopes And keeps us locked inside Forever and ever along for the ride And we’re moving through a phosphorescent gel a semi-solid self-lit ocean And it’s a funny notion, isn’t it?  Yeah, but I’m kinda digging it And it’s rigged and isn’t nearly so big And it speaks only of its own perpetual near miss Like the uncertain memory of a stranger’s mistaken kiss As faces slide by in glowing shadows Like snowbound ghosts that go up and down In epileptic shivers and negative radioactive slivers In a landscape of endless dull glitter And a taste in my mouth so sweet, yet so bitter And we exhaust ourselves trying to get there Somebody scream all right We’ll try to fill the echoless night So fasten up and hold tight We can’t give up without a fight And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So in the end, whatever, we die, we dissolve Equations unbalanced, riddles unsolved And we were never connected or involved Except for the intersections and crazy mathematics With no time and no space and no schedule and no place And they pass right through us without a trace And sometimes that music drifts through my car On a spring night when anything is possible And I close my eyes and I nod my head And I wonder how you been And I count to a hundred and ten Because you’ll always be my hero even if I never see you again".bytes().collect();
    let encrypted = encryption_oracle(&msg);
    let is_ecb = detect_ecb(&encrypted);
    let blocksize = detect_blocksize();
    let mut results: Vec<u8> = vec!();

    if is_ecb {
        let mut results_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

        for s in (1..blocksize - 1) {
            let mut ciphertext = encryption_oracle(&gen_n_bytes(blocksize - s));
            ciphertext.truncate(16);

            for i in 0..255 {
                let byte = i as u8;
                let mut vec = gen_n_bytes(blocksize - s);
                // stick our results from previous iterations
                // on our vector
                for b in results.clone() {
                    vec.push(b);
                }
                vec.push(byte);
                let mut ctext = encryption_oracle(&vec);
                ctext.truncate(16);
                results_map.insert(ctext, vec);
            }

            let answer = results_map.get(&ciphertext);
            match answer {
                Some(vec) => { 
                    if let Some(value) = vec.last() {
                        results.push(*value);
                    }
                },
                None => println!("{:?}", "Not Found")
            }
        }

        let result_string = String::from_utf8(results);
        println!("{:?}", result_string);

    }
}

