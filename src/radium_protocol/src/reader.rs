use std::io;
use std::io::ErrorKind;

#[derive(Debug, Eq, PartialEq)]
pub enum ReaderStatus<T> {
    Pending,
    Complete(T),
}

pub trait HasReader: Sized {
    type Reader: Reader<Output=Self>;

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
pub struct ReaderController<R>
    where R: Reader
{
    inner: R,
}

/// tl;dr - A `SyncReaderController` is used with blocking I/O.
///
/// The `SyncReaderController` implements a similar interface to the [`ReaderController`]
/// but consumes a [`Reader`] in one take, ignoring `ErrorKind::WouldBlock` errors.
///
/// [`Reader`]: ./trait.Reader.html
/// [`ReaderController`]: ./struct.ReaderController.html
#[derive(Debug)]
pub struct SyncReaderController<R>
    where R: Reader
{
    inner: R,
}

/// A `Reader` is a resumable parser that eventually emits a value of type `T`.
/// Its `resume` method is called when the `io::Read` instance becomes readable.
///
/// Note that a Reader should not catch `io::Error`s of the kind `ErrorKind::WouldBlock`
/// this is the job of the [`ReaderController`].
///
/// [`ReaderController`]: ./struct.ReaderController.html
pub trait Reader {
    type Output;

    /// Resumes the reader with the given input
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<Self::Output>> where I: io::Read;

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
impl<R> ReaderController<R>
    where R: Reader
{
    pub fn new(inner: R) -> Self {
        ReaderController { inner }
    }

    /// Resumes with the given input
    pub fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<R::Output>>
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

impl<R> SyncReaderController<R>
    where R: Reader
{
    pub fn new(inner: R) -> Self {
        SyncReaderController { inner }
    }

    /// Resumes with the given input
    pub fn resume<I>(&mut self, input: &mut I) -> io::Result<R::Output>
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
