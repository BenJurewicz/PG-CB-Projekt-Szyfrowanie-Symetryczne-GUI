pub fn vec_to_hex(vec: &[u8]) -> String {
    hex::encode_upper(vec)
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn hex_to_vec(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.replace(" ", "");
    hex::decode(hex).map_err(|e| e.to_string())
}
