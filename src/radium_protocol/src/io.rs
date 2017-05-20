use std::io;
use super::ReadError;

pub trait WriteTo: Sized {
    // TODO: use own error type instead of io::Error
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()>;
}

pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError>;
}

pub trait ReadValue: Sized + io::Read {
    fn read_value<R: ReadFrom>(&mut self) -> Result<R, ReadError> {
        R::read_from(self)
    }
}

pub trait WriteValue: Sized + io::Write {
    fn write_value<R: WriteTo>(&mut self, value: &R) -> io::Result<()> {
        value.write_to(self)
    }
}

impl<R> ReadValue for R where R: io::Read {}
impl<W> WriteValue for W where W: io::Write {}