use std::io;
use byteorder::WriteBytesExt;
use super::{MessageType, WriteTo};
use super::messages::{AddEntry, EntryExpired, RemoveEntry};

pub enum Message {
    Ping,
    Pong,
    AddEntry(AddEntry),
    RemoveEntry(RemoveEntry),
    EntryExpired(EntryExpired),
    #[doc(hidden)]
    __NonExhaustive,
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            &Message::Ping => MessageType::Ping,
            &Message::Pong => MessageType::Pong,
            &Message::AddEntry(_) => MessageType::AddEntry,
            &Message::RemoveEntry(_) => MessageType::RemoveEntry,
            &Message::EntryExpired(_) => MessageType::EntryExpired,
            _ => panic!("invalid Message")
        }
    }

    pub fn is_command(&self) -> bool {
        self.message_type().is_command()
    }
}

impl WriteTo for Message {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.message_type().into())?;

        match self {
            &Message::Ping => Ok(()),
            &Message::Pong => Ok(()),
            &Message::RemoveEntry(ref msg) => msg.write_to(target),
            &Message::AddEntry(ref msg) => msg.write_to(target),
            &Message::EntryExpired(ref msg) => msg.write_to(target),
            _ => panic!("invalid Message")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        }
    }

    test_message!(test_ping, Ping);
    test_message!(test_pong, Pong);

    test_message!(test_add_entry,
                  Message::AddEntry(AddEntry::new(0, vec![])),
                  MessageType::AddEntry);

    test_message!(test_remove_entry,
                  Message::RemoveEntry(RemoveEntry::new(0, 0)),
                  MessageType::RemoveEntry);

    test_message!(test_entry_expired,
                  Message::EntryExpired(EntryExpired::new(0, vec![])),
                  MessageType::EntryExpired);
}