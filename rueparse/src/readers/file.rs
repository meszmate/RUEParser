use crate::models::FGuid;
use crate::readers::Reader;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::io::{self, Read, Seek, SeekFrom};

pub struct FileReader<R: Read + Seek> {
    inner: R,
}
impl<R: Read + Seek> FileReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}
impl<R: Read + Seek> Reader for FileReader<R> {
    fn read_u8(&mut self) -> io::Result<u8> {
        self.inner.read_u8()
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        self.inner.read_u16::<LittleEndian>()
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        self.inner.read_i32::<LittleEndian>()
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        self.inner.read_u32::<LittleEndian>()
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        self.inner.read_u64::<LittleEndian>()
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        self.inner.read_i64::<LittleEndian>()
    }

    fn read_exact(&mut self, o: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(o)
    }

    fn seek(&mut self, pos: u64) -> io::Result<u64> {
        self.inner.seek(SeekFrom::Start(pos))
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
    pub fn from_reader(reader: &mut dyn Reader) -> io::Result<Self> {
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
            reserved1: reader.read_u16()?,
            toc_header_size: reader.read_u32()?,
            toc_entry_count: reader.read_u32()?,
            toc_compressed_block_entry_count: reader.read_u32()?,
            toc_compressed_block_entry_size: reader.read_u32()?,
            compression_method_name_count: reader.read_u32()?,
            compression_method_name_length: reader.read_u32()?,
            compression_block_size: reader.read_u32()?,
            directory_index_size: reader.read_u32()?,
            partition_count: reader.read_u32()?,
            container_id: FIoContainerId(reader.read_u64()?),
            encryption_key_guid: FGuid::from_reader(reader)?,
            container_flags: reader.read_u8()?,
            reserved3: reader.read_u8()?,
            reserved4: reader.read_u16()?,
            toc_chunk_perfect_hash_seeds_count: reader.read_u32()?,
            partition_size: reader.read_u64()?,
            toc_chunks_without_perfect_hash_count: reader.read_u32()?,
            reserved7: reader.read_u32()?,
            reserved8: {
                let mut arr = [0u64; 5];
                for val in &mut arr {
                    *val = reader.read_u64()?;
                }
                arr
            },
        })
    }
}
