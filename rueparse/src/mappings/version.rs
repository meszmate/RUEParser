use std::io::{self, Read, Seek};

use crate::reader::{FGuid, Reader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EUsmapVersion {
    Initial,
    PackageVersioning,
    LongFName,
    LargeEnums,
}

impl EUsmapVersion {
    pub const LATEST: EUsmapVersion = EUsmapVersion::LargeEnums;
}

impl From<u8> for EUsmapVersion {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return EUsmapVersion::Initial,
            1 => return EUsmapVersion::PackageVersioning,
            2 => return EUsmapVersion::LongFName,
            3 => return EUsmapVersion::LargeEnums,
            _ => return EUsmapVersion::LATEST,
        };
    }
}

pub struct FPackageFileVersion {
    pub file_version_ue4: i32,
    pub file_version_ue5: i32,
}

impl FPackageFileVersion {
    pub fn from_reader<R: Read + Seek>(reader: &mut Reader<R>) -> io::Result<Self> {
        let file_version_ue4 = reader.read_i32()?;
        let file_version_ue5 = reader.read_i32()?;
        Ok(Self {
            file_version_ue4,
            file_version_ue5,
        })
    }
    pub fn default() -> Self {
        Self {
            file_version_ue4: 0,
            file_version_ue5: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ECustomVersionSerializationFormat {
    Unknown,
    Guids,
    Enums,
    Optimized,
}
impl ECustomVersionSerializationFormat {
    pub const LATEST: ECustomVersionSerializationFormat =
        ECustomVersionSerializationFormat::Optimized;
}

pub struct FCustomVersion {
    pub key: FGuid,
    pub version: i32,
}
impl FCustomVersion {
    pub fn from_reader<R: Read + Seek>(reader: &mut Reader<R>) -> io::Result<Self> {
        Ok(Self {
            key: FGuid::from_reader(reader)?,
            version: reader.read_i32()?,
        })
    }
}

pub struct FCustomVersionContainer {
    pub versions: Vec<FCustomVersion>,
}
impl FCustomVersionContainer {
    pub fn new<R: Read + Seek>(
        reader: &mut Reader<R>,
        format: Option<ECustomVersionSerializationFormat>,
    ) -> io::Result<Self> {
        let f = format.unwrap_or(ECUSTOMVERSIONSERIALIZATIONFORMAT_LATEST);
        match f {
            ECustomVersionSerializationFormat::Optimized => {
                let length = reader.read_i32()?;
                let mut versions: Vec<FCustomVersion> = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    versions.push(match FCustomVersion::from_reader(reader) {
                        Ok(f) => f,
                        Err(e) => return Err(e),
                    });
                }
                return Ok(Self { versions });
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Format not supported",
                ));
            }
        }
    }
    pub fn default() -> Self {
        Self {
            versions: Vec::new(),
        }
    }
}
