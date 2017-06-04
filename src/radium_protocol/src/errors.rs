use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct InvalidValueError;

#[derive(Debug)]
pub struct DataLengthError;

#[derive(Debug)]
pub enum TryFromError {
    InvalidValue,
}

#[derive(Debug)]
#[deprecated(note = "Use custom `io::Error`s instead")]
pub enum WriteError {
    IoError(io::Error),
    DataLengthOverflow,
}

impl_err_display!(InvalidValueError);

impl InvalidValueError {
    pub fn new() -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, InvalidValueError {})
    }
}

impl Error for InvalidValueError {
    fn description(&self) -> &str {
        "Invalid value"
    }
}

impl_err_display!(DataLengthError);

impl DataLengthError {
    pub fn new() -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, DataLengthError {})
    }
}

impl Error for DataLengthError {
    fn description(&self) -> &str {
        "Data length does not match or overflows maximum"
    }
}


impl_err_display!(TryFromError);

impl Error for TryFromError {
    fn description(&self) -> &str {
        "Invalid value"
    }
}

impl From<TryFromError> for io::Error {
    fn from(err: TryFromError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, err)
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

impl From<WriteError> for io::Error {
    fn from(err: WriteError) -> Self {
        match err {
            WriteError::IoError(err) => err,
            WriteError::DataLengthOverflow => {
                io::Error::new(io::ErrorKind::Other, WriteError::DataLengthOverflow)
            }
        }
    }
}
