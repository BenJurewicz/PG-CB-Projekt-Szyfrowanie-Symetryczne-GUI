#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub name: String,
    pub contents: Vec<u8>,
}

impl FileData {
    pub fn new(name: String, contents: Vec<u8>) -> Self {
        Self { name, contents }
    }

    pub fn bin_as_hex_string(&self) -> String {
        let hexadecimal = hex::encode_upper(self.contents.clone());

        hexadecimal
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn content_as_string(&self) -> String {
        String::from_utf8_lossy(&self.contents).to_string()
    }
}
