use std::io;
use super::errors::{ReadError, WriteError};

pub type WriteResult = Result<(), WriteError>;
pub type ReadResult<T> = Result<T, ReadError>;

pub trait WriteTo: Sized {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult;
}

pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self>;
}

pub trait ReadValue: Sized + io::Read {
    fn read_value<R: ReadFrom>(&mut self) -> ReadResult<R> {
        R::read_from(self)
    }
}

pub trait WriteValue: Sized + io::Write {
    fn write_value<R: WriteTo>(&mut self, value: &R) -> WriteResult {
        value.write_to(self)?;

        Ok(())
    }
}

impl<R> ReadValue for R where R: io::Read {}
impl<W> WriteValue for W where W: io::Write {}