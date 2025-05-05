use crate::readers::FUsmapReader;
use crate::readers::FileReader;
use crate::readers::Reader;
use brotli;
use compression::EUsmapCompressionMethod;
use oodle;
use oodle::Oodle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Cursor, Read};
use std::rc::Rc;
use std::string::FromUtf8Error;

mod compression;
mod epropertytype;
mod properties;
mod version;

pub use epropertytype::*;
pub use properties::*;
pub use version::*;

#[derive(Debug)]
pub enum UsmapParserError {
    InvalidMagic,
    CompressionSizeEquality,
    OodleNotFound,
    InvalidCompressionMethod,
    ReadError(io::Error),
    FromUtf8Error(FromUtf8Error),
    FileOpenError(io::Error),
}

#[derive(Debug)]
pub struct UsmapProvider {
    pub mappings_for_game: TypeMappings,
}

impl UsmapProvider {
    pub fn from_path(path: &str, oo: &Oodle) -> Result<Self, UsmapParserError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(UsmapParserError::FileOpenError(e)),
        };
        let mut reader = FileReader::new(file);
        let usmap = match UsmapParser::from_reader(&mut reader, Some(&oo)) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };
        Ok(Self {
            mappings_for_game: usmap.mappings,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TypeMappings {
    pub types: Rc<RefCell<HashMap<String, Box<Struct>>>>,
    pub enums: Rc<RefCell<HashMap<String, HashMap<i32, String>>>>,
}
impl TypeMappings {
    pub fn new(
        types: Rc<RefCell<HashMap<String, Box<Struct>>>>,
        enums: Rc<RefCell<HashMap<String, HashMap<i32, String>>>>,
    ) -> Self {
        Self { types, enums }
    }
}

pub struct UsmapParser {
    magic: u16,
    version: EUsmapVersion,
    package_version: FPackageFileVersion,
    compression_method: EUsmapCompressionMethod,
    custom_versions: FCustomVersionContainer,
    mappings: TypeMappings,
    netcl: u32,
}

fn decompress_brotli(
    compressed_data: &[u8],
    decompressed_buffer: &mut Vec<u8>,
) -> std::io::Result<()> {
    let mut decompressor = brotli::Decompressor::new(Cursor::new(compressed_data), 4096);
    decompressor.read_to_end(decompressed_buffer)?;
    Ok(())
}

impl UsmapParser {
    pub fn from_reader(
        reader: &mut dyn Reader,
        oo: Option<&oodle::Oodle>,
    ) -> Result<Self, UsmapParserError> {
        const EXPECTED_MAGIC: u16 = 0x30C4;
        let magic: u16 = match reader.read_u16() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        if EXPECTED_MAGIC != magic {
            return Err(UsmapParserError::InvalidMagic);
        }
        let version_byte: u8 = match reader.read_u8() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let version: EUsmapVersion = EUsmapVersion::from(version_byte);
        let package_version: FPackageFileVersion;
        let custom_versions: FCustomVersionContainer;
        let netcl: u32;
        if version as u8 >= EUsmapVersion::PackageVersioning as u8
            && match reader.read_bool() {
                Ok(b) => b,
                Err(e) => return Err(UsmapParserError::ReadError(e)),
            }
        {
            package_version = match FPackageFileVersion::from_reader(reader) {
                Ok(f) => f,
                Err(e) => return Err(UsmapParserError::ReadError(e)),
            };
            custom_versions = match FCustomVersionContainer::new(reader, None) {
                Ok(f) => f,
                Err(e) => return Err(UsmapParserError::ReadError(e)),
            };
            netcl = match reader.read_u32() {
                Ok(r) => r,
                Err(e) => return Err(UsmapParserError::ReadError(e)),
            };
        } else {
            package_version = FPackageFileVersion::default();
            custom_versions = FCustomVersionContainer::default();
            netcl = 0;
        }
        let compression_method_byte: u8 = match reader.read_u8() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let compression_method: EUsmapCompressionMethod =
            EUsmapCompressionMethod::from(compression_method_byte);

        let comp_size: u32 = match reader.read_u32() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let decomp_size: u32 = match reader.read_u32() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };

        let mut data = vec![0u8; decomp_size as usize];
        println!("{:?} {:?} {:?}", compression_method, comp_size, decomp_size);
        match compression_method {
            EUsmapCompressionMethod::None => {
                if (comp_size != decomp_size) {
                    return Err(UsmapParserError::CompressionSizeEquality);
                }
                match reader.read_exact(&mut data[..comp_size as usize]) {
                    Ok(_) => {}
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
            }
            EUsmapCompressionMethod::Oodle => match oo {
                Some(o) => {
                    let mut comp_bytes = vec![0u8; comp_size as usize];
                    match reader.read_exact(&mut comp_bytes[..comp_size as usize]) {
                        Ok(_) => {}
                        Err(e) => return Err(UsmapParserError::ReadError(e)),
                    };
                    o.decompress(&comp_bytes, &mut data);
                }
                _ => {
                    return Err(UsmapParserError::OodleNotFound);
                }
            },
            EUsmapCompressionMethod::Brotli => {
                let mut comp_bytes = vec![0u8; comp_size as usize];
                match reader.read_exact(&mut comp_bytes[..comp_size as usize]) {
                    Ok(_) => {}
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                match decompress_brotli(&comp_bytes, &mut data) {
                    Ok(_) => {}
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
            }
            EUsmapCompressionMethod::ZStandart => {
                let mut comp_bytes = vec![0u8; comp_size as usize];
                match reader.read_exact(&mut comp_bytes[..comp_size as usize]) {
                    Ok(_) => {}
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                match zstd::bulk::decompress_to_buffer(&comp_bytes, &mut data) {
                    Ok(_) => {}
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
            }
            _ => {
                return Err(UsmapParserError::InvalidCompressionMethod);
            }
        }
        let mut reader = FUsmapReader::new(&mut data, version);
        let name_size: u32 = match reader.read_u32() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let mut name_lut: Vec<String> = Vec::with_capacity(name_size as usize);
        for _ in 0..name_size {
            let name_length: usize = if reader.version as u8 >= EUsmapVersion::LongFName as u8 {
                let name_byte = match reader.read_u16() {
                    Ok(r) => r,
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                name_byte as usize
            } else {
                let name_byte = match reader.read_u8() {
                    Ok(r) => r,
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                name_byte as usize
            };
            let mut name_bytes = vec![0u8; name_length as usize];
            reader.read_exact(&mut name_bytes);
            let name = match String::from_utf8(name_bytes.to_vec()) {
                Ok(s) => s,
                Err(e) => {
                    return Err(UsmapParserError::FromUtf8Error(e));
                }
            };
            name_lut.push(name);
        }

        let enum_count: u32 = match reader.read_u32() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let enums: Rc<RefCell<HashMap<String, HashMap<i32, String>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        for _ in 0..enum_count {
            let enum_name = reader.read_name(&name_lut);

            let enum_names_length: usize = if reader.version as u8 >= EUsmapVersion::LongFName as u8
            {
                let name_byte = match reader.read_u16() {
                    Ok(r) => r,
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                name_byte as usize
            } else {
                let name_byte = match reader.read_u8() {
                    Ok(r) => r,
                    Err(e) => return Err(UsmapParserError::ReadError(e)),
                };
                name_byte as usize
            };
            let mut enum_names: HashMap<i32, String> = HashMap::with_capacity(enum_names_length);
            for i in 0..enum_names_length {
                enum_names.insert(i as i32, reader.read_name(&name_lut));
            }
            enums.borrow_mut().insert(enum_name, enum_names);
        }

        let struct_count: u32 = match reader.read_u32() {
            Ok(r) => r,
            Err(e) => return Err(UsmapParserError::ReadError(e)),
        };
        let structs: Rc<RefCell<HashMap<String, Box<Struct>>>> =
            Rc::new(RefCell::new(HashMap::with_capacity(struct_count as usize)));
        let mappings: Rc<RefCell<TypeMappings>> = Rc::new(RefCell::new(TypeMappings::new(
            Rc::clone(&structs),
            Rc::clone(&enums),
        )));
        for _ in 0..struct_count {
            let s = Struct::parse(Some(Rc::clone(&mappings)), &mut reader, &name_lut);
            structs
                .borrow_mut()
                .insert(s.name.clone(), Box::new(s.clone()));
        }

        Ok(Self {
            magic,
            version,
            package_version,
            compression_method,
            custom_versions,
            mappings: mappings.borrow_mut().clone(),
            netcl,
        })
    }
}
