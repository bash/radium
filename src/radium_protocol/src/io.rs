use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;
use super::errors::{ReadError, WriteError};

#[deprecated()]
pub type WriteResult = Result<(), WriteError>;
#[deprecated()]
#[doc(hidden)]
pub type ReadResult<T> = Result<T, ReadError>;

#[deprecated(note = "Use Writer<T> instead")]
pub trait WriteTo: Sized {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult;
}

#[deprecated(note = "Use Reader<T> instead")]
#[doc(hidden)]
pub trait ReadFrom: Sized {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self>;
}

#[deprecated(note = "Use Reader<T> instead")]
#[doc(hidden)]
pub trait ReadValueExt: Sized + io::Read {
    fn read_value<R: ReadFrom>(&mut self) -> ReadResult<R> {
        R::read_from(self)
    }
}

#[deprecated(note = "Use Writer<T> instead")]
pub trait WriteValueExt: Sized + io::Write {
    fn write_value<R: WriteTo>(&mut self, value: &R) -> WriteResult {
        value.write_to(self)?;

        Ok(())
    }
}

#[deprecated(note = "Use Reader<T> instead")]
#[doc(hidden)]
impl<R> ReadValueExt for R where R: io::Read {}

#[deprecated(note = "Use Writer<T> instead")]
impl<W> WriteValueExt for W where W: io::Write {}

#[derive(Debug, Eq, PartialEq)]
pub enum ReaderStatus<T> {
    Pending,
    Complete(T),
    // TODO: should readers automatically rewind or do we need `Ended`?
    Ended,
}

/// A `ReaderController` wraps a [`Reader`] and offers some additional functionality.
///
/// The `resume` method of the inner [`Reader`] is called until either it becomes no longer pending
/// or an `io::Error` of kind `ErrorKind::WouldBlock` is returned.
///
/// [`Reader`]: ./trait.Reader.html
/// [`ReaderStatus::Pending`]: ./enum.ReaderStatus.html
#[derive(Debug)]
pub struct ReaderController<T, R> where R: Reader<T> {
    inner: R,
    _marker: PhantomData<T>,
}

/// A `Reader` is a resumable parser that eventually emits a value of type `T`.
/// Its `resume` method is called when the `io::Read` instance becomes readable.
///
/// Note that a Reader should not catch `io::Error`s of the kind `ErrorKind::WouldBlock`
/// this is the job of the [`ReaderController`].
///
/// [`ReaderController`]: ./struct.ReaderController.html
pub trait Reader<T> {
    /// Resumes the reader with the given input
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<T>> where I: io::Read;
    /// Rewinds the reader back to its first state
    fn rewind(&mut self);
}

/// The [`ReaderController`] intentionally does not implement [`Reader<T>`]
/// as it behaves slightly different. (see above)
///
/// [`Reader<T>`]: ./trait.Reader.html
/// [`ReaderController`]: ./struct.ReaderController.html
impl<T, R> ReaderController<T, R> where R: Reader<T>  {
    pub fn new(inner: R) -> Self {
        ReaderController { inner, _marker: PhantomData {} }
    }

    /// Resumes with the given input
    pub fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<T>> where I: io::Read {
        match self.inner.resume(input) {
            Ok(val) => match val {
                ReaderStatus::Complete(val) => Ok(ReaderStatus::Complete(val)),
                ReaderStatus::Pending => self.resume(input),
                ReaderStatus::Ended => Ok(ReaderStatus::Ended)
            },
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => Ok(ReaderStatus::Pending),
                _ => Err(err)
            },
        }
    }

    /// Rewinds the inner reader back to its first state
    pub fn rewind(&mut self) {
        self.inner.rewind();
    }
}