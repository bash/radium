use std::io::{Read, Write, Result as IoResult};

#[derive(Debug)]
pub struct Error {}

pub trait Readable: Sized {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error>;
}

pub trait Writable: Sized {
    fn write_to<W: Write>(&self, write: &mut W) -> IoResult<()>;
}