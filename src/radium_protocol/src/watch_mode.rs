use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use std::io;
use super::errors::InvalidValueError;
use super::{WriteTo, WriteResult, Reader, ReaderStatus};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The `WatchMode` indicates whether the client wants to be notified about
/// expired entries or not.
pub enum WatchMode {
    /// The client will not receive notifications
    None,
    /// The client will receive notifications for all tags
    All,
    /// The client will receive notifications only for one tag
    Tagged(u64)
}

#[derive(Debug)]
enum WatchModeReaderState {
    Mode,
    Tag
}

#[derive(Debug)]
pub struct WatchModeReader {
    state: WatchModeReaderState
}

impl WatchMode {
    pub fn matches_tag(&self, tag: u64) -> bool {
        match self {
            &WatchMode::None => false,
            &WatchMode::All => true,
            &WatchMode::Tagged(val) => val == tag,
        }
    }

    pub fn reader() -> WatchModeReader {
        WatchModeReader { state: WatchModeReaderState::Mode }
    }
}

impl Reader<WatchMode> for WatchModeReader {
    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<WatchMode>> where R: io::Read {
        let (state, status) = match self.state {
            WatchModeReaderState::Mode => {
                let mode = input.read_u8()?;

                match mode {
                    0 => (WatchModeReaderState::Mode, ReaderStatus::Complete(WatchMode::None)),
                    1 => (WatchModeReaderState::Mode, ReaderStatus::Complete(WatchMode::All)),
                    2 => (WatchModeReaderState::Tag, ReaderStatus::Pending),
                    _ => { return Err(InvalidValueError::new()) }
                }
            },
            WatchModeReaderState::Tag => {
                let tag = input.read_u64::<NetworkEndian>()?;

                (WatchModeReaderState::Mode, ReaderStatus::Complete(WatchMode::Tagged(tag)))
            },
        };

        self.state = state;

        Ok(status)
    }
}

impl WriteTo for WatchMode {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        let mode = match self {
            &WatchMode::None => 0,
            &WatchMode::All => 1,
            &WatchMode::Tagged(..) => 2,
        };

        target.write_u8(mode)?;

        if let &WatchMode::Tagged(tag) = self {
            target.write_u64::<NetworkEndian>(tag)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_watch_mode {
        ($test:ident, $mode:expr, $raw:expr) => {
            #[test]
            fn $test() {
                let mut buf = vec![];
                assert!($mode.write_to(&mut buf).is_ok());
                assert_eq!($raw, &mut buf.as_ref());
            }
        }
    }

    test_watch_mode!(test_none, WatchMode::None, &mut [0]);
    test_watch_mode!(test_all, WatchMode::All, &mut [1]);
    test_watch_mode!(test_tagged, WatchMode::Tagged(42), &mut [2, 0, 0, 0, 0, 0, 0, 0, 42]);
}