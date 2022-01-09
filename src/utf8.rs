pub fn encode_codepoint(codepoint: u32) -> Vec<u8> {
    let mut seq = Vec::new();

    match codepoint {
        _ if codepoint <= 0x7F => {
            seq.push(codepoint as u8);
        },
        _ if codepoint <= 0x7FF => {
            seq.push(((codepoint >> 6) as u8 & 0x1F) | 0xC0);
            seq.push(((codepoint) as u8 & 0x3F) | 0x80);
        },
        _ if codepoint <= 0xFFFF => {
            seq.push(((codepoint >> 12) as u8 & 0x0F) | 0xE0);
            seq.push(((codepoint >> 6) as u8 & 0x3F) | 0x80);
            seq.push(((codepoint) as u8 & 0x3F) | 0x80);
        },
        _ if codepoint <= 0x10FFFF => {
            seq.push(((codepoint >> 18) as u8 & 0x07) | 0xF0);
            seq.push(((codepoint >> 12) as u8 & 0x3F) | 0x80);
            seq.push(((codepoint >> 6) as u8 & 0x3F) | 0x80);
            seq.push(((codepoint) as u8 & 0x3F) | 0x80);
        },
        _ => {
            seq.push(0xEF);
            seq.push(0xBF);
            seq.push(0xBD);
        },
    }

    seq
}
