extern crate openssl;
extern crate rand;
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter};
use self::openssl::crypto::symm::Mode::{Encrypt};
use self::openssl::crypto::rand::{rand_bytes};
use self::rand::Rng;
use ::shared::{detect_ecb, xor};

enum EncryptionType { CBC, ECB }

// Write a function to generate a random AES key;
// that's just 16 random bytes.'
pub fn generate_random_aes_key() -> Vec<u8> {
    rand_bytes(16)
}

fn append(data: &mut Vec<u8>, append: Vec<u8>) {
    for byte in append.into_iter() {
        data.push(byte);
    }
}

fn encrypt_cbc(data: &[u8]) -> Vec<u8> {
    let key = generate_random_aes_key();
    let iv: Vec<u8> = vec![0; 16];
 
    let (encrypted_result, _): (Vec<u8>, Vec<u8>) = data.chunks(16).fold((vec!(), iv), |acc, plaintext| {
        let (encrypted_result, iv) = acc;
        let xored = xor(&plaintext, &iv);
        let ciphertext = encrypt(&xored, &key);
        let mut encrypted_result_clone = encrypted_result.clone();
        for byte in ciphertext.clone() {
            encrypted_result_clone.push(byte);
        }
        (encrypted_result_clone, ciphertext)
    });

    encrypted_result
}

fn encrypt_ecb(data: &[u8]) -> Vec<u8> {
    let key = generate_random_aes_key();
    encrypt(data, &key)
}

fn encrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&data)
}

fn encryption_oracle(input: Vec<u8>) -> Option<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u8> = vec!();
    let n_prepend: u16 = rng.gen_range(5, 11);
    let n_append: u16 = rng.gen_range(5, 11);
    let prepend_bytes: Vec<u8> = rand_bytes(n_prepend as usize);
    let append_bytes: Vec<u8> = rand_bytes(n_append as usize);
    let input_copy = input.clone();

    append(&mut data, prepend_bytes);
    append(&mut data, input_copy);
    append(&mut data, append_bytes);

    match rng.gen_range(0, 2) {
        0 => Some(encrypt_cbc(&data)),
        1 => Some(encrypt_ecb(&data)),
        _ => None
    }
}

fn detect_mode(input: Vec<u8>) -> EncryptionType {
    if detect_ecb(&input) {
        EncryptionType::ECB
    } else {
        EncryptionType::CBC
    }
}

pub fn run() {
    let secret_message_bytes: Vec<u8> =
        "There’s a kind of music that reminds me of you It’s all clear expensive drinks and shiny shirts And the click of heels as they descend from the taxi like the first foot on the moon Oh and it glows with ache And if it hits me right on it’s almost too much to take And it’s got right angle razor thin lines That curve and swerve like perfect sines As we dress to the nines In an attempt to leave it all behind In a search of the moment between the seconds Where everything is just fine That silver thread embedded deep within our spines And I used to be kind of weird about this A fear of dependence on a guilty gilt-edged hedged transcendence that makes us liars And tense with fear of looking down and seeing that nothing really suspends us But it was never just another Saturday night Not with you in attendance So throw your hands in the air And wave them like you just don’t care It’s on a whim; it’s on a dare To shrug away what we can’t bear And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So it’s a deep blue see-through membrane That protects us, it connects us A pulsing cellophane Party-train skein that helps us and envelopes And keeps us locked inside Forever and ever along for the ride And we’re moving through a phosphorescent gel a semi-solid self-lit ocean And it’s a funny notion, isn’t it?  Yeah, but I’m kinda digging it And it’s rigged and isn’t nearly so big And it speaks only of its own perpetual near miss Like the uncertain memory of a stranger’s mistaken kiss As faces slide by in glowing shadows Like snowbound ghosts that go up and down In epileptic shivers and negative radioactive slivers In a landscape of endless dull glitter And a taste in my mouth so sweet, yet so bitter And we exhaust ourselves trying to get there Somebody scream all right We’ll try to fill the echoless night So fasten up and hold tight We can’t give up without a fight And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So in the end, whatever, we die, we dissolve Equations unbalanced, riddles unsolved And we were never connected or involved Except for the intersections and crazy mathematics With no time and no space and no schedule and no place And they pass right through us without a trace And sometimes that music drifts through my car On a spring night when anything is possible And I close my eyes and I nod my head And I wonder how you been And I count to a hundred and ten Because you’ll always be my hero even if I never see you again".bytes().collect();
    let result = encryption_oracle(secret_message_bytes).unwrap();

    match detect_mode(result) {
        EncryptionType::ECB => println!("{:?}", "ECB!"),
        EncryptionType::CBC => println!("{:?}", "CBC!")
    }
}
