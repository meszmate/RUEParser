use std::io::{self, Read, Seek};

use crate::models::FGuid;
use crate::readers::Reader;

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
    pub fn from_reader(reader: &mut dyn Reader) -> io::Result<Self> {
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
