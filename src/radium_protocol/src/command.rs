use std::io;
use std::error::Error;
use std::fmt;
use byteorder::{WriteBytesExt, BigEndian};

pub enum Command {
    Ping,
    AddEntry(AddEntry),
    RemoveEntry(RemoveEntry),
}

/// ts: i64 | len: u16 | data: (len)
pub struct AddEntry {
    timestamp: i64,
    data: Vec<u8>,
}

/// ts: i64 | id: u16
pub struct RemoveEntry {
    timestamp: i64,
    id: u16,
}

pub enum CommandResult {
    Pong,
    EntryAdded { id: u16 },
    EntryRemoved,
}

#[derive(Debug)]
enum WriteError {
    DataLengthOverflow,
}

impl Error for WriteError {
    fn description(&self) -> &str {
        match self {
            &WriteError::DataLengthOverflow => "Data overflows maximum length",
        }
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Command {
    pub fn command_type(&self) -> u8 {
        match self {
            &Command::Ping => 0,
            &Command::AddEntry(_) => 1,
            &Command::RemoveEntry(_) => 2,
        }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_u8(self.command_type())?;

        match self {
            &Command::Ping => Ok(()),
            &Command::RemoveEntry(ref cmd) => cmd.write_to(target),
            &Command::AddEntry(ref cmd) => cmd.write_to(target),
        }
    }
}

impl AddEntry {
    pub fn new(timestamp: i64, data: Vec<u8>) -> Self {
        AddEntry { timestamp, data }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        let len = self.data.len();

        if len > u16::max_value() as usize {
            return Err(io::Error::new(io::ErrorKind::Other, WriteError::DataLengthOverflow));
        }

        target.write_i64::<BigEndian>(self.timestamp)?;
        target.write_u16::<BigEndian>(len as u16)?;

        target.write(&self.data)?;

        Ok(())
    }
}

impl RemoveEntry {
    pub fn new(timestamp: i64, id: u16) -> Self {
        RemoveEntry { timestamp, id }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_i64::<BigEndian>(self.timestamp)?;
        target.write_u16::<BigEndian>(self.id)?;

        Ok(())
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

    #[test]
    fn test_add_entry() {
        let cmd = Command::AddEntry(AddEntry::new(10, vec![1, 2, 3]));
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* cmd  */ 1,
                /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
                /* len  */ 0, 3,
                /* data */ 1, 2, 3
            ],
            vec
        );
    }

    #[test]
    fn test_add_entry_checks_size() {
        let mut data = Vec::<u8>::new();

        for _ in 0..((u16::max_value() as u32) + 1) {
            data.push(0);
        }

        let cmd = Command::AddEntry(AddEntry::new(0, data));
        let mut target = Vec::<u8>::new();

        let result = cmd.write_to(&mut target);

        assert!(result.is_err());
        assert!(result.err().unwrap().description() == WriteError::DataLengthOverflow.description())
    }

    #[test]
    fn test_remove_entry() {
        let cmd = Command::RemoveEntry(RemoveEntry::new(12345, 23));
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* cmd */ 2,
                /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
                /* id  */ 0, 23,
            ],
            vec
        );
    }
}