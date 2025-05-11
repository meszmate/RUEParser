pub mod assets;
pub mod compression;
mod errors;
pub mod fileprovider;
pub mod mappings;
pub mod models;
pub mod objects;
pub mod readers;
pub mod ue;
pub mod versions;

use mappings::UsmapProvider;
use oodle::Oodle;
use std::io;
use std::{collections::HashMap, fs, path::Path};

use fileprovider::objects::{DirectoryInfo, FileInfo, GameFile, OsGameFile};
use hex::FromHexError;
use models::{FAesKey, FGuid};
pub use versions::*;

pub struct UEParse {
    pub mappings: Option<UsmapProvider>,
    pub oodle: Option<Oodle>,
    pub keys: HashMap<FGuid, FAesKey>,
    pub working_directory: DirectoryInfo,
}

impl UEParse {
    pub fn new(path: &str) -> io::Result<UEParse> {
        Ok(UEParse {
            mappings: None,
            oodle: None,
            keys: HashMap::new(),
            working_directory: match DirectoryInfo::new(&path) {
                Ok(d) => d,
                Err(e) => return Err(e),
            },
        })
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

    fn iterate_files(
        &mut self,
        directory: &Path,
        recursive: bool,
    ) -> Result<HashMap<String, OsGameFile>, io::Error> {
        let mut os_files: HashMap<String, OsGameFile> = HashMap::new();
        if !directory.exists() || !directory.is_dir() {
            return Ok(os_files);
        }

        let uproject = fs::read_dir(directory).ok().and_then(|entries| {
            entries.filter_map(Result::ok).find(|entry| {
                entry.path().is_file()
                    && entry.path().extension().and_then(|e| e.to_str()) == Some("uproject")
            })
        });

        let mount_point = if let Some(ref entry) = uproject {
            entry
                .path()
                .file_stem()
                .map(|s| format!("{}/", s.to_string_lossy()))
                .unwrap_or_else(|| "Unknown/".to_string())
        } else {
            directory
                .file_name()
                .map(|s| format!("{}/", s.to_string_lossy()))
                .unwrap_or_else(|| "Unknown/".to_string())
        };

        let should_recurse = uproject.is_some() || recursive;

        let entries = if should_recurse {
            walkdir::WalkDir::new(directory)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
                .map(|e| e.path().to_path_buf())
                .collect::<Vec<_>>()
        } else {
            match fs::read_dir(directory) {
                Ok(read_dir) => read_dir
                    .filter_map(Result::ok)
                    .filter(|e| e.path().is_file())
                    .map(|e| e.path())
                    .collect(),
                Err(e) => return Err(e),
            }
        };
        for file in entries {
            match file.to_str() {
                Some(f) => {
                    os_files.insert(
                        f.to_string(),
                        OsGameFile::new(
                            self.working_directory,
                            FileInfo::new(&f),
                            mount_point,
                            versions,
                        ),
                    );
                }
                None => {}
            }
        }
        Ok(os_files)
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
