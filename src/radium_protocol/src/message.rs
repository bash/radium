use std::io;
use byteorder::WriteBytesExt;
use super::{MessageType, WriteTo, WriteResult, ReaderStatus, Reader};
use super::messages::{AddEntry, EntryAdded, EntryExpired, RemoveEntry, SetWatchMode, ErrorMessage, SetWatchModeReader, AddEntryReader};

macro_rules! msg_reader {
    ($reader: expr, $input: expr) => {
        match $reader.resume($input)? {
            ReaderStatus::Pending => (None, ReaderStatus::Pending),
            ReaderStatus::Complete(inner) => {
                let msg = inner.wrap();

                (Some(ReaderState::Type), ReaderStatus::Complete(msg))
            },
        }
    }
}

macro_rules! into_msg_reader {
    ($variant: ident) => {
        into_msg_reader!($variant, $variant)
    };
    ($variant: ident, $msg: ident) => {
        (Some(ReaderState::$variant($msg::reader())), ReaderStatus::Pending)
    };
}

macro_rules! empty_msg {
    ($variant: ident) => {
        (Some(ReaderState::Type), ReaderStatus::Complete(Message::$variant))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    Ping,
    Pong,
    AddEntry(AddEntry),
    EntryAdded(EntryAdded),
    RemoveEntry(RemoveEntry),
    // `EntryRemoved` should also contain the entry's data. However, this requires changing
    // libradium, because the frontend does not block when adding or removing entries.
    // until then, we use Ok as confirmation
    #[doc(hidden)]
    EntryRemoved,
    EntryExpired(EntryExpired),
    SetWatchMode(SetWatchMode),
    Ok,
    Error(ErrorMessage),
}

#[derive(Debug)]
enum ReaderState {
    Type,
    Message(MessageType),
    SetWatchMode(SetWatchModeReader),
    AddEntry(AddEntryReader)
}

#[derive(Debug)]
pub struct MessageReader {
    state: ReaderState
}

pub trait MessageInner {
    /// Wraps the inner message inside its corresponding `Message` variant
    fn wrap(self) -> Message;
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

    /// Determines if the message is a command that is handled by the server
    pub fn is_command(&self) -> bool {
        self.message_type().is_command()
    }

    pub fn reader() -> MessageReader {
        MessageReader { state: ReaderState::Type }
    }
}

impl Reader<Message> for MessageReader {
    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<Message>> where R: io::Read {
        let (state, status) = match self.state {
            ReaderState::Type => {
                let state = match MessageType::reader().resume(input)? {
                    ReaderStatus::Pending => None,
                    ReaderStatus::Complete(val) => Some(ReaderState::Message(val))
                };

                (state, ReaderStatus::Pending)
            },
            ReaderState::Message(msg_type) => {
                match msg_type {
                    MessageType::Ping => empty_msg!(Ping),
                    MessageType::Pong => empty_msg!(Pong),
                    MessageType::EntryRemoved => empty_msg!(EntryRemoved),
                    MessageType::Ok => empty_msg!(Ok),
                    MessageType::SetWatchMode => into_msg_reader!(SetWatchMode),
                    MessageType::AddEntry => into_msg_reader!(AddEntry),
                    _ => { panic!("not implemented") }
                }
            },
            ReaderState::SetWatchMode(ref mut reader) => msg_reader!(reader, input),
            ReaderState::AddEntry(ref mut reader) => msg_reader!(reader, input),
        };

        if let Some(state) = state {
            self.state = state;
        }

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = ReaderState::Type;
    }
}

impl WriteTo for Message {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
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
    use super::super::{WatchMode, ErrorCode};
    use super::super::messages::SetWatchMode;

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
            }
        };
    }

    test_message!(test_ping, Ping);
    test_message!(test_pong, Pong);

    test_message!(test_add_entry,
                  Message::AddEntry(AddEntry::new(0, 0, vec![])),
                  MessageType::AddEntry);

    test_message!(test_entry_added,
                  Message::EntryAdded(EntryAdded::new(0, 0)),
                  MessageType::EntryAdded);

    test_message!(test_remove_entry,
                  Message::RemoveEntry(RemoveEntry::new(0, 0)),
                  MessageType::RemoveEntry);

    test_message!(test_entry_removed, EntryRemoved);

    test_message!(test_entry_expired,
                  Message::EntryExpired(EntryExpired::new(0, 7, 12, vec![])),
                  MessageType::EntryExpired);

    test_message!(test_ok, Ok);

    test_message!(test_set_watch_mode,
                  Message::SetWatchMode(SetWatchMode::new(WatchMode::None)),
                  MessageType::SetWatchMode);

    test_message!(test_error, Message::Error(ErrorMessage::new(ErrorCode::ClientRejected)), MessageType::Error);
}