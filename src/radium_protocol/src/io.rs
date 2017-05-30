use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;
use super::errors::{ReadError, WriteError};

pub type WriteResult = Result<(), WriteError>;

#[doc(hidden)]
pub type ReadResult<T> = Result<T, ReadError>;

pub trait WriteTo: Sized {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult;
}

#[doc(hidden)]
pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self>;
}

#[doc(hidden)]
pub trait ReadValueExt: Sized + io::Read {
    fn read_value<R: ReadFrom>(&mut self) -> ReadResult<R> {
        R::read_from(self)
    }
}

pub trait WriteValueExt: Sized + io::Write {
    fn write_value<R: WriteTo>(&mut self, value: &R) -> WriteResult {
        value.write_to(self)?;

        Ok(())
    }
}

#[doc(hidden)]
impl<R> ReadValueExt for R where R: io::Read {}

impl<W> WriteValueExt for W where W: io::Write {}