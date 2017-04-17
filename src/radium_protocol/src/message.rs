use std::io;
use byteorder::WriteBytesExt;
use super::{MessageType, WriteTo};
use super::messages::{AddEntry, EntryExpired, RemoveEntry};

pub enum Message {
    Ping,
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

    #[test]
    fn test_ping() {
        let cmd = Message::Ping;
        let mut vec = Vec::new();

        assert!(cmd.write_to(&mut vec).is_ok());
        assert_eq!(vec![0], vec);
    }
}