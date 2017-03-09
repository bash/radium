use std::convert::{TryFrom, Into, From};
use std::fmt;
use std::io::Read;
use byteorder::{NetworkEndian, ReadBytesExt};
use super::io::{ReadFrom, Error};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActionTypeTryFromErr(());

impl fmt::Display for ActionTypeTryFromErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid message type")
    }
}

impl From<ActionTypeTryFromErr> for Error {
    fn from(_: ActionTypeTryFromErr) -> Self {
        Error {}
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionType {
    Ping,
    Close = 2,
}

impl TryFrom<u16> for ActionType {
    type Err = ActionTypeTryFromErr;

    fn try_from(value: u16) -> Result<Self, Self::Err> {
        match value {
            0 => Ok(ActionType::Ping),
            2 => Ok(ActionType::Close),
            _ => Err(ActionTypeTryFromErr(())),
        }
    }
}

impl Into<u16> for ActionType {
    fn into(self) -> u16 {
        self as u16
    }
}

impl ReadFrom for ActionType {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error> {
        let raw_msg_type = read.read_u16::<NetworkEndian>()?;

        Ok(ActionType::try_from(raw_msg_type)?)
    }
}
