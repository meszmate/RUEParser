#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EUsmapCompressionMethod {
    None,
    Oodle,
    Brotli,
    ZStandart,

    Unknown = 0xFF,
}

impl From<u8> for EUsmapCompressionMethod {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return EUsmapCompressionMethod::None,
            1 => return EUsmapCompressionMethod::Oodle,
            2 => return EUsmapCompressionMethod::Brotli,
            3 => return EUsmapCompressionMethod::ZStandart,
            _ => return EUsmapCompressionMethod::Unknown,
        };
    }
}
