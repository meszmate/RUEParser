#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    None = 0,
    Zlib = 1,
    Gzip = 2,
    Custom = 3,
    Oodle = 4,
    LZ4 = 5,
    Zstd = 6,
    Unknown = 7,
}
impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionMethod::None,
            1 => CompressionMethod::Zlib,
            2 => CompressionMethod::Gzip,
            3 => CompressionMethod::Custom,
            4 => CompressionMethod::Oodle,
            5 => CompressionMethod::LZ4,
            6 => CompressionMethod::Zstd,
            _ => CompressionMethod::Unknown,
        }
    }
}
