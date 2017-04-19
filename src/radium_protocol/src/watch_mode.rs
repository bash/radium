use std::convert::TryFrom;
use super::TryFromError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The `WatchMode` indicates whether the client wants to be notified about
/// expired entries or not.
pub enum WatchMode {
    /// The client will not receive notifications
    None,
    /// The client will receive notifications
    Watching
}

impl Into<u8> for WatchMode {
    fn into(self) -> u8 {
        match self {
            WatchMode::None => 0,
            WatchMode::Watching => 1,
        }
    }
}

impl TryFrom<u8> for WatchMode {
    type Error = TryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WatchMode::None),
            1 => Ok(WatchMode::Watching),
            _ => Err(TryFromError::InvalidValue),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into() {
        assert_eq!(0u8, WatchMode::None.into());
        assert_eq!(1u8, WatchMode::Watching.into());
    }

    #[test]
    fn test_from() {
        assert_eq!(WatchMode::None, WatchMode::try_from(0).unwrap());
        assert_eq!(WatchMode::Watching, WatchMode::try_from(1).unwrap());
    }
}