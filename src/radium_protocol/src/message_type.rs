use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::{ReadFrom, WriteTo, ReadResult, WriteResult};
use super::errors::TryFromError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    /// 0x00
    Ping,
    /// 0x01
    Pong,
    /// 0x02
    AddEntry,
    /// 0x03
    EntryAdded,
    /// 0x04
    RemoveEntry,
    /// 0x05
    #[doc(hidden)]
    EntryRemoved,
    /// 0x06
    EntryExpired,
    /// 0x07
    SetWatchMode,
    /// 0x08
    Ok,
    /// 0x09
    Error,
}

impl MessageType {
    /// Determines if the message is a command that is handled by the server
    pub fn is_command(self) -> bool {
        match self {
            MessageType::Ping |
            MessageType::AddEntry |
            MessageType::RemoveEntry |
            MessageType::SetWatchMode => true,
            _ => false
        }
    }

    pub fn to_u8(self) -> u8 {
        self.into()
    }
}

impl ReadFrom for MessageType {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let value = source.read_u8()?;

        Ok(Self::try_from(value)?)
    }
}

impl WriteTo for MessageType {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        target.write_u8((*self).into())?;
        Ok(())
    }
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        match self {
            MessageType::Ping => 0,
            MessageType::Pong => 1,
            MessageType::AddEntry => 2,
            MessageType::EntryAdded => 3,
            MessageType::RemoveEntry => 4,
            MessageType::EntryRemoved => 5,
            MessageType::EntryExpired => 6,
            MessageType::SetWatchMode => 7,
            MessageType::Ok => 8,
            MessageType::Error => 9,
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Ping),
            1 => Ok(MessageType::Pong),
            2 => Ok(MessageType::AddEntry),
            3 => Ok(MessageType::EntryAdded),
            4 => Ok(MessageType::RemoveEntry),
            5 => Ok(MessageType::EntryRemoved),
            6 => Ok(MessageType::EntryExpired),
            7 => Ok(MessageType::SetWatchMode),
            8 => Ok(MessageType::Ok),
            9 => Ok(MessageType::Error),
            _ => Err(TryFromError::InvalidValue),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_message_type {
        ($test:ident, $msg:expr, $value:expr, $command:expr) => {
            #[test]
            fn $test() {
                let mut buf = vec![];
                assert!($msg.write_to(&mut buf).is_ok());
                assert_eq!(vec![$value], buf);

                assert_eq!($value as u8, $msg.into());
                assert_eq!($msg, MessageType::try_from($value as u8).unwrap());
                assert_eq!($command, $msg.is_command());

                assert_eq!($msg, MessageType::read_from(&mut [$value as u8].as_ref()).unwrap());
            }
        }
    }

    test_message_type!(test_ping, MessageType::Ping, 0, true);
    test_message_type!(test_pong, MessageType::Pong, 1, false);
    test_message_type!(test_add_entry, MessageType::AddEntry, 2, true);
    test_message_type!(test_entry_added, MessageType::EntryAdded, 3, false);
    test_message_type!(test_remove_entry, MessageType::RemoveEntry, 4, true);
    test_message_type!(test_entry_removed, MessageType::EntryRemoved, 5, false);
    test_message_type!(test_entry_expired, MessageType::EntryExpired, 6, false);
    test_message_type!(test_set_watch_mode, MessageType::SetWatchMode, 7, true);
    test_message_type!(test_ok, MessageType::Ok, 8, false);
    test_message_type!(test_error, MessageType::Error, 9, false);
}