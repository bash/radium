use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::{TryFromError, ReadError, ReadFrom, WriteTo};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Ping,
    AddEntry,
    RemoveEntry,
    #[doc(hidden)]
    __NonExhaustive,
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
            MessageType::AddEntry => 1,
            MessageType::RemoveEntry => 2,
            _ => panic!("Invalid Message type"),
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Ping),
            1 => Ok(MessageType::AddEntry),
            2 => Ok(MessageType::RemoveEntry),
            _ => Err(TryFromError::InvalidValue)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_message_type {
        ($msg:expr, $value:expr) => {
            let mut buf = vec![];
            assert!($msg.write_to(&mut buf).is_ok());
            assert_eq!(vec![$value], buf);

            assert_eq!($value as u8, $msg.into());
            assert_eq!($msg, MessageType::try_from($value as u8).unwrap());

            assert_eq!($msg, MessageType::read_from(&mut [$value as u8].as_ref()).unwrap());
        }
    }

    #[test]
    fn test_ping() {
        test_message_type!(MessageType::Ping, 0);
    }

    #[test]
    fn test_add_entry() {
        test_message_type!(MessageType::AddEntry, 1);
    }

    #[test]
    fn test_remove_entry() {
        test_message_type!(MessageType::RemoveEntry, 2);
    }
}