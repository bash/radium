use std::io;
use super::super::{ReadFrom, WriteTo, WatchMode, ReadResult, WriteResult};

#[derive(Debug)]
pub struct SetWatchMode {
    mode: WatchMode,
}

impl SetWatchMode {
    pub fn new(mode: WatchMode) -> Self {
        SetWatchMode { mode }
    }

    pub fn mode(&self) -> WatchMode {
        self.mode
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