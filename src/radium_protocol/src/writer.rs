use std::io;
use std::io::ErrorKind;
use std::collections::VecDeque;
use std::error;

#[derive(Debug, Eq, PartialEq)]
pub enum WriterStatus {
    Pending,
    Complete,
}

pub trait HasWriter: Sized {
    type Writer: Writer;

    fn writer(self) -> Self::Writer;
}

/// tl;dr - A `WriterController` is used with non-blocking I/O.
///
/// A `WriterController` wraps a [`Writer`] and offers some additional functionality.
///
/// The `resume` method of the inner [`Writer`] is called until either it becomes no longer pending
/// or an `io::Error` of kind `ErrorKind::WouldBlock` is returned.
///
/// [`Writer`]: ./trait.Writer.html
/// [`WriterStatus::Pending`]: ./enum.WriterStatus.html
#[derive(Debug)]
pub struct WriterController<W> where W: Writer {
    inner: W,
}

/// tl;dr - A `SyncWriterController` is used with blocking I/O.
///
/// The `SyncWriterController` implements a similar interface to the [`WriterController`]
/// but consumes a [`Writer`] in one take, ignoring `ErrorKind::WouldBlock` errors.
///
/// [`Writer`]: ./trait.Writer.html
/// [`WriterController`]: ./struct.WriterController.html
#[derive(Debug)]
pub struct SyncWriterController<W> where W: Writer {
    inner: W,
}

pub struct WriteQueue<T> where T: HasWriter {
    queue: VecDeque<T>,
    writer: Option<WriterController<T::Writer>>,
    limit: Option<usize>,
}

#[derive(Debug)]
pub enum WriteQueueError {
    LimitExceeded,
}

impl_err_display!(WriteQueueError);

impl error::Error for WriteQueueError {
    fn description(&self) -> &str {
        "queue limit exceeded"
    }
}

/// A `Writer` is a resumable parser that eventually emits a value of type `T`.
/// Its `resume` method is called when the `io::Read` instance becomes readable.
///
/// Note that a Writer should not catch `io::Error`s of the kind `ErrorKind::WouldBlock`
/// this is the job of the [`WriterController`].
///
/// [`WriterController`]: ./struct.WriterController.html
pub trait Writer {
    /// Resumes the Writer with the given output
    fn resume<O>(&mut self, output: &mut O) -> io::Result<WriterStatus> where O: io::Write;

    /// Rewinds the Writer to its initial state
    fn rewind(&mut self);
}

impl WriterStatus {
    pub fn map<F>(self, map_fn: F) -> WriterStatus where F: FnOnce() -> () {
        match self {
            WriterStatus::Pending => WriterStatus::Pending,
            WriterStatus::Complete => {
                map_fn();
                WriterStatus::Complete
            }
        }
    }
}

/// The [`WriterController`] intentionally does not implement [`Writer<T>`]
/// as it behaves slightly different. (see above)
///
/// [`Writer<T>`]: ./trait.Writer.html
/// [`WriterController`]: ./struct.WriterController.html
impl<W> WriterController<W> where W: Writer {
    pub fn new(inner: W) -> Self {
        WriterController { inner }
    }

    /// Resumes with the given output
    pub fn resume<O>(&mut self, output: &mut O) -> io::Result<WriterStatus> where O: io::Write {
        match self.inner.resume(output) {
            Ok(status) => match status {
                WriterStatus::Complete => Ok(WriterStatus::Complete),
                WriterStatus::Pending => self.resume(output),
            },
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => Ok(WriterStatus::Pending),
                _ => {
                    self.inner.rewind();
                    Err(err)
                }
            },
        }
    }
}

impl<W> SyncWriterController<W> where W: Writer {
    pub fn new(inner: W) -> Self {
        SyncWriterController { inner }
    }

    /// Resumes with the given output
    pub fn resume<O>(&mut self, output: &mut O) -> io::Result<()> where O: io::Write {
        loop {
            match self.inner.resume(output) {
                Ok(status) => match status {
                    WriterStatus::Complete => { return Ok(()) }
                    WriterStatus::Pending => {}
                },
                Err(err) => {
                    self.inner.rewind();
                    return Err(err);
                }
            };
        }
    }
}

impl<T> WriteQueue<T> where T: HasWriter {
    pub fn new() -> Self {
        WriteQueue {
            limit: None,
            queue: VecDeque::new(),
            writer: None,
        }
    }

    pub fn with_limit(limit: usize) -> Self {
        WriteQueue {
            limit: Some(limit),
            queue: VecDeque::with_capacity(limit),
            writer: None,
        }
    }

    pub fn queue(&mut self, value: T) -> Result<(), WriteQueueError> {
        if Some(self.queue.len()) == self.limit {
            return Err(WriteQueueError::LimitExceeded);
        }

        self.queue.push_back(value);

        Ok(())
    }

    fn next_writer(&mut self) {
        if let Some(value) = self.queue.pop_front() {
            self.writer = Some(WriterController::new(value.writer()));
        }
    }

    pub fn resume<O>(&mut self, output: &mut O) -> io::Result<()> where O: io::Write {
        if let None = self.writer {
            self.next_writer();
        }

        loop {
            match self.writer {
                Some(ref mut writer) => match writer.resume(output)? {
                    WriterStatus::Pending => { break }
                    WriterStatus::Complete => {}
                },
                None => { break }
            }

            self.next_writer();
        }

        Ok(())
    }
}