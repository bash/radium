use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::{WriteTo, WriteResult};
use super::errors::TryFromError;
use super::reader::{Reader, ReaderStatus, HasReader};

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

pub struct MessageTypeReader;

impl MessageType {
    /// Determines if the message is a command that is handled by the server
    pub fn is_command(self) -> bool {
        match self {
            MessageType::Ping |
            MessageType::AddEntry |
            MessageType::RemoveEntry |
            MessageType::SetWatchMode => true,
            _ => false,
        }
    }

    pub fn to_u8(self) -> u8 {
        self.into()
    }
}

impl HasReader for MessageType {
    type Reader = MessageTypeReader;

    fn reader() -> Self::Reader {
        MessageTypeReader {}
    }
}

impl Reader for MessageTypeReader {
    type Output = MessageType;

    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<MessageType>>
        where I: io::Read
    {
        let value = input.read_u8()?;
        let msg_type = MessageType::try_from(value)?;

        Ok(ReaderStatus::Complete(msg_type))
    }

    fn rewind(&mut self) {}
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
    use super::super::reader::ReaderStatus;

    macro_rules! test_message_type {
        ($msg:expr, $value:expr, $command:expr) => {{
            let mut buf = vec![];
            assert!($msg.write_to(&mut buf).is_ok());
            assert_eq!(vec![$value], buf);

            assert_eq!($value as u8, $msg.into());
            assert_eq!($msg, MessageType::try_from($value as u8).unwrap());
            assert_eq!($command, $msg.is_command());

            let mut reader = MessageType::reader();
            let input = &mut ::std::io::Cursor::new(vec![$value]);

            assert_eq!(ReaderStatus::Complete($msg), reader.resume(input).unwrap());
        }};
    }

    #[test]
    fn test_ping() {
        test_message_type!(MessageType::Ping, 0, true);
    }

    #[test]
    fn test_pong() {
        test_message_type!(MessageType::Pong, 1, false);
    }

    #[test]
    fn test_add_entry() {
        test_message_type!(MessageType::AddEntry, 2, true);
    }

    #[test]
    fn test_entry_added() {
        test_message_type!(MessageType::EntryAdded, 3, false);
    }

    #[test]
    fn test_remove_entry() {
        test_message_type!(MessageType::RemoveEntry, 4, true);
    }

    #[test]
    fn test_entry_removed() {
        test_message_type!(MessageType::EntryRemoved, 5, false);
    }

    #[test]
    fn test_entry_expired() {
        test_message_type!(MessageType::EntryExpired, 6, false);
    }

    #[test]
    fn test_set_watch_mode() {
        test_message_type!(MessageType::SetWatchMode, 7, true);
    }

    #[test]
    fn test_ok() {
        test_message_type!(MessageType::Ok, 8, false);
    }

    #[test]
    fn test_error() {
        test_message_type!(MessageType::Error, 9, false);
    }
}
