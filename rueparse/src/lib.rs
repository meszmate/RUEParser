mod aes;
mod reader;
use std::collections::HashMap;

use aes::FAesKey;
use hex::FromHexError;
use reader::FGuid;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct UEParse {
    pub keys: HashMap<FGuid, FAesKey>,
    pub path: String,
}

impl UEParse {
    pub fn new(path: &str) -> UEParse {
        UEParse {
            keys: HashMap::new(),
            path: String::from(path),
        }
    }
    pub fn add_key(&mut self, guid: FGuid, key: FAesKey) -> Result<(), FromHexError> {
        self.keys.insert(guid, key);
        Ok(())
    }
    pub fn clear_keys(&mut self) -> () {
        self.keys = HashMap::new();
    }
    pub fn remove_key(&mut self, guid: &FGuid) -> Option<FAesKey> {
        self.keys.remove(guid)
    }
}

#[cfg(test)]
mod tests {
    use crate::{aes::FAesKey, reader::FIoStoreTocHeader};

    use super::*;

    #[test]
    fn it_works() {
        let path = "/Volumes/DELIVERZ/Paks/pakchunk1001-WindowsClient.utoc";
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open file {}: {}", path, e),
        };

        let header = match FIoStoreTocHeader::from_reader(&mut file) {
            Ok(h) => h,
            Err(e) => panic!("failed to read header: {}", e),
        };
        println!("{:?}", header);
        println!("{}", header.encryption_key_guid.to_hex());
    }
}
