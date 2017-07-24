use std::io;
use super::errors::WriteError;

pub type WriteResult = Result<(), WriteError>;

pub trait WriteTo: Sized {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult;
}

pub trait WriteValueExt: Sized + io::Write {
    fn write_value<R: WriteTo>(&mut self, value: &R) -> WriteResult {
        value.write_to(self)?;

        Ok(())
    }
}

impl<W> WriteValueExt for W where W: io::Write {}
