pub fn encode_text(message: &str, key: &str) -> String {
    (0..message.len())
        .map(|index| {
            let ci = (
                message.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                +
                key.chars().nth(index).unwrap().to_ascii_uppercase() as i32
            ) % 26;
            
            ((ci + 1) as u8 + 64u8) as char
        })
        .collect()
}

pub fn decode_text(ciphertext: &str, keyword: &str) -> String {
    (0..ciphertext.len())
        .map(|index| {
            let mi = (
                ciphertext.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                - 
                keyword.chars().nth(index).unwrap().to_ascii_uppercase() as i32
                +
                26
            ) % 26;

            ((mi + 1) as u8 + 64u8) as char
        })
        .collect()
}

pub fn generate_key(message: &str, keyword: &str) -> String {
    let keyword = keyword.to_ascii_uppercase();
    (0..message.len())
        .map(|index| keyword.chars().nth(index % keyword.len()).unwrap())
        .collect()
}

fn main() {
    let message = "I need bout tree fiddy";
    let keyword = "SNEED";

    let key = generate_key(&message, &keyword);
    let encoded = encode_text(&message, &key);

    println!("{}", encoded);
    println!("{}", decode_text(&encoded, &key));
}
