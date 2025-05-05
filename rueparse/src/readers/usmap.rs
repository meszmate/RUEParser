use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

use super::Reader;
use crate::mappings::EUsmapVersion;

pub struct FUsmapReader<'a> {
    cursor: Cursor<&'a [u8]>,
    pub version: EUsmapVersion,
}

impl<'a> FUsmapReader<'a> {
    pub fn new(data: &'a [u8], version: EUsmapVersion) -> Self {
        Self {
            cursor: Cursor::new(data),
            version,
        }
    }
    pub fn read_name(&mut self, names: &Vec<String>) -> String {
        let name_entry: i32 = self.read_i32().unwrap();
        if name_entry != -1 {
            return names.get(name_entry as usize).cloned().unwrap();
        } else {
            return String::new();
        }
    }
}

impl<'a> Reader for FUsmapReader<'a> {
    fn read_u8(&mut self) -> io::Result<u8> {
        self.cursor.read_u8()
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        self.cursor.read_u16::<LittleEndian>()
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        self.cursor.read_u32::<LittleEndian>()
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        self.cursor.read_i32::<LittleEndian>()
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        self.cursor.read_u64::<LittleEndian>()
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        self.cursor.read_i64::<LittleEndian>()
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.cursor.read_exact(buf)
    }

    fn seek(&mut self, pos: u64) -> io::Result<u64> {
        self.cursor.seek(SeekFrom::Start(pos))
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        match self.read_i32() {
            Ok(u) => match u {
                0 => return Ok(false),
                1 => return Ok(true),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid TOC magic",
                    ));
                }
            },
            Err(e) => return Err(e),
        }
    }
}
