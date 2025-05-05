use std::io;

pub enum Error {
    ReadError(io::Error),
}
