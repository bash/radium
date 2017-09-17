extern crate radium_protocol;
extern crate serde_json;

use radium_protocol::{Message, ServerMessage, WatchMode, ErrorCode};

fn main() {
    {
        let message = Message::SetWatchMode { mode: WatchMode::Tagged { tag: "foobar" } };
        let serialized = serde_json::to_string(&message).unwrap();
        println!("{}", serialized);
    }

    {
        let message = ServerMessage::Error {
            message: Message::Ping,
            error: ErrorCode::ConnectionFailure,
        };
        let serialized = serde_json::to_string(&message).unwrap();
        println!("{}", serialized);
    }
}
