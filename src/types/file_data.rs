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
        self.contents
            .iter()
            .map(|&byte| format!("0x{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn content_as_string(&self) -> String {
        String::from_utf8_lossy(&self.contents).to_string()
    }
}
