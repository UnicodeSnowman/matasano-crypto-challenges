use std::collections::HashMap;

pub fn profile_for(email_str: &str) {
    let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();
    let mut encoded_string = String::new();
    key_value_pairs.insert("email", email_str);
    key_value_pairs.insert("uid", "10");
    key_value_pairs.insert("role", "user");

    for (key, value) in key_value_pairs {
        let key_value_pair: String = vec!(key, value).join("=");

        // FIXME these two if statements blow. there's gotta
        // be a way to do this with join, but the type/borrow check
        // is giving me hell. more later...
        if encoded_string.len() > 0 {
            encoded_string.push_str(&"&");
        }

        if key_value_pair.len() > 0 {
            encoded_string.push_str(&key_value_pair);
        }
    }
    println!("{:?}", encoded_string);
}

pub fn k_equals_v_parse(string: &str) -> HashMap<&str, &str> {
    let res: Vec<&str> = string.split("&").collect();
    let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();

    for val in string.split("&") {
        let pair: Vec<&str> = val.split("=").collect();
        key_value_pairs.insert(pair[0], pair[1]);
        // TODO should be able to pattern match on this, but can't in this
        // version of rust (1.3.0). update this when upgrading version
//        match val.split("=").collect()[..] {
//            [key, value] => println!("{:?}", "key and value"),
//            _ => println!("{:?}", "Unknown")
//        }
    }

    println!("{:?}", key_value_pairs);
    key_value_pairs
}
