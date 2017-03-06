use std::convert::{TryFrom, Into, From};
use std::fmt;
use std::io::{Read, Error as IoError};
use byteorder::{NetworkEndian, ReadBytesExt};
use ::io::{Readable, Error};

macro_rules! gen_from {
    ($from:ty) => (
        impl From<$from> for Error {
            fn from(_: $from) -> Self {
                Error {}
            }
        }
    )
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MessageTypeTryFromErr(());

impl fmt::Display for MessageTypeTryFromErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid message type")
    }
}

#[derive(Debug)]
pub enum MessageTypeReadError {
    IoError(IoError),
    MessageTypeTryFromErr(MessageTypeTryFromErr)
}

gen_from!(IoError);
gen_from!(MessageTypeTryFromErr);

impl fmt::Display for MessageTypeReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MessageType {
    Ping
}

impl TryFrom<u16> for MessageType {
    type Err = MessageTypeTryFromErr;

    fn try_from(value: u16) -> Result<Self, Self::Err> {
        match value {
            0 => Ok(MessageType::Ping),
            _ => Err(MessageTypeTryFromErr(())),
        }
    }
}

impl Into<u16> for MessageType {
    fn into(self) -> u16 {
        self as u16
    }
}

impl Readable for MessageType {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error> {
        let raw_msg_type = read.read_u16::<NetworkEndian>()?;

        Ok(MessageType::try_from(raw_msg_type)?)
    }
}