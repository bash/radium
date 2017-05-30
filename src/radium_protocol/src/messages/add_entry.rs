use std::env;
use std::io;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{WriteTo, WriteResult, ReaderStatus, Reader, MessageInner, Message};
use super::super::errors::{WriteError, DataLengthError};

use ReaderStatus::{Complete, Pending};

/// default value for maximum bytes of data (2KiB)
const MAX_DATA_BYTES: u64 = 2048;

fn get_max_data_bytes() -> u64 {
    match env::var("RADIUM_MAX_DATA_BYTES") {
        Ok(val) => match val.parse::<u64>() {
            Ok(val) => val,
            Err(..) => MAX_DATA_BYTES
        },
        Err(..) => MAX_DATA_BYTES
    }
}

/// ts: i64 | len: u16 | data: (len < 2**16)
#[derive(Debug, Eq, PartialEq)]
pub struct AddEntry {
    timestamp: i64,
    tag: u64,
    data: Vec<u8>,
}

#[derive(Debug)]
enum ReaderState {
    Timestamp,
    Tag(i64),
    Length(i64, u64),
    Data(i64, u64, u64),
}

#[derive(Debug)]
pub struct AddEntryReader {
    state: ReaderState,
}

impl AddEntry {
    pub fn new<T: Into<i64>>(timestamp: T, tag: u64, data: Vec<u8>) -> Self {
        AddEntry {
            timestamp: timestamp.into(),
            tag,
            data
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn tag(&self) -> u64 {
        self.tag
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn consume_data(self) -> Vec<u8> {
        self.data
    }

    pub fn reader() -> AddEntryReader {
        AddEntryReader { state: ReaderState::Timestamp }
    }
}

impl MessageInner for AddEntry {
    fn wrap(self) -> Message {
        Message::AddEntry(self)
    }
}

impl Reader<AddEntry> for AddEntryReader {
    fn resume<R>(&mut self, input: &mut R) -> io::Result<ReaderStatus<AddEntry>> where R: io::Read {
        let (state, status) = match self.state {
            ReaderState::Timestamp => {
                let timestamp = input.read_i64::<NetworkEndian>()?;

                (ReaderState::Tag(timestamp), Pending)
            },
            ReaderState::Tag(timestamp) => {
                let tag = input.read_u64::<NetworkEndian>()?;

                (ReaderState::Length(timestamp, tag), Pending)
            },
            ReaderState::Length(timestamp, tag) => {
                let length = input.read_u16::<NetworkEndian>()? as u64;

                if length > get_max_data_bytes() {
                    return Err(DataLengthError::new());
                }

                (ReaderState::Data(timestamp, tag, length), Pending)
            },
            ReaderState::Data(timestamp, tag, length) => {
                let mut buf = Vec::new();
                let bytes_read = input.take(length).read_to_end(&mut buf)?;

                if (bytes_read as u64) < length {
                    return Err(DataLengthError::new());
                }

                (ReaderState::Timestamp, Complete(AddEntry::new(timestamp, tag, buf)))
            },
        };

        self.state = state;

        Ok(status)
    }

    fn rewind (&mut self) {
        self.state = ReaderState::Timestamp;
    }
}

impl WriteTo for AddEntry {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        let len = self.data.len();

        if len > u16::max_value() as usize {
            return Err(WriteError::DataLengthOverflow);
        }

        target.write_i64::<NetworkEndian>(self.timestamp)?;
        target.write_u64::<NetworkEndian>(self.tag)?;
        target.write_u16::<NetworkEndian>(len as u16)?;

        target.write(&self.data)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use std::io::Cursor;
    use super::super::super::{Message, MessageType};
    use super::super::super::errors::DataLengthError;

    #[test]
    fn test_reader() {
        let input = vec![
            /* type */ MessageType::AddEntry.into(),
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag  */ 0, 0, 0, 0, 0, 0, 0, 42,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3
        ];

        test_reader! {
            Message::reader(),
            input,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Complete(Message::AddEntry(AddEntry::new(10, 42, vec![1, 2, 3])))
        };
    }

    #[test]
    fn test_read_respects_size() {
        let input = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag  */ 0, 0, 0, 0, 0, 0, 255, 255,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3, 4
        ];

        test_reader! {
            AddEntry::reader(),
            input,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Pending,
            ReaderStatus::Complete(AddEntry::new(10, 65535, vec![1, 2, 3]))
        };
    }

    #[test]
    fn test_fails_on_data_eof() {
        let vec = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag */  0, 0, 0, 0, 0, 0, 0, 123,
            /* len  */ 0, 10,
            /* data */ 1, 2, 3
        ];

        let input = &mut Cursor::new(vec);
        let mut reader = AddEntry::reader();

        assert_eq!(ReaderStatus::Pending, reader.resume(input).unwrap());
        assert_eq!(ReaderStatus::Pending, reader.resume(input).unwrap());
        assert_eq!(ReaderStatus::Pending, reader.resume(input).unwrap());

        let result = reader.resume(input);

        assert_eq!(DataLengthError::new().description(), result.err().unwrap().description());
    }

    #[test]
    fn test_write() {
        let cmd = AddEntry::new(10, 128, vec![1, 2, 3]);
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
                /* tag  */ 0, 0, 0, 0, 0, 0, 0, 128,
                /* len  */ 0, 3,
                /* data */ 1, 2, 3
            ],
            vec
        );
    }

    #[test]
    fn test_write_checks_size() {
        let mut data = Vec::<u8>::new();

        for _ in 0..((u16::max_value() as u32) + 1) {
            data.push(0);
        }

        let cmd = AddEntry::new(0, 0, data);
        let mut target = Vec::<u8>::new();

        let result = cmd.write_to(&mut target);

        assert!(result.is_err());
        assert!(result.err().unwrap().description() == WriteError::DataLengthOverflow.description())
    }
}