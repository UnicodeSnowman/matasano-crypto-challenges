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
//    let mut bytes = "YELLOW SUBMARINE".bytes().collect();
//    two::pad_pkcs_7(&mut bytes, 20);
//    println!("{:?}", String::from_utf8(bytes).unwrap());

//    let secret_message_bytes: Vec<u8> =
//        "There’s a kind of music that reminds me of you It’s all clear expensive drinks and shiny shirts And the click of heels as they descend from the taxi like the first foot on the moon Oh and it glows with ache And if it hits me right on it’s almost too much to take And it’s got right angle razor thin lines That curve and swerve like perfect sines As we dress to the nines In an attempt to leave it all behind In a search of the moment between the seconds Where everything is just fine That silver thread embedded deep within our spines And I used to be kind of weird about this A fear of dependence on a guilty gilt-edged hedged transcendence that makes us liars And tense with fear of looking down and seeing that nothing really suspends us But it was never just another Saturday night Not with you in attendance So throw your hands in the air And wave them like you just don’t care It’s on a whim; it’s on a dare To shrug away what we can’t bear And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So it’s a deep blue see-through membrane That protects us, it connects us A pulsing cellophane Party-train skein that helps us and envelopes And keeps us locked inside Forever and ever along for the ride And we’re moving through a phosphorescent gel a semi-solid self-lit ocean And it’s a funny notion, isn’t it?  Yeah, but I’m kinda digging it And it’s rigged and isn’t nearly so big And it speaks only of its own perpetual near miss Like the uncertain memory of a stranger’s mistaken kiss As faces slide by in glowing shadows Like snowbound ghosts that go up and down In epileptic shivers and negative radioactive slivers In a landscape of endless dull glitter And a taste in my mouth so sweet, yet so bitter And we exhaust ourselves trying to get there Somebody scream all right We’ll try to fill the echoless night So fasten up and hold tight We can’t give up without a fight And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So in the end, whatever, we die, we dissolve Equations unbalanced, riddles unsolved And we were never connected or involved Except for the intersections and crazy mathematics With no time and no space and no schedule and no place And they pass right through us without a trace And sometimes that music drifts through my car On a spring night when anything is possible And I close my eyes and I nod my head And I wonder how you been And I count to a hundred and ten Because you’ll always be my hero even if I never see you again".bytes().collect();
//    let result = two::an_ecb_cbc_detection_oracle::encryption_oracle(secret_message_bytes).unwrap();
//
//    match two::an_ecb_cbc_detection_oracle::detect_mode(result) {
//        two::an_ecb_cbc_detection_oracle::EncryptionType::ECB => println!("{:?}", "ECB!"),
//        two::an_ecb_cbc_detection_oracle::EncryptionType::CBC => println!("{:?}", "CBC!")
//    }

    let secret_message_bytes: Vec<u8> =
        "There’s a kind of music that reminds me of you It’s all clear expensive drinks and shiny shirts And the click of heels as they descend from the taxi like the first foot on the moon Oh and it glows with ache And if it hits me right on it’s almost too much to take And it’s got right angle razor thin lines That curve and swerve like perfect sines As we dress to the nines In an attempt to leave it all behind In a search of the moment between the seconds Where everything is just fine That silver thread embedded deep within our spines And I used to be kind of weird about this A fear of dependence on a guilty gilt-edged hedged transcendence that makes us liars And tense with fear of looking down and seeing that nothing really suspends us But it was never just another Saturday night Not with you in attendance So throw your hands in the air And wave them like you just don’t care It’s on a whim; it’s on a dare To shrug away what we can’t bear And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So it’s a deep blue see-through membrane That protects us, it connects us A pulsing cellophane Party-train skein that helps us and envelopes And keeps us locked inside Forever and ever along for the ride And we’re moving through a phosphorescent gel a semi-solid self-lit ocean And it’s a funny notion, isn’t it?  Yeah, but I’m kinda digging it And it’s rigged and isn’t nearly so big And it speaks only of its own perpetual near miss Like the uncertain memory of a stranger’s mistaken kiss As faces slide by in glowing shadows Like snowbound ghosts that go up and down In epileptic shivers and negative radioactive slivers In a landscape of endless dull glitter And a taste in my mouth so sweet, yet so bitter And we exhaust ourselves trying to get there Somebody scream all right We’ll try to fill the echoless night So fasten up and hold tight We can’t give up without a fight And we’re going back and forth And back and forth and back and forth and back We’re going back and forth And back and forth and back and forth and back So in the end, whatever, we die, we dissolve Equations unbalanced, riddles unsolved And we were never connected or involved Except for the intersections and crazy mathematics With no time and no space and no schedule and no place And they pass right through us without a trace And sometimes that music drifts through my car On a spring night when anything is possible And I close my eyes and I nod my head And I wonder how you been And I count to a hundred and ten Because you’ll always be my hero even if I never see you again".bytes().collect();
    let result = two::byte_at_a_time_ecb_decryption_simple::encryption_oracle(secret_message_bytes);
}
