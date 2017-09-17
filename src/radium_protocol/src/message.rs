use super::error_code::ErrorCode;
use super::watch_mode::WatchMode;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Message<'a> {
    // TODO: determine if ping/pong is still required
    Ping,
    AddEntry {
        timestamp: i64,
        tag: &'a str,
        data: &'a str,
    },
    RemoveEntry { timestamp: i64, id: u16 },
    SetWatchMode { mode: WatchMode<'a> },
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage<'a> {
    // TODO: determine if ping/pong is still required
    Pong,
    EntryAdded { timestamp: i64, id: u16 },
    EntryExpired {
        timestamp: i64,
        tag: &'a str,
        data: &'a str,
    },
    Ok { message: Message<'a> },
    Error {
        message: Message<'a>,
        error: ErrorCode,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        let serialized = serde_json::to_string(&Message::Ping);

        assert_eq!("{\"type\":\"ping\"}", serialized.unwrap());
    }
}
