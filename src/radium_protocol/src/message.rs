use std::io;
use byteorder::WriteBytesExt;
use super::{MessageType, ReadFrom, ReadError, WriteTo};
use super::messages::{AddEntry, EntryAdded, EntryExpired, RemoveEntry, SetWatchMode, ErrorMessage};

#[derive(Debug)]
pub enum Message {
    Ping,
    Pong,
    AddEntry(AddEntry),
    EntryAdded(EntryAdded),
    RemoveEntry(RemoveEntry),
    // `EntryRemoved` should also contain the entry's data. However, this requires changing
    // libradium, because the frontend does not block when adding or removing entries.
    // until then, we use Ok as confirmation
    EntryRemoved,
    EntryExpired(EntryExpired),
    SetWatchMode(SetWatchMode),
    Ok,
    Error(ErrorMessage),
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            &Message::Ping => MessageType::Ping,
            &Message::Pong => MessageType::Pong,
            &Message::AddEntry(..) => MessageType::AddEntry,
            &Message::EntryAdded(..) => MessageType::EntryAdded,
            &Message::RemoveEntry(..) => MessageType::RemoveEntry,
            &Message::EntryRemoved => MessageType::EntryRemoved,
            &Message::EntryExpired(..) => MessageType::EntryExpired,
            &Message::SetWatchMode(..) => MessageType::SetWatchMode,
            &Message::Ok => MessageType::Ok,
            &Message::Error(..) => MessageType::Error,
        }
    }

    pub fn is_command(&self) -> bool {
        self.message_type().is_command()
    }
}

impl ReadFrom for Message {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let msg_type = MessageType::read_from(source)?;

        match msg_type {
            MessageType::Ping => Ok(Message::Ping),
            MessageType::Pong => Ok(Message::Pong),
            MessageType::AddEntry => Ok(Message::AddEntry(AddEntry::read_from(source)?)),
            MessageType::EntryAdded => Ok(Message::EntryAdded(EntryAdded::read_from(source)?)),
            MessageType::RemoveEntry => Ok(Message::RemoveEntry(RemoveEntry::read_from(source)?)),
            MessageType::EntryRemoved => Ok(Message::EntryRemoved),
            MessageType::EntryExpired => Ok(Message::EntryExpired(EntryExpired::read_from(source)?)),
            MessageType::SetWatchMode => Ok(Message::SetWatchMode(SetWatchMode::read_from(source)?)),
            MessageType::Ok => Ok(Message::Ok),
            MessageType::Error => Ok(Message::Error(ErrorMessage::read_from(source)?)),
        }
    }
}

impl WriteTo for Message {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.message_type().into())?;

        match self {
            &Message::Ping => Ok(()),
            &Message::Pong => Ok(()),
            &Message::AddEntry(ref msg) => msg.write_to(target),
            &Message::EntryAdded(ref msg) => msg.write_to(target),
            &Message::RemoveEntry(ref msg) => msg.write_to(target),
            &Message::EntryRemoved => Ok(()),
            &Message::EntryExpired(ref msg) => msg.write_to(target),
            &Message::SetWatchMode(ref msg) => msg.write_to(target),
            &Message::Ok => Ok(()),
            &Message::Error(ref msg) => msg.write_to(target),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::WatchMode;
    use super::super::messages::ErrorCode;

    macro_rules! test_message {
        ($test:ident, $ty:ident) => {
            test_message!($test, Message::$ty, MessageType::$ty);
        };
        ($test:ident, $msg:expr, $ty:expr) => {
            #[test]
            fn $test() {
                let msg = $msg;

                assert_eq!($ty, msg.message_type());

                let mut vec = Vec::new();
                assert!(msg.write_to(&mut vec).is_ok());
                assert_eq!(msg.message_type().to_u8(), vec[0]);

                let read_msg = Message::read_from(&mut vec.as_mut_slice().as_ref()).unwrap();
                assert_eq!(msg.message_type(), read_msg.message_type());
            }
        };
    }

    test_message!(test_ping, Ping);
    test_message!(test_pong, Pong);

    test_message!(test_add_entry,
                  Message::AddEntry(AddEntry::new(0, vec![])),
                  MessageType::AddEntry);

    test_message!(test_entry_added,
                  Message::EntryAdded(EntryAdded::new(0, 0)),
                  MessageType::EntryAdded);

    test_message!(test_remove_entry,
                  Message::RemoveEntry(RemoveEntry::new(0, 0)),
                  MessageType::RemoveEntry);

    test_message!(test_entry_removed, EntryRemoved);

    test_message!(test_entry_expired,
                  Message::EntryExpired(EntryExpired::new(0, 7, vec![])),
                  MessageType::EntryExpired);

    test_message!(test_ok, Ok);

    test_message!(test_set_watch_mode,
                  Message::SetWatchMode(SetWatchMode::new(WatchMode::None)),
                  MessageType::SetWatchMode);

    test_message!(test_error, Message::Error(ErrorMessage::new(ErrorCode::ClientRejected)), MessageType::Error);
}