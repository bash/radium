use std::io;
use super::errors::{ReadError, WriteError};

////
//// FUCK THIS SHIT...
////

pub type WriteResult = Result<(), WriteError>;
pub type ReadResult<T> = Result<T, ReadError>;

pub trait WriteTo: Sized {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult;
}

pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self>;
}

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

impl<R> ReadValueExt for R where R: io::Read {}
impl<W> WriteValueExt for W where W: io::Write {}


////
//// ...HERE COMES THE NEW SHIT
////

pub enum ReaderStatus<T> {
    Pending,
    Complete(T),
    Ended,
}

#[derive(Debug)]
pub struct ReaderController<S> {
    inner: S
}

pub trait Reader<T, I>: Sized {
    fn resume(&mut self, input: &mut I) -> io::Result<ReaderStatus<T>>;
    fn rewind(&mut self);
}

impl<S> ReaderController<S> {
    pub fn new(inner: S) -> Self {
        ReaderController { inner }
    }

    pub fn resume<T, I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<T>> where S: Reader<T, I> {
        match self.inner.resume(input) {
            Ok(val) => {
                match val {
                    ReaderStatus::Complete(val) => Ok(ReaderStatus::Complete(val)),
                    ReaderStatus::Pending => self.resume(input),
                    ReaderStatus::Ended => Ok(ReaderStatus::Ended)
                }
            }
            Err(err) => {
                if let io::ErrorKind::WouldBlock = err.kind() {
                    Ok(ReaderStatus::Pending)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn rewind<T, I>(&mut self) where S: Reader<T, I> {
        self.inner.rewind();
    }
}