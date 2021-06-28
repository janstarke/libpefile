pub fn utf16_from_slice(slice: &[u8], mut offset: usize, characters: usize) -> String {
    let mut name_chars = Vec::new();
    for _ in 0..characters {
        name_chars.push(slice[offset] as u16 | ((slice[offset+1] as u16)<<8 as u8));
        offset += 2;
    }
    String::from_utf16_lossy(&name_chars[..])
}