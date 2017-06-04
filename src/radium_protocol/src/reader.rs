use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq)]
pub enum ReaderStatus<T> {
    Pending,
    Complete(T),
}

pub trait HasReader: Sized {
    type Reader: Reader<Self>;

    fn reader() -> Self::Reader;
}

/// tl;dr - A `ReaderController` is used with non-blocking I/O.
///
/// A `ReaderController` wraps a [`Reader`] and offers some additional functionality.
///
/// The `resume` method of the inner [`Reader`] is called until either it becomes no longer pending
/// or an `io::Error` of kind `ErrorKind::WouldBlock` is returned.
///
/// [`Reader`]: ./trait.Reader.html
/// [`ReaderStatus::Pending`]: ./enum.ReaderStatus.html
#[derive(Debug)]
pub struct ReaderController<T, R>
    where R: Reader<T>
{
    inner: R,
    _marker: PhantomData<T>,
}

/// tl;dr - A `SyncReaderController` is used with blocking I/O.
///
/// The `SyncReaderController` implements a similar interface to the [`ReaderController`]
/// but consumes a [`Reader`] in one take, ignoring `ErrorKind::WouldBlock` errors.
///
/// [`Reader`]: ./trait.Reader.html
/// [`ReaderController`]: ./struct.ReaderController.html
#[derive(Debug)]
pub struct SyncReaderController<T, R>
    where R: Reader<T>
{
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

    /// Rewinds the reader to its initial state
    fn rewind(&mut self);
}

impl<T> ReaderStatus<T> {
    pub fn map<F, R>(self, map_fn: F) -> ReaderStatus<R>
        where F: FnOnce(T) -> R
    {
        match self {
            ReaderStatus::Pending => ReaderStatus::Pending,
            ReaderStatus::Complete(value) => ReaderStatus::Complete(map_fn(value)),
        }
    }
}

/// The [`ReaderController`] intentionally does not implement [`Reader<T>`]
/// as it behaves slightly different. (see above)
///
/// [`Reader<T>`]: ./trait.Reader.html
/// [`ReaderController`]: ./struct.ReaderController.html
impl<T, R> ReaderController<T, R>
    where R: Reader<T>
{
    pub fn new(inner: R) -> Self {
        ReaderController {
            inner,
            _marker: PhantomData {},
        }
    }

    /// Resumes with the given input
    pub fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<T>>
        where I: io::Read
    {
        match self.inner.resume(input) {
            Ok(status) => {
                match status {
                    ReaderStatus::Complete(val) => Ok(ReaderStatus::Complete(val)),
                    ReaderStatus::Pending => self.resume(input),
                }
            }
            Err(err) => {
                match err.kind() {
                    ErrorKind::WouldBlock => Ok(ReaderStatus::Pending),
                    _ => {
                        self.inner.rewind();
                        Err(err)
                    }
                }
            }
        }
    }
}

impl<T, R> SyncReaderController<T, R>
    where R: Reader<T>
{
    pub fn new(inner: R) -> Self {
        SyncReaderController {
            inner,
            _marker: PhantomData {},
        }
    }

    /// Resumes with the given input
    pub fn resume<I>(&mut self, input: &mut I) -> io::Result<T>
        where I: io::Read
    {
        loop {
            match self.inner.resume(input) {
                Ok(status) => {
                    match status {
                        ReaderStatus::Complete(value) => return Ok(value),
                        ReaderStatus::Pending => {}
                    }
                }
                Err(err) => {
                    self.inner.rewind();
                    return Err(err);
                }
            };
        }
    }
}
