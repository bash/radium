use byteorder::{ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::io;
use super::super::errors::TryFromError;
use super::super::{ReadFrom, WriteTo, ReadError};

#[derive(Debug)]
pub struct ErrorMessage {
    code: ErrorCode
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorCode {
    /// The client was rejected because
    /// the worker was unable to handle more clients
    ClientRejected,
    /// The action that was sent is not implemented
    ActionNotImplemented,
    /// The message that was sent is not an action
    InvalidAction,
    /// This message is sent when the connection is somehow broken
    /// e.g. reads and/or writes fail
    ConnectionFailure,
}

impl Into<u8> for ErrorCode {
    fn into(self) -> u8 {
        match self {
            ErrorCode::ClientRejected => 0,
            ErrorCode::ActionNotImplemented => 1,
            ErrorCode::InvalidAction => 2,
            ErrorCode::ConnectionFailure => 3,
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
            3 => Ok(ErrorCode::ConnectionFailure),
            _ => Err(TryFromError::InvalidValue),
        }
    }
}

impl ErrorMessage {
    pub fn new(code: ErrorCode) -> Self {
        ErrorMessage { code }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

impl ReadFrom for ErrorMessage {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let raw_code = source.read_u8()?;
        let code = ErrorCode::try_from(raw_code)?;

        Ok(ErrorMessage::new(code))
    }
}

impl WriteTo for ErrorMessage {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.code.into())
    }
}