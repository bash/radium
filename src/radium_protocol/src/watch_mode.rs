use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use std::io;
use super::errors::ReadError;
use super::{ReadFrom, WriteTo, ReadResult, WriteResult};

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

impl ReadFrom for WatchMode {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let mode = source.read_u8()?;

        match mode {
            0 => { Ok(WatchMode::None) }
            1 => { Ok(WatchMode::All) }
            2 => {
                let tag = source.read_u64::<NetworkEndian>()?;

                Ok(WatchMode::Tagged(tag))
            },
            _ => { Err(ReadError::InvalidValue) }
        }
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
                assert_eq!($mode, WatchMode::read_from(&mut $raw.as_ref()).unwrap());
            }
        }
    }

    test_watch_mode!(test_none, WatchMode::None, &mut [0]);
    test_watch_mode!(test_all, WatchMode::All, &mut [1]);
    test_watch_mode!(test_tagged, WatchMode::Tagged(42), &mut [2, 0, 0, 0, 0, 0, 0, 0, 42]);
}