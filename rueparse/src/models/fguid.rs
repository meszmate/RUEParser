use crate::readers::Reader;
use std::fmt;
use std::io;

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
    pub fn from_reader(reader: &mut dyn Reader) -> io::Result<Self> {
        Ok(Self {
            a: reader.read_u32()?,
            b: reader.read_u32()?,
            c: reader.read_u32()?,
            d: reader.read_u32()?,
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
