use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// ========== FGuid ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FGuid {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

impl FGuid {
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        if hex.len() != 32 {
            return Err("Hex string must be 32 characters long");
        }
        let a = u32::from_str_radix(&hex[0..8], 16).map_err(|_| "Invalid hex format")?;
        let b = u32::from_str_radix(&hex[8..16], 16).map_err(|_| "Invalid hex format")?;
        let c = u32::from_str_radix(&hex[16..24], 16).map_err(|_| "Invalid hex format")?;
        let d = u32::from_str_radix(&hex[24..32], 16).map_err(|_| "Invalid hex format")?;

        Ok(FGuid::new(a, b, c, d))
    }
    fn from_str(s: &str) -> Result<Self, &'static str> {
        let clean = s.replace("-", "");

        let a = u32::from_str_radix(&clean[0..8], 16).map_err(|_| "Invalid hex format")?;
        let b1 = u16::from_str_radix(&clean[8..12], 16).map_err(|_| "Invalid hex format")?;
        let b2 = u16::from_str_radix(&clean[12..16], 16).map_err(|_| "Invalid hex format")?;
        let c1 = u16::from_str_radix(&clean[16..20], 16).map_err(|_| "Invalid hex format")?;
        let c2 = u16::from_str_radix(&clean[20..24], 16).map_err(|_| "Invalid hex format")?;
        let d = u32::from_str_radix(&clean[24..32], 16).map_err(|_| "Invalid hex format")?;

        Ok(FGuid {
            a,
            b: ((b1 as u32) << 16) | (b2 as u32),
            c: ((c1 as u32) << 16) | (c2 as u32),
            d,
        })
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            a: reader.read_u32::<LittleEndian>()?,
            b: reader.read_u32::<LittleEndian>()?,
            c: reader.read_u32::<LittleEndian>()?,
            d: reader.read_u32::<LittleEndian>()?,
        })
    }
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self { a, b, c, d }
    }
    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
    pub fn to_hex(&self) -> String {
        format!("{:08X}{:08X}{:08X}{:08X}", self.a, self.b, self.c, self.d)
    }
}

impl fmt::Display for FGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:04X}-{:04X}{:08X}",
            self.a,
            self.b >> 16,
            self.b & 0xFFFF,
            self.c >> 16,
            self.c & 0xFFFF,
            self.d
        )
    }
}
#[derive(Debug)]
pub struct FIoContainerId(pub u64);

impl fmt::Display for FIoContainerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum EIoContainerFlags {
    None = 0,
    Compressed = 1 << 0,
    Encrypted = 1 << 1,
    Signed = 1 << 2,
    Indexed = 1 << 3,
    OnDemand = 1 << 4,
}
#[derive(Debug)]
pub struct FIoStoreTocHeader {
    pub toc_magic: [u8; 16],
    pub version: u8,
    reserved0: u8,
    reserved1: u16,
    pub toc_header_size: u32,
    pub toc_entry_count: u32,
    pub toc_compressed_block_entry_count: u32,
    pub toc_compressed_block_entry_size: u32,
    pub compression_method_name_count: u32,
    pub compression_method_name_length: u32,
    pub compression_block_size: u32,
    pub directory_index_size: u32,
    pub partition_count: u32,
    pub container_id: FIoContainerId,
    pub encryption_key_guid: FGuid,
    pub container_flags: u8,
    reserved3: u8,
    reserved4: u16,
    pub toc_chunk_perfect_hash_seeds_count: u32,
    pub partition_size: u64,
    pub toc_chunks_without_perfect_hash_count: u32,
    reserved7: u32,
    reserved8: [u64; 5],
}

impl FIoStoreTocHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut toc_magic = [0u8; 16];
        reader.read_exact(&mut toc_magic)?;

        const EXPECTED_MAGIC: [u8; 16] = [
            0x2D, 0x3D, 0x3D, 0x2D, 0x2D, 0x3D, 0x3D, 0x2D, 0x2D, 0x3D, 0x3D, 0x2D, 0x2D, 0x3D,
            0x3D, 0x2D,
        ];

        if toc_magic != EXPECTED_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid TOC magic",
            ));
        }

        Ok(Self {
            toc_magic,
            version: reader.read_u8()?,
            reserved0: reader.read_u8()?,
            reserved1: reader.read_u16::<LittleEndian>()?,
            toc_header_size: reader.read_u32::<LittleEndian>()?,
            toc_entry_count: reader.read_u32::<LittleEndian>()?,
            toc_compressed_block_entry_count: reader.read_u32::<LittleEndian>()?,
            toc_compressed_block_entry_size: reader.read_u32::<LittleEndian>()?,
            compression_method_name_count: reader.read_u32::<LittleEndian>()?,
            compression_method_name_length: reader.read_u32::<LittleEndian>()?,
            compression_block_size: reader.read_u32::<LittleEndian>()?,
            directory_index_size: reader.read_u32::<LittleEndian>()?,
            partition_count: reader.read_u32::<LittleEndian>()?,
            container_id: FIoContainerId(reader.read_u64::<LittleEndian>()?),
            encryption_key_guid: FGuid::from_reader(reader)?,
            container_flags: reader.read_u8()?,
            reserved3: reader.read_u8()?,
            reserved4: reader.read_u16::<LittleEndian>()?,
            toc_chunk_perfect_hash_seeds_count: reader.read_u32::<LittleEndian>()?,
            partition_size: reader.read_u64::<LittleEndian>()?,
            toc_chunks_without_perfect_hash_count: reader.read_u32::<LittleEndian>()?,
            reserved7: reader.read_u32::<LittleEndian>()?,
            reserved8: {
                let mut arr = [0u64; 5];
                for val in &mut arr {
                    *val = reader.read_u64::<LittleEndian>()?;
                }
                arr
            },
        })
    }
}
