use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::{TryFromError, ReadError, ReadFrom, WriteTo};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Ping,
    Pong,
    AddEntry,
    EntryAdded,
    RemoveEntry,
    EntryRemoved,
    #[doc(hidden)]
    __NonExhaustive,
}

impl MessageType {
    pub fn is_command(self) -> bool {
        match self {
            MessageType::Ping | MessageType::AddEntry | MessageType::RemoveEntry => true,
            _ => false
        }
    }
}

impl ReadFrom for MessageType {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let value = source.read_u8()?;

        Ok(Self::try_from(value)?)
    }
}

impl WriteTo for MessageType {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8((*self).into())
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
            _ => panic!("Invalid Message type"),
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
            _ => Err(TryFromError::InvalidValue)
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
}