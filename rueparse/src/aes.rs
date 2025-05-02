use std::string::FromUtf8Error;

use hex;
pub struct FAesKey {
    key: Vec<u8>,
    pub key_string: String,
}

impl FAesKey {
    pub fn from_str(key: &str) -> Result<Self, hex::FromHexError> {
        let new_key: String = if !key.starts_with("0x") {
            format!("0x{}", key)
        } else {
            key.to_string()
        };

        let hex_str = new_key.strip_prefix("0x").unwrap_or(&new_key);
        let key_bytes = match hex::decode(hex_str) {
            Ok(bytes) => bytes,
            Err(e) => return Err(e),
        };

        Ok(FAesKey {
            key: key_bytes,
            key_string: new_key,
        })
    }
    pub fn from_bytes(key: Vec<u8>) -> Result<Self, FromUtf8Error> {
        let key_string: String = match String::from_utf8(key.clone()) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        Ok(FAesKey {
            key,
            key_string: format!("0x{}", key_string),
        })
    }
}
