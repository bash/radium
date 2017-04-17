use std::io;
use super::ReadError;

pub trait WriteTo: Sized {
    // TODO: use own error type instead of io::Error
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()>;
}

pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError>;
}