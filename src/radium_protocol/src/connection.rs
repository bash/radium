use std::io;
use std::fmt;
use std::error::Error;
use std::convert::TryFrom;
use byteorder::{WriteBytesExt, ReadBytesExt};

macro_rules! impl_err_display {
    ($ty:ty) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.description())
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConnectionType {
    Command,
    Listen,
    // Prevent exhaustive matching to allow for future extension
    #[doc(hidden)]
    __NonExhaustive,
}

#[derive(Debug)]
pub enum ConnectionTypeTryFromError {
    InvalidValue(u8),
}

#[derive(Debug)]
pub enum ConnectionTypeReadError {
    InvalidValue(u8),
    ReadError(io::Error),
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
                                  -> Result<ConnectionType, ConnectionTypeReadError> {
        let value = source.read_u8()?;

        Ok(ConnectionType::try_from(value)?)
    }
}

impl TryFrom<u8> for ConnectionType {
    type Error = ConnectionTypeTryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConnectionType::Command),
            1 => Ok(ConnectionType::Listen),
            _ => Err(ConnectionTypeTryFromError::InvalidValue(value)),
        }
    }
}

impl Error for ConnectionTypeTryFromError {
    fn description(&self) -> &str {
        "invalid connection type value"
    }
}

impl_err_display!(ConnectionTypeTryFromError);

impl From<ConnectionTypeTryFromError> for ConnectionTypeReadError {
    fn from(err: ConnectionTypeTryFromError) -> Self {
        match err {
            ConnectionTypeTryFromError::InvalidValue(value) => {
                ConnectionTypeReadError::InvalidValue(value)
            }
        }
    }
}

impl From<io::Error> for ConnectionTypeReadError {
    fn from(err: io::Error) -> Self {
        ConnectionTypeReadError::ReadError(err)
    }
}

impl_err_display!(ConnectionTypeReadError);

impl Error for ConnectionTypeReadError {
    fn description(&self) -> &str {
        match self {
            &ConnectionTypeReadError::InvalidValue(..) => "invalid connection type value",
            &ConnectionTypeReadError::ReadError(ref err) => err.description(),
            _ => panic!("invalid error"),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ConnectionTypeReadError::ReadError(ref err) => err.cause(),
            _ => None,
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