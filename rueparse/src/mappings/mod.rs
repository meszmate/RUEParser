use crate::models::FGuid;
use crate::readers::FUsmapReader;
use crate::readers::FileReader;
use crate::readers::Reader;
use brotli;
use byteorder::{LittleEndian, ReadBytesExt};
use compression::EUsmapCompressionMethod;
use oodle_safe;
use std::collections::HashMap;
use std::convert;
use std::io::Cursor;
use std::io::{self, Read, Seek};
use std::rc::Rc;

mod compression;
mod epropertytype;
mod properties;
mod version;

pub use compression::*;
pub use epropertytype::*;
pub use properties::*;
pub use version::*;

#[derive(Debug, Clone)]
pub struct TypeMappings {
    pub types: HashMap<String, Rc<Struct>>,
    pub enums: HashMap<String, HashMap<i32, String>>,
}
impl TypeMappings {
    pub fn new(
        types: HashMap<String, Rc<Struct>>,
        enums: HashMap<String, HashMap<i32, String>>,
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
    pub fn from_reader(reader: &mut dyn Reader) -> io::Result<Self> {
        const EXPECTED_MAGIC: u16 = 0x30C4;
        let magic: u16 = reader.read_u16()?;
        if EXPECTED_MAGIC != magic {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Magic"));
        }
        let version_byte: u8 = reader.read_u8()?;
        let version: EUsmapVersion = EUsmapVersion::from(version_byte);
        let package_version: FPackageFileVersion;
        let custom_versions: FCustomVersionContainer;

        let netcl: u32;
        if version as u8 >= EUsmapVersion::PackageVersioning as u8
            && match reader.read_bool() {
                Ok(b) => b,
                Err(e) => return Err(e),
            }
        {
            package_version = match FPackageFileVersion::from_reader(reader) {
                Ok(f) => f,
                Err(e) => return Err(e),
            };
            custom_versions = match FCustomVersionContainer::new(reader, None) {
                Ok(f) => f,
                Err(e) => return Err(e),
            };
            netcl = reader.read_u32()?;
        } else {
            package_version = FPackageFileVersion::default();
            custom_versions = FCustomVersionContainer::default();
            netcl = 0;
        }
        let compression_method_byte: u8 = reader.read_u8()?;
        let compression_method: EUsmapCompressionMethod =
            EUsmapCompressionMethod::from(compression_method_byte);

        let comp_size: u32 = reader.read_u32()?;
        let decomp_size: u32 = reader.read_u32()?;

        let mut data = vec![0u8; decomp_size as usize];

        match compression_method {
            EUsmapCompressionMethod::None => {
                if (comp_size != decomp_size) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No compression: Compression size must be equal to decompression size",
                    ));
                }
                reader.read_exact(&mut data[..comp_size as usize]);
            }
            EUsmapCompressionMethod::Oodle => {
                let mut comp_bytes = vec![0u8; comp_size as usize];
                reader.read_exact(&mut comp_bytes[..comp_size as usize])?;
                oodle_safe::decompress(&comp_bytes, &mut data, None, None, None, None);
            }
            EUsmapCompressionMethod::Brotli => {
                let mut comp_bytes = vec![0u8; comp_size as usize];
                reader.read_exact(&mut comp_bytes[..comp_size as usize])?;
                decompress_brotli(&comp_bytes, &mut data);
            }
            EUsmapCompressionMethod::ZStandart => {
                let mut comp_bytes = vec![0u8; comp_size as usize];
                reader.read_exact(&mut comp_bytes[..comp_size as usize])?;
                zstd::bulk::decompress_to_buffer(&comp_bytes, &mut data)?;
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid compression method",
                ));
            }
        }
        let mut reader = FUsmapReader::new(&mut data, version);
        let name_size: u32 = reader.read_u32()?;
        let mut name_lut: Vec<String> = Vec::with_capacity(name_size as usize);
        for _ in 0..name_size {
            let name_length: usize = if reader.version as u8 >= EUsmapVersion::LongFName as u8 {
                let name_byte = reader.read_u16()?;
                name_byte as usize
            } else {
                let name_byte = reader.read_u8()?;
                name_byte as usize
            };
            let mut name_bytes = vec![0u8; name_length as usize];
            reader.read_exact(&mut name_bytes);
            let name = match String::from_utf8(name_bytes.to_vec()) {
                Ok(s) => s,
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                }
            };
            name_lut.push(name);
        }

        let enum_count: u32 = reader.read_u32()?;
        let mut enums: HashMap<String, HashMap<i32, String>> = HashMap::new();
        for _ in 0..enum_count {
            let enum_name = reader.read_name(&name_lut);

            let enum_names_length: usize = if reader.version as u8 >= EUsmapVersion::LongFName as u8
            {
                let name_byte = reader.read_u16()?;
                name_byte as usize
            } else {
                let name_byte = reader.read_u8()?;
                name_byte as usize
            };
            let mut enum_names: HashMap<i32, String> = HashMap::with_capacity(enum_names_length);
            for i in 0..enum_names_length {
                enum_names.insert(i as i32, reader.read_name(&name_lut));
            }
            enums.insert(enum_name, enum_names);
        }

        Ok(Self {
            magic,
            version,
            package_version,
            compression_method,
            custom_versions,
            netcl,
        })
    }
}
