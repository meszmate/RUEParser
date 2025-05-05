mod compression;
mod errors;
mod fileprovider;
mod mappings;
mod models;
mod objects;
mod readers;

use mappings::UsmapProvider;
use oodle::Oodle;
use std::collections::HashMap;

use hex::FromHexError;
use models::{FAesKey, FGuid};

pub struct UEParse {
    pub mappings: Option<UsmapProvider>,
    pub oodle: Option<Oodle>,
    pub keys: HashMap<FGuid, FAesKey>,
    pub path: String,
}

impl UEParse {
    pub fn new(path: &str) -> UEParse {
        UEParse {
            mappings: None,
            oodle: None,
            keys: HashMap::new(),
            path: String::from(path),
        }
    }
    pub fn init_oodle(&mut self, path: &str) -> Result<(), oodle::Error> {
        self.oodle.insert(match Oodle::load(path) {
            Ok(o) => o,
            Err(e) => return Err(e),
        });
        Ok(())
    }

    pub fn add_mappings(&mut self, mappings: UsmapProvider) {
        self.mappings.insert(mappings);
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
    use crate::{
        mappings::UsmapProvider,
        readers::{FIoStoreTocHeader, FileReader},
    };
    use std::fs::File;

    #[test]
    fn it_works() {
        let path = "/Volumes/DELIVERZ/Paks/pakchunk1001-WindowsClient.utoc";
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open file {}: {}", path, e),
        };
        let mut reader = FileReader::new(file);

        let header = match FIoStoreTocHeader::from_reader(&mut reader) {
            Ok(h) => h,
            Err(e) => panic!("failed to read header: {}", e),
        };
        println!("{:?}", header);
        println!("{}", header.encryption_key_guid.to_hex());

        let usmap_path = "/Volumes/DELIVERZ/mappings.usmap";
        let oo = oodle::Oodle::load(&"/Volumes/DELIVERZ/liboo2coremac64.2.9.13.dylib").unwrap();
        let usmap = UsmapProvider::from_path(&usmap_path, &oo).unwrap();
        println!("{:?}", usmap);
    }
}
