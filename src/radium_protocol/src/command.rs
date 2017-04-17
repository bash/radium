use std::io;
use byteorder::WriteBytesExt;
use super::{CommandType, AddEntry, RemoveEntry};

pub enum Command {
    Ping,
    AddEntry(AddEntry),
    RemoveEntry(RemoveEntry),
    #[doc(hidden)]
    __NonExhaustive,
}

pub enum CommandResult {
    Pong,
    EntryAdded { id: u16 },
    EntryRemoved,
}

impl Command {
    pub fn command_type(&self) -> CommandType {
        match self {
            &Command::Ping => CommandType::Ping,
            &Command::AddEntry(_) => CommandType::AddEntry,
            &Command::RemoveEntry(_) => CommandType::RemoveEntry,
            _ => panic!("invalid command")
        }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.command_type().into())?;

        match self {
            &Command::Ping => Ok(()),
            &Command::RemoveEntry(ref cmd) => cmd.write_to(target),
            &Command::AddEntry(ref cmd) => cmd.write_to(target),
            _ => panic!("invalid command")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ping() {
        let cmd = Command::Ping;
        let mut vec = Vec::new();

        assert!(cmd.write_to(&mut vec).is_ok());
        assert_eq!(vec![0], vec);
    }
}