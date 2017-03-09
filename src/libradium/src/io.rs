use std::io::{Read, Write, Result as IoResult, Error as IoError};

pub type WriteToResult = IoResult<()>;

#[derive(Debug)]
pub struct Error {}

impl From<IoError> for Error {
    fn from(_: IoError) -> Self {
        Error {}
    }
}

pub trait ReadFrom: Sized {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error>;
}

pub trait WriteTo {
    fn write_to<W: Write + Sized>(&self, write: &mut W) -> WriteToResult;
}