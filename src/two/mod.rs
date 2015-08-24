pub fn pad_pkcs_7(block: &mut Vec<u8>, block_size: u8) {
    // "YELLOW SUBMARINE" padded to 20 bytes is...
    // "YELLOW SUBMARINE\x04\x04\x04\x04"

    let length = block.len() as u8;
    let padding_length = block_size - (length % block_size);

    for i in (0..padding_length) {
        block.push(padding_length as u8);
    }
}

pub fn cbc_mode() {}
