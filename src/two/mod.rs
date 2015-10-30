extern crate rustc_serialize as serialize;
extern crate openssl;

use ::shared::{open_file};
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};
use self::openssl::crypto::symm::Type::{AES_128_ECB};
use self::openssl::crypto::symm::{encrypt, decrypt};
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

fn xor(vec_a: &Vec<u8>, vec_b: &Vec<u8>) -> Vec<u8> {
    let zipped: Zip<Iter<u8>, Iter<u8>> = vec_a.iter().zip(vec_b.iter());
    zipped.map(|(&a, &b)| a^b).collect()
}

fn encrypt_cbc() {
    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
    let iv: Vec<u8> = vec![0; 10];
    let iv2: Vec<u8> = vec![0; 10];
    let secret_message = "Bacon ipsum dolor amet short loin turkey tail, jowl drumstick spare ribs strip steak pastrami shank frankfurter brisket shoulder pork loin sausage. Frankfurter sausage corned beef swine. Corned beef chuck picanha kevin andouille, filet mignon sausage pig hamburger drumstick ribeye. Spare ribs kevin pork, pancetta short loin meatloaf bresaola frankfurter pork loin boudin. Sausage short loin ham hock, landjaeger tail rump jowl beef cupim spare ribs shoulder kielbasa. Beef ribs drumstick beef landjaeger turkey venison.  Andouille strip steak ham tail landjaeger flank short ribs chicken swine. Landjaeger pork belly ground round ham strip steak. Turducken ball tip beef corned beef fatback shank. Prosciutto hamburger spare ribs ribeye ham, fatback tenderloin chuck chicken shoulder andouille short ribs drumstick brisket.  Pastrami ground round beef ribs, porchetta picanha drumstick tri-tip pork tenderloin meatball bacon doner. Cupim hamburger jerky, chicken pig prosciutto pork loin tail ham. Beef jowl shoulder, beef ribs capicola turkey picanha fatback pork chop swine ground round. Short ribs bresaola biltong, porchetta leberkas bacon rump t-bone. Filet mignon boudin shankle venison, pork belly tenderloin shank meatball kielbasa pork loin chicken cow.  Cow boudin frankfurter corned beef chicken t-bone pork fatback flank porchetta rump chuck prosciutto short ribs pastrami. Picanha pork chop chicken tenderloin prosciutto drumstick boudin cow pancetta landjaeger ground round flank ham pork. Biltong rump jowl alcatra cow corned beef bresaola tongue jerky ground round tenderloin short loin salami. Boudin beef meatloaf hamburger pancetta strip steak. Kevin turkey meatball chuck, turducken pancetta brisket.  Fatback filet mignon meatloaf tri-tip turducken meatball tenderloin, bresaola shankle doner strip steak chuck. Ball tip kevin shankle fatback leberkas hamburger beef ribs. Brisket flank swine kevin, andouille hamburger frankfurter. Pork loin biltong landjaeger capicola. Sausage landjaeger kielbasa salami tri-tip flank beef.";
    let secret_message_bytes: Vec<u8> = secret_message.bytes().collect();

    let vecs: Vec<Vec<u8>> = secret_message_bytes.chunks(16).fold(vec!(iv), |acc, chunk| {
        if let Some(v) = acc.last() {
            let encrypted_chunk = encrypt(AES_128_ECB, &key, vec![0], chunk);
            let mut v_copy = acc.clone();
            let result = xor(&encrypted_chunk, &v);
            v_copy.push(result);
            return v_copy;
        }
        acc
    });


    let decrypted: Vec<Vec<u8>> = vecs.into_iter().fold(vec!(), |acc, chunk| {
        if let Some(v) = acc.last() {
            let result = xor(&chunk, &v);
            let decrypted_chunk = decrypt(AES_128_ECB, &key, vec![0], &result);
            let mut v_copy = acc.clone();
            v_copy.push(result);
            return v_copy;
        } else {
            let mut v_copy = acc.clone();
            v_copy.push(chunk);
            return v_copy;
        }
    });

    let flattened: Vec<u8> = decrypted.into_iter().flat_map(|a| a).collect();
    //println!("{:?}", vecs);
    println!("{:?}", String::from_utf8(flattened));
}

//fn encrypt_cbc() {
//    let file_string: String = open_file("assets/10.txt").unwrap();
//    let file_bytes: Vec<u8> = file_string.from_base64().unwrap();
//
//    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
//    let iv: Vec<u8> = vec![0; 10];
//
//    let vecs: Vec<Vec<u8>> = file_bytes.chunks(16).fold(vec!(iv), |acc, chunk| {
//        if let Some(v) = acc.last() {
//           let encrypted_chunk = encrypt(AES_128_ECB, &key, v, chunk);
//           let mut v = acc.clone();
//           v.push(encrypted_chunk);
//           return v
//        } 
//        acc
//    });
//
//    let flattened: Vec<_> = vecs.iter().flat_map(|v| String::from_utf8(*v)).collect();
//  //let str_res = String::from_utf8(flattened);
//    println!("{:?}", str_res);
//
//}
