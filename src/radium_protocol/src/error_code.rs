use std::convert::TryFrom;
use std::fmt;
use super::errors::TryFromError;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::u8;

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

struct ErrorCodeVisitor;

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

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> Visitor<'de> for ErrorCodeVisitor {
    type Value = ErrorCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 4")
    }

    fn visit_i64<E>(self, value: i64) -> Result<ErrorCode, E>
        where E: de::Error
    {
        if value >= u8::MIN as i64 && value <= u8::MAX as i64 {
            match ErrorCode::try_from(value as u8) {
                Ok(code) => Ok(code),
                Err(_) => Err(E::custom(format!("invalid value: {}", value)))
            }
        } else {
            Err(E::custom(format!("invalid value: {}", value)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<ErrorCode, E>
        where E: de::Error
    {
        if value >= u8::MIN as u64 && value <= u8::MAX as u64 {
            match ErrorCode::try_from(value as u8) {
                Ok(code) => Ok(code),
                Err(_) => Err(E::custom(format!("invalid value: {}", value)))
            }
        } else {
            Err(E::custom(format!("invalid value: {}", value)))
        }
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<ErrorCode, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u8(ErrorCodeVisitor)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    pub fn test_serialize() {
        let code = ErrorCode::ActionProcessingError;
        let serialized = serde_json::to_string(&code);

        assert_eq!("3".to_string(), serialized.unwrap());
    }

    #[test]
    pub fn test_deserialize() {
        let deserialized = serde_json::from_str("3");

        assert_eq!(ErrorCode::ActionProcessingError, deserialized.unwrap());
    }
}
