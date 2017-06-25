use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use std::io;
use super::errors::InvalidValueError;
use super::reader::{Reader, ReaderStatus, HasReader};
use super::writer::{Writer, WriterStatus, HasWriter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The `WatchMode` indicates whether the client wants to be notified about
/// expired entries or not.
pub enum WatchMode {
    /// The client will not receive notifications
    None,
    /// The client will receive notifications for all tags
    All,
    /// The client will receive notifications only for one tag
    Tagged(u64),
}

#[derive(Debug)]
enum ReaderState {
    Mode,
    Tag,
}

#[derive(Debug)]
enum WriterState {
    Mode,
    Tag,
}

#[derive(Debug)]
pub struct WatchModeReader {
    state: ReaderState,
}

#[derive(Debug)]
pub struct WatchModeWriter {
    state: WriterState,
    value: WatchMode,
}

impl WatchMode {
    pub fn matches_tag(&self, tag: u64) -> bool {
        match self {
            &WatchMode::None => false,
            &WatchMode::All => true,
            &WatchMode::Tagged(val) => val == tag,
        }
    }
}

impl HasReader for WatchMode {
    type Reader = WatchModeReader;

    fn reader() -> Self::Reader {
        WatchModeReader { state: ReaderState::Mode }
    }
}

impl HasWriter for WatchMode {
    type Writer = WatchModeWriter;

    fn writer(self) -> Self::Writer {
        WatchModeWriter::new(self)
    }
}

impl ReaderState {
    pub fn initial() -> Self {
        ReaderState::Mode
    }
}

impl WriterState {
    pub fn initial() -> Self {
        WriterState::Mode
    }
}

impl Reader for WatchModeReader {
    type Output = WatchMode;

    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<Self::Output>>
        where R: io::Read
    {
        let (state, status) = match self.state {
            ReaderState::Mode => {
                let mode = input.read_u8()?;

                match mode {
                    0 => (ReaderState::initial(), ReaderStatus::Complete(WatchMode::None)),
                    1 => (ReaderState::initial(), ReaderStatus::Complete(WatchMode::All)),
                    2 => (ReaderState::Tag, ReaderStatus::Pending),
                    _ => return Err(InvalidValueError::new()),
                }
            }
            ReaderState::Tag => {
                let tag = input.read_u64::<NetworkEndian>()?;

                (ReaderState::initial(), ReaderStatus::Complete(WatchMode::Tagged(tag)))
            }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = ReaderState::Mode;
    }
}

impl WatchModeWriter {
    pub fn new(value: WatchMode) -> WatchModeWriter {
        WatchModeWriter { state: WriterState::initial(), value }
    }
}

impl Writer for WatchModeWriter {
    fn resume<O>(&mut self, output: &mut O) -> io::Result<WriterStatus>
        where O: io::Write
    {
        let (state, status) = match self.state {
            WriterState::Mode => {
                let (next, status, mode) = match self.value {
                    WatchMode::None => (WriterState::initial(), WriterStatus::Complete, 0),
                    WatchMode::All => (WriterState::initial(), WriterStatus::Complete, 1),
                    WatchMode::Tagged(..) => (WriterState::Tag, WriterStatus::Pending, 2),
                };

                output.write_u8(mode)?;

                (next, status)
            }
            WriterState::Tag => {
                if let WatchMode::Tagged(tag) = self.value {
                    output.write_u64::<NetworkEndian>(tag)?;
                }

                (WriterState::initial(), WriterStatus::Complete)
            }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = WriterState::initial();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_watch_mode {
        ($mode:expr, $raw:expr) => {
            {
                let result = test_reader!(WatchMode::reader(), $raw);

                assert!(result.is_ok());
                assert_eq!($mode, result.unwrap());
            }

            {
                let (buf, result) = test_writer!($mode.writer());

                assert!(result.is_ok());
                assert_eq!($raw, buf);
            }
        }
    }

    #[test]
    fn test_none() {
        test_watch_mode!(WatchMode::None, vec![0]);
    }

    #[test]
    fn test_all() {
        test_watch_mode!(WatchMode::All, vec![1]);
    }

    #[test]
    fn test_tagged() {
        test_watch_mode!(WatchMode::Tagged(42), vec![2, 0, 0, 0, 0, 0, 0, 0, 42]);
    }
}
