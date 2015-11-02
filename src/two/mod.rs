extern crate rustc_serialize as serialize;
extern crate openssl;

use ::shared::{open_file};
use self::serialize::hex::{ToHex, FromHex};
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{Crypter, encrypt, decrypt};
use self::openssl::crypto::symm::Mode::{Encrypt, Decrypt};
use std::slice::Iter;
use std::iter::Zip;

pub fn pad_pkcs_7(block: &mut Vec<u8>, block_size: u8) {
    // "YELLOW SUBMARINE" padded to 20 bytes is...
    // "YELLOW SUBMARINE\x04\x04\x04\x04"

    let length = block.len() as u8;
    let padding_length = block_size - (length % block_size);

    for _ in (0..padding_length) {
        block.push(padding_length as u8);
    }
}

pub fn cbc_mode() {
    encrypt_cbc();
}

fn xor(vec_a: &[u8], vec_b: &Vec<u8>) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

fn encrypt_plaintext(key: &Vec<u8>, plaintext: &[u8], v: &Vec<u8>) -> Vec<u8> {
    let xored = xor(plaintext, v);
    let encrypter = Crypter::new(AES_128_ECB);
    encrypter.init(Encrypt, &key, &vec![0]);
    encrypter.pad(false);
    encrypter.update(&xored[..])
}

fn encrypt_cbc() {
    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
    let iv: Vec<u8> = vec![0; 16];
    let iv2: Vec<u8> = vec![0; 16];
    //let mut secret_message_bytes: Vec<u8> = "Bacon ipsum dolor amet short loin turkey tail, jowl drumstick spare ribs strip steak pastrami shank frankfurter brisket shoulder pork loin sausage. Frankfurter sausage corned beef swine. Corned beef chuck picanha kevin andouille, filet mignon sausage pig hamburger drumstick ribeye. Spare ribs kevin pork, pancetta short loin meatloaf bresaola frankfurter pork loin boudin. Sausage short loin ham hock, landjaeger tail rump jowl beef cupim spare ribs shoulder kielbasa. Beef ribs drumstick beef landjaeger turkey venison.  Andouille strip steak ham tail landjaeger flank short ribs chicken swine. Landjaeger pork belly ground round ham strip steak. Turducken ball tip beef corned beef fatback shank. Prosciutto hamburger spare ribs ribeye ham, fatback tenderloin chuck chicken shoulder andouille short ribs drumstick brisket.  Pastrami ground round beef ribs, porchetta picanha drumstick tri-tip pork tenderloin meatball bacon doner. Cupim hamburger jerky, chicken pig prosciutto pork loin tail ham. Beef jowl shoulder, beef ribs capicola turkey picanha fatback pork chop swine ground round. Short ribs bresaola biltong, porchetta leberkas bacon rump t-bone. Filet mignon boudin shankle venison, pork belly tenderloin shank meatball kielbasa pork loin chicken cow.  Cow boudin frankfurter corned beef chicken t-bone pork fatback flank porchetta rump chuck prosciutto short ribs pastrami. Picanha pork chop chicken tenderloin prosciutto drumstick boudin cow pancetta landjaeger ground round flank ham pork. Biltong rump jowl alcatra cow corned beef bresaola tongue jerky ground round tenderloin short loin salami. Boudin beef meatloaf hamburger pancetta strip steak. Kevin turkey meatball chuck, turducken pancetta brisket.  Fatback filet mignon meatloaf tri-tip turducken meatball tenderloin, bresaola shankle doner strip steak chuck. Ball tip kevin shankle fatback leberkas hamburger beef ribs. Brisket flank swine kevin, andouille hamburger frankfurter. Pork loin biltong landjaeger capicola. Sausage landjaeger kielbasa salami tri-tip flank beef.".bytes().collect();
    let mut secret_message_bytes: Vec<u8> = "Bacon ipsum dolr amet short loin".bytes().collect();
    //pad_pkcs_7(&mut secret_message_bytes, 16);

    let encrypted: Vec<Vec<u8>> = secret_message_bytes.chunks(16).fold(vec!(), |acc, plaintext| {
        if let Some(v) = acc.last() {
            let cipher_text = encrypt_plaintext(&key, plaintext, &v);
            let mut acc_copy = acc.clone();
            acc_copy.push(cipher_text);
            return acc_copy;
        } else {
            let cipher_text = encrypt_plaintext(&key, plaintext, &iv);
            vec!(cipher_text)
        }
    });

    let file_string: String = open_file("assets/10.txt").unwrap();
    let file_bytes: Vec<u8> = file_string.bytes().collect();
    //let file_bytes: Vec<u8> = file_string.replace("\n", "").from_base64().unwrap();

    //let decrypted: Vec<Vec<u8>> = encrypted.into_iter().fold(vec!(), |acc, ciphertext| {
    let decrypted: Vec<Vec<u8>> = file_bytes.chunks(16).fold(vec!(), |acc, ciphertext| {
        if let Some(v) = acc.last() {
            let decrypter = Crypter::new(AES_128_ECB);
            decrypter.init(Decrypt, &key, &vec![0]);
            decrypter.pad(false);
            let decrypted = decrypter.update(&ciphertext);
            let plaintext = xor(&decrypted, &v);
            let mut v_copy = acc.clone();
            v_copy.push(plaintext);
            v_copy
        } else {
            let decrypter = Crypter::new(AES_128_ECB);
            decrypter.init(Decrypt, &key, &vec![0]);
            decrypter.pad(false);
            let decrypted = decrypter.update(&ciphertext);
            let plaintext = xor(&decrypted, &iv2);
            vec!(plaintext)
        }
    });

    println!("{:?}", decrypted);
    println!("{:?}", "=====");
    let flattened: Vec<u8> = decrypted.into_iter().flat_map(|a| a).collect();
    println!("{:?}", String::from_utf8(flattened));
    println!("{:?}", "=====");
}

