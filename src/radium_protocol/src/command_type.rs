use std::io;
use std::convert::TryFrom;
use byteorder::{ReadBytesExt, WriteBytesExt};
use super::{TryFromError, ReadError, ReadFrom, WriteTo};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandType {
    Ping,
    AddEntry,
    RemoveEntry,
    #[doc(hidden)]
    __NonExhaustive,
}

impl ReadFrom for CommandType {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let value = source.read_u8()?;

        Ok(Self::try_from(value)?)
    }
}

impl WriteTo for CommandType {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8((*self).into())
    }
}

impl Into<u8> for CommandType {
    fn into(self) -> u8 {
        match self {
            CommandType::Ping => 0,
            CommandType::AddEntry => 1,
            CommandType::RemoveEntry => 2,
            _ => panic!("Invalid command type"),
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Ping),
            1 => Ok(CommandType::AddEntry),
            2 => Ok(CommandType::RemoveEntry),
            _ => Err(TryFromError::InvalidValue)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write() {
        {
            let mut buf = vec![];
            assert!(CommandType::Ping.write_to(&mut buf).is_ok());
            assert_eq!(vec![0], buf);
        }
        {
            let mut buf = vec![];
            assert!(CommandType::AddEntry.write_to(&mut buf).is_ok());
            assert_eq!(vec![1], buf);
        }
        {
            let mut buf = vec![];
            assert!(CommandType::RemoveEntry.write_to(&mut buf).is_ok());
            assert_eq!(vec![2], buf);
        }
    }

    #[test]
    fn test_into() {
        assert_eq!(0u8, CommandType::Ping.into());
        assert_eq!(1u8, CommandType::AddEntry.into());
        assert_eq!(2u8, CommandType::RemoveEntry.into());
    }

    #[test]
    fn test_from() {
        assert_eq!(CommandType::Ping, CommandType::try_from(0u8).unwrap());
        assert_eq!(CommandType::AddEntry, CommandType::try_from(1u8).unwrap());
        assert_eq!(CommandType::RemoveEntry, CommandType::try_from(2u8).unwrap());
    }

    #[test]
    fn test_read() {
        assert_eq!(CommandType::Ping,
                   CommandType::read_from(&mut [0u8].as_ref()).unwrap());

        assert_eq!(CommandType::AddEntry,
                   CommandType::read_from(&mut [1u8].as_ref()).unwrap());

        assert_eq!(CommandType::RemoveEntry,
                   CommandType::read_from(&mut [2u8].as_ref()).unwrap());
    }
}