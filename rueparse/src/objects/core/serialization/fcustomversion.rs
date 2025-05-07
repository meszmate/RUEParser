use std::io;

use crate::models::FGuid;
use crate::readers::Reader;

#[derive(Debug)]
pub struct FCustomVersion {
    key: FGuid,
    version: i32,
}
impl FCustomVersion {
    pub fn new(key: FGuid, version: i32) -> Self {
        Self { key, version }
    }

    pub fn from_reader(reader: &mut dyn Reader) -> io::Result<Self> {
        Ok(Self {
            key: FGuid::from_reader(reader)?,
            version: reader.read_i32()?,
        })
    }
}

#[repr(u8)]
#[derive(Debug)]
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
