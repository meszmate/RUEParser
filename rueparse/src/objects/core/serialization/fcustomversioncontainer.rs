use std::io;

use super::{ECustomVersionSerializationFormat, FCustomVersion};
use crate::readers::Reader;

#[derive(Debug)]
pub struct FCustomVersionContainer {
    pub versions: Vec<FCustomVersion>,
}
impl FCustomVersionContainer {
    pub fn default() -> Self {
        Self {
            versions: Vec::new(),
        }
    }
    pub fn from_versions(versions: Vec<FCustomVersion>) -> Self {
        Self { versions }
    }
    pub fn new(
        reader: &mut dyn Reader,
        format: Option<ECustomVersionSerializationFormat>,
    ) -> io::Result<Self> {
        let f = format.unwrap_or(ECustomVersionSerializationFormat::LATEST);
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
}
