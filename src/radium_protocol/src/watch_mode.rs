/// The `WatchMode` indicates whether the client wants to be notified about
/// expired entries or not.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WatchMode<'a> {
    /// The client will not receive notifications
    #[serde(rename = "none")]
    None,
    /// The client will receive notifications for all tags
    #[serde(rename = "all")]
    All,
    /// The client will receive notifications only for one tag
    #[serde(rename = "tagged")]
    Tagged { tag: &'a str },
}

impl<'a> WatchMode<'a> {
    pub fn matches_tag<'b>(&self, other_tag: &'b str) -> bool {
        match self {
            &WatchMode::None => false,
            &WatchMode::All => true,
            &WatchMode::Tagged { ref tag } => tag == &other_tag,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    macro_rules! test_watch_mode {
        ($mode:expr, $serialized:expr) => {
            let serialized = serde_json::to_string(&$mode);
            assert_eq!($serialized, serialized.unwrap());

            let unserialized = serde_json::from_str($serialized);
            assert_eq!($mode, unserialized.unwrap());
        }
    }

    #[test]
    fn test_none() {
        test_watch_mode!(WatchMode::None, "{\"type\":\"none\"}");
    }

    #[test]
    fn test_all() {
        test_watch_mode!(WatchMode::All, "{\"type\":\"all\"}");
    }

    #[test]
    fn test_tagged() {
        test_watch_mode!(WatchMode::Tagged { tag: "foo" }, "{\"type\":\"tagged\",\"tag\":\"foo\"}");
    }
}