use std::io;
use super::super::{WatchMode, WatchModeReader, MessageInner, Message};
use super::super::reader::{Reader, ReaderStatus, HasReader};
use super::super::writer::HasWriter;
use super::super::WatchModeWriter;

pub type SetWatchModeWriter = WatchModeWriter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SetWatchMode {
    mode: WatchMode,
}

#[derive(Debug)]
pub struct SetWatchModeReader {
    inner: WatchModeReader,
}

impl SetWatchMode {
    pub fn new(mode: WatchMode) -> Self {
        SetWatchMode { mode }
    }

    pub fn mode(&self) -> WatchMode {
        self.mode
    }
}

impl HasWriter for SetWatchMode {
    type Writer = WatchModeWriter;

    fn writer(self) -> Self::Writer {
        WatchModeWriter::new(self.mode)
    }
}

impl MessageInner for SetWatchMode {
    fn wrap(self) -> Message {
        Message::SetWatchMode(self)
    }
}

impl HasReader for SetWatchMode {
    type Reader = SetWatchModeReader;

    fn reader() -> Self::Reader {
        SetWatchModeReader { inner: WatchMode::reader() }
    }
}

impl Reader for SetWatchModeReader {
    type Output = SetWatchMode;

    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<Self::Output>>
        where R: io::Read
    {
        let status = self.inner.resume(input)?;

        Ok(status.map(|mode| SetWatchMode::new(mode)))
    }

    fn rewind(&mut self) {
        self.inner.rewind();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::WatchMode;

    #[test]
    fn test_reader_with_tagged() {
        let input = vec![
            /* mode = tagged */ 2,
            /* tag           */ 0, 0, 0, 0, 0, 0, 255, 255
        ];

        let result = test_reader!(SetWatchMode::reader(), input);

        assert!(result.is_ok());
        assert_eq!(SetWatchMode::new(WatchMode::Tagged(65535)), result.unwrap());
    }

    #[test]
    fn test_reader() {
        let input = vec![
            /* mode = all */ 1
        ];

        let result = test_reader!(SetWatchMode::reader(), input);

        assert!(result.is_ok());
        assert_eq!(SetWatchMode::new(WatchMode::All), result.unwrap());
    }
}
