use std::io;
use super::super::{ReadFrom, WriteTo, WatchMode, WatchModeReader, ReadResult, WriteResult, Reader, ReaderStatus, MessageInner, Message};

#[derive(Debug, Eq, PartialEq)]
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

    pub fn reader() -> SetWatchModeReader {
        SetWatchModeReader { inner: WatchMode::reader() }
    }
}

impl MessageInner for SetWatchMode {
    #[inline]
    fn wrap(self) -> Message {
        Message::SetWatchMode(self)
    }
}

impl Reader<SetWatchMode> for SetWatchModeReader {
    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<SetWatchMode>> where R: io::Read {
        let status = self.inner.resume(input)?;

        match status {
            ReaderStatus::Complete(mode) => Ok(ReaderStatus::Complete(SetWatchMode::new(mode))),
            ReaderStatus::Pending => Ok(ReaderStatus::Pending),
            ReaderStatus::Ended => Ok(ReaderStatus::Ended),
        }
    }

    fn rewind(&mut self) {
        self.inner.rewind()
    }
}


impl ReadFrom for SetWatchMode {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let mode = WatchMode::read_from(source)?;

        Ok(SetWatchMode::new(mode))
    }
}

impl WriteTo for SetWatchMode {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        self.mode.write_to(target)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::{MessageType, WatchMode};

    #[test]
    fn test_reader_with_tagged () {
        let mut input = vec![
            /* type          */ MessageType::SetWatchMode.into(),
            /* mode = tagged */ 2,
            /* tag           */ 0, 0, 0, 0, 0, 0, 255, 255,
        ];

        test_reader! {
            Message::reader(),
            input,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Complete(Message::SetWatchMode(SetWatchMode::new(WatchMode::Tagged(65535))))
        };
    }

    #[test]
    fn test_reader () {
        let mut input = vec![
            /* type        */ MessageType::SetWatchMode.into(),
            /* mode = all  */ 1,
        ];

        test_reader! {
            Message::reader(),
            input,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Complete(Message::SetWatchMode(SetWatchMode::new(WatchMode::All)))
        };
    }
}