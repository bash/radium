use super::io::{ReadFrom, WriteTo, Error};
use std::convert::{TryFrom, From};
use std::io::{Read, Write, Result as IoResult};
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone, Debug)]
pub enum ConnectionMode {
    Action,
    Listen
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ConnectionModeTryFromErr(());

impl From<ConnectionModeTryFromErr> for Error {
    fn from(_: ConnectionModeTryFromErr) -> Self {
        Error {}
    }
}

impl TryFrom<u8> for ConnectionMode {
    type Err = ConnectionModeTryFromErr;

    fn try_from(value: u8) -> Result<Self, Self::Err> {
        match value {
            0 => Ok(ConnectionMode::Action),
            1 => Ok(ConnectionMode::Listen),
            _ => Err(ConnectionModeTryFromErr(())),
        }
    }
}

impl Into<u8> for ConnectionMode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl ReadFrom for ConnectionMode {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error> {
        let raw = read.read_u8()?;

        Ok(ConnectionMode::try_from(raw)?)
    }
}

impl WriteTo for ConnectionMode {
    fn write_to<W: Write + Sized>(&self, write: &mut W) -> IoResult<()> where Self: Sized, W: Sized {
        write.write_u8((*self).into())
    }
}