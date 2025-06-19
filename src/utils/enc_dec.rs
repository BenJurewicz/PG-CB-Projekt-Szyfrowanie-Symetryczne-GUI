use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, StreamCipher};
use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use sha2::Sha256;

use aes::cipher::KeyInit;
use ctr::cipher::KeyIvInit;

#[derive(Copy, Clone)]
pub enum CipherMode {
    Ecb,
    Cbc,
    Ctr,
}

pub fn encrypt_data(passwd: &str, data: &[u8], mode: CipherMode) -> Vec<u8> {
    let pd = PasswordData::new_for_encrypt(passwd);
    let encrypted_data = match mode {
        CipherMode::Ecb => encrypt_ecb(pd, data),
        CipherMode::Cbc => encrypt_cbc(pd, data),
        CipherMode::Ctr => encrypt_ctr(pd, data),
    };
    EncryptedData::new(mode, pd.salt, &encrypted_data).into()
}

pub fn decrypt_data(passwd: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    let ed = EncryptedData::try_from(data.to_vec())
        .map_err(|e| format!("Failed to parse encrypted data: {}", e))?;
    let pd = PasswordData::new_for_decrypt(passwd, ed.salt);
    match ed.mode {
        CipherMode::Ecb => decrypt_ecb(pd, &ed.data),
        CipherMode::Cbc => decrypt_cbc(pd, &ed.data),
        CipherMode::Ctr => decrypt_ctr(pd, &ed.data),
    }
}

impl From<CipherMode> for u8 {
    fn from(mode: CipherMode) -> Self {
        match mode {
            CipherMode::Ecb => 0,
            CipherMode::Cbc => 1,
            CipherMode::Ctr => 2,
        }
    }
}

impl TryFrom<u8> for CipherMode {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(CipherMode::Ecb),
            1 => Ok(CipherMode::Cbc),
            2 => Ok(CipherMode::Ctr),
            _ => Err(format!("Invalid mode byte: {}", byte)),
        }
    }
}

#[derive(Copy, Clone)]
struct PasswordData {
    key: [u8; 16], // 128-bit key for AES-128
    iv: [u8; 16],  // Initialization vector for CBC and CTR modes
    salt: [u8; 16],
}

impl PasswordData {
    fn new(password: &str, salt: [u8; 16]) -> PasswordData {
        let derived_key = pbkdf2_hmac_array::<Sha256, 32>(
            password.as_bytes(),
            &salt,
            1000
        );

        let mut key = [0u8; 16];
        let mut iv = [0u8; 16];

        // Split the derived key into key and IV
        key.copy_from_slice(&derived_key[0..16]);
        iv.copy_from_slice(&derived_key[16..32]);

        PasswordData {
            key,
            iv,
            salt,
        }
    }

    fn new_for_encrypt(password: &str) -> PasswordData {
        let salt = rand::rng().random::<[u8; 16]>();
        PasswordData::new(password, salt)
    }

    fn new_for_decrypt(password: &str, salt: [u8; 16]) -> PasswordData {
        PasswordData::new(password, salt)
    }
}

struct EncryptedData {
    mode: CipherMode,
    salt: [u8; 16],
    data: Vec<u8>,
}

impl EncryptedData {
    fn new(mode: CipherMode, salt: [u8;16], raw_data: &[u8]) -> Self {
        EncryptedData {
            mode,
            salt,
            data: raw_data.to_vec(),
        }
    }
}

impl From<EncryptedData> for Vec<u8> {
    fn from(ed: EncryptedData) -> Self {
        // 1 byte for mode, 16 bytes for salt, and the rest is data
        // 1 + 16 + data.len() = 17 + data.len()
        let mut res = Vec::with_capacity(17 + ed.data.len());
        res.push(u8::from(ed.mode));
        res.extend_from_slice(&ed.salt);
        res.extend(ed.data);
        res
    }
}

impl TryFrom<Vec<u8>> for EncryptedData {
    type Error = String;

    fn try_from(raw_data: Vec<u8>) -> Result<Self, Self::Error> {
        if raw_data.len() < 17 {
            return Err("Raw data for is too short to contain mode, salt and some data".to_string());
        }

        let mode = CipherMode::try_from(raw_data[0])?;

        let mut salt = [0u8; 16];
        salt.copy_from_slice(&raw_data[1..17]);

        let data = raw_data[17..].to_vec();

        Ok(EncryptedData {
            mode,
            salt,
            data
        })
    }
}

fn encrypt_ecb(pd: PasswordData, data: &[u8]) -> Vec<u8> {
    type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

    Aes128EcbEnc::new(&pd.key.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data)
}

fn decrypt_ecb(pd: PasswordData, data: &[u8]) -> Result<Vec<u8>, String> {
    type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

    Aes128EcbDec::new(&pd.key.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .map_err(|_| "ECB Decryption failed".to_string())
}

fn encrypt_cbc(pd: PasswordData, data: &[u8]) -> Vec<u8> {
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

    Aes128CbcEnc::new(&pd.key.into(), &pd.iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data)
}

fn decrypt_cbc(pd: PasswordData, data: &[u8]) -> Result<Vec<u8>, String> {
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    Aes128CbcDec::new(&pd.key.into(), &pd.iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .map_err(|_| "CBC Decryption failed".to_string())
}

fn encrypt_ctr(pd: PasswordData, data: &[u8]) -> Vec<u8> {
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

    let mut cipher = Aes128Ctr64LE::new(&pd.key.into(), &pd.iv.into());
    let mut res = data.to_vec();
    cipher.apply_keystream(&mut res);
    res
}

fn decrypt_ctr(pd: PasswordData, data: &[u8]) -> Result<Vec<u8>, String> {
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

    let mut cipher = Aes128Ctr64LE::new(&pd.key.into(), &pd.iv.into());
    let mut res = data.to_vec();
    cipher.apply_keystream(&mut res);
    Ok(res)
}