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
pub enum EntryWriteError {
    DataLengthOverflow,
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

impl_err_display!(EntryWriteError);

impl Error for EntryWriteError {
    fn description(&self) -> &str {
        match self {
            &EntryWriteError::DataLengthOverflow => "Data overflows maximum length",
        }
    }
}
