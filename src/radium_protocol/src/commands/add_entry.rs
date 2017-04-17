use std::io;
use std::error::Error;
use byteorder::{WriteBytesExt, BigEndian};

#[derive(Debug)]
enum AddEntryWriteError {
    DataLengthOverflow,
}

/// ts: i64 | len: u16 | data: (len)
pub struct AddEntry {
    timestamp: i64,
    data: Vec<u8>,
}

impl_err_display!(AddEntryWriteError);

impl Error for AddEntryWriteError {
    fn description(&self) -> &str {
        match self {
            &AddEntryWriteError::DataLengthOverflow => "Data overflows maximum length",
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
            return Err(io::Error::new(io::ErrorKind::Other, AddEntryWriteError::DataLengthOverflow));
        }

        target.write_i64::<BigEndian>(self.timestamp)?;
        target.write_u16::<BigEndian>(len as u16)?;

        target.write(&self.data)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::Command;

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
        assert!(result.err().unwrap().description() == AddEntryWriteError::DataLengthOverflow.description())
    }
}