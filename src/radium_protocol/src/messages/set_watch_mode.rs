use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::super::{ReadFrom, WriteTo, ReadError, WatchMode};

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
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let value = source.read_u8()?;
        let mode = WatchMode::try_from(value)?;

        Ok(SetWatchMode::new(mode))
    }
}

impl WriteTo for SetWatchMode {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        let value = self.mode.into();

        target.write_u8(value)?;

        Ok(())
    }
}