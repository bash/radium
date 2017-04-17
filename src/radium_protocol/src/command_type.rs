use std::convert::TryFrom;
use super::TryFromError;

#[derive(Copy, Clone)]
pub enum CommandType {
    Ping,
    AddEntry,
    RemoveEntry,
    #[doc(hidden)]
    __NonExhaustive,
}

impl Into<u8> for CommandType {
    fn into(self) -> u8 {
        match self {
            CommandType::Ping => 0,
            CommandType::AddEntry => 1,
            CommandType::RemoveEntry => 2,
            _ => panic!("Invalid command type"),
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Ping),
            1 => Ok(CommandType::AddEntry),
            2 => Ok(CommandType::RemoveEntry),
            _ => Err(TryFromError::InvalidValue)
        }
    }
}