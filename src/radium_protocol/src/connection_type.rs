use std::io;
use std::convert::TryFrom;
use byteorder::{WriteBytesExt, ReadBytesExt};
use super::{ReadError, TryFromError};

#[derive(Debug, Eq, PartialEq)]
pub enum ConnectionType {
    Command,
    Listen,
    // Prevent exhaustive matching to allow for future extension
    #[doc(hidden)]
    __NonExhaustive,
}

impl ConnectionType {
    pub fn command_type(&self) -> u8 {
        match self {
            &ConnectionType::Command => 0,
            &ConnectionType::Listen => 1,
            _ => panic!("invalid connection type"),
        }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.command_type())
    }

    pub fn read_from<R: io::Read>(source: &mut R)
                                  -> Result<ConnectionType, ReadError> {
        let value = source.read_u8()?;

        Ok(ConnectionType::try_from(value)?)
    }
}

impl TryFrom<u8> for ConnectionType {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConnectionType::Command),
            1 => Ok(ConnectionType::Listen),
            _ => Err(TryFromError::InvalidValue),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_write() {
        let mut result = vec![];

        assert!(ConnectionType::Command.write_to(&mut result).is_ok());
        assert_eq!(vec![0], result);
    }

    #[test]
    fn test_listen_write() {
        let mut result = vec![];

        assert!(ConnectionType::Listen.write_to(&mut result).is_ok());
        assert_eq!(vec![1], result);
    }

    #[test]
    fn test_read() {
        assert_eq!(ConnectionType::Command,
                   ConnectionType::read_from(&mut [0u8].as_ref()).unwrap());

        assert_eq!(ConnectionType::Listen,
                   ConnectionType::read_from(&mut [1u8].as_ref()).unwrap());

        assert!(ConnectionType::read_from(&mut [2u8].as_ref()).is_err());
    }
}