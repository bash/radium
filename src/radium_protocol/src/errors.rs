use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum TryFromError {
    InvalidValue,
}

#[derive(Debug)]
pub enum ReadError {
    InvalidValue,
    UnexpectedEof,
    LimitReached,
    IoError(io::Error),
}

#[derive(Debug)]
pub enum WriteError {
    IoError(io::Error),
    DataLengthOverflow
}

impl_err_display!(TryFromError);

impl Error for TryFromError {
    fn description(&self) -> &str {
        "Invalid value"
    }
}

impl From<TryFromError> for ReadError {
    fn from(_: TryFromError) -> Self {
        ReadError::InvalidValue
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> Self {
        ReadError::IoError(err)
    }
}

impl_err_display!(ReadError);

impl Error for ReadError {
    fn description(&self) -> &str {
        match self {
            &ReadError::InvalidValue => "invalid value",
            &ReadError::UnexpectedEof => "unexpected eof",
            &ReadError::LimitReached => "limit reached",
            &ReadError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ReadError::IoError(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl_err_display!(WriteError);

impl Error for WriteError {
    fn description(&self) -> &str {
        match self {
            &WriteError::IoError(ref err) => err.description(),
            &WriteError::DataLengthOverflow => "Data overflows maximum length",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &WriteError::IoError(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for WriteError {
    fn from(err: io::Error) -> Self {
        WriteError::IoError(err)
    }
}