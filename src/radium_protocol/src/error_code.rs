use byteorder::{WriteBytesExt, ReadBytesExt};
use std::convert::TryFrom;
use std::io;
use super::errors::TryFromError;
use super::{ReadResult, WriteResult, ReadFrom, WriteTo};
use super::reader::{Reader, ReaderStatus, HasReader};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    /// The client was rejected because
    /// the worker was unable to handle more clients
    ClientRejected,
    /// The action that was sent is not implemented
    ActionNotImplemented,
    /// The message that was sent is not an action
    InvalidAction,
    /// An error occurred when processing the action
    ActionProcessingError,
    /// This message is sent when the connection is somehow broken
    /// e.g. reads and/or writes fail
    ConnectionFailure,
}

pub struct ErrorCodeReader;

impl HasReader for ErrorCode {
    type Reader = ErrorCodeReader;

    fn reader() -> ErrorCodeReader {
        ErrorCodeReader {}
    }
}

impl Into<u8> for ErrorCode {
    fn into(self) -> u8 {
        match self {
            ErrorCode::ClientRejected => 0,
            ErrorCode::ActionNotImplemented => 1,
            ErrorCode::InvalidAction => 2,
            ErrorCode::ActionProcessingError => 3,
            ErrorCode::ConnectionFailure => 4,
        }
    }
}

impl TryFrom<u8> for ErrorCode {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ErrorCode::ClientRejected),
            1 => Ok(ErrorCode::ActionNotImplemented),
            2 => Ok(ErrorCode::InvalidAction),
            3 => Ok(ErrorCode::ActionProcessingError),
            4 => Ok(ErrorCode::ConnectionFailure),
            _ => Err(TryFromError::InvalidValue),
        }
    }
}

impl ReadFrom for ErrorCode {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let value = source.read_u8()?;

        Ok(ErrorCode::try_from(value)?)
    }
}

impl WriteTo for ErrorCode {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        target.write_u8((*self).into())?;
        Ok(())
    }
}

impl Reader<ErrorCode> for ErrorCodeReader {
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<ErrorCode>> where I: io::Read {
        let value = input.read_u8()?;
        let code = ErrorCode::try_from(value)?;

        Ok(ReaderStatus::Complete(code))
    }

    fn rewind(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_reader() {
        let vec = vec![0];
        let result = test_reader2!(ErrorCode::reader(), vec);

        assert!(result.is_ok());
        assert_eq!(ErrorCode::ClientRejected, result.unwrap());
    }
}
