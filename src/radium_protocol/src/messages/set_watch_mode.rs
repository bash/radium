use std::io;
use super::super::{ReadFrom, WriteTo, WatchMode, WatchModeReader, ReadResult, WriteResult, Reader, ReaderStatus};
use super::super::errors::ReadError;

#[derive(Debug)]
pub struct SetWatchMode {
    mode: WatchMode,
}

#[derive(Debug)]
enum ReaderState {
    Mode,
    Ended,
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

impl<R: io::Read> Reader<SetWatchMode, R> for SetWatchModeReader {
    fn resume(&mut self, input: &mut R) -> io::Result<ReaderStatus<SetWatchMode>> {
        let status = self.inner.resume(input)?;

        match status {
            ReaderStatus::Complete(mode) => Ok(ReaderStatus::Complete(SetWatchMode::new(mode))),
            ReaderStatus::Pending => Ok(ReaderStatus::Pending),
            ReaderStatus::Ended => Ok(ReaderStatus::Ended),
        }
    }

    fn rewind(&mut self) {
        panic!("TODO: Not implemented!");
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