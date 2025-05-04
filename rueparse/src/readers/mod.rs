mod file;
mod usmap;
use std::io;

pub use file::*;
pub use usmap::*;

pub trait Reader {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;
    fn read_bool(&mut self) -> io::Result<bool>;
    fn seek(&mut self, pos: u64) -> io::Result<u64>;
}
