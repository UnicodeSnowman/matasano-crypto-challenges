use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn open_file(path: &str) -> io::Result<String> {
    let mut file_string = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut file_string));
    Ok(file_string)
}
