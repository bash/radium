use std::io;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{WriteTo, WriteResult, Message, MessageInner};
use super::super::errors::{WriteError, DataLengthError};
use super::super::reader::{Reader, ReaderStatus, HasReader};
use super::super::reader::ReaderStatus::{Pending, Complete};

/// ts: i64 | id: u16 | tag: u64 | len: u16 | data: (len < 2**16)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EntryExpired {
    timestamp: i64,
    id: u16,
    tag: u64,
    data: Vec<u8>,
}

#[derive(Debug)]
enum ReaderState {
    Timestamp,
    Id(i64),
    Tag(i64, u16),
    Length(i64, u16, u64),
    Data(i64, u16, u64, u64),
}

impl ReaderState {
    fn initial() -> ReaderState {
        ReaderState::Timestamp
    }
}

#[derive(Debug)]
pub struct EntryExpiredReader {
    state: ReaderState,
}

impl EntryExpired {
    pub fn new<T: Into<i64>>(timestamp: T, id: u16, tag: u64, data: Vec<u8>) -> Self {
        EntryExpired {
            timestamp: timestamp.into(),
            id,
            tag,
            data,
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn tag(&self) -> u64 {
        self.tag
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl MessageInner for EntryExpired {
    fn wrap(self) -> Message {
        Message::EntryExpired(self)
    }
}

impl HasReader for EntryExpired {
    type Reader = EntryExpiredReader;

    fn reader() -> Self::Reader {
        EntryExpiredReader { state: ReaderState::initial() }
    }
}

impl WriteTo for EntryExpired {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        let len = self.data.len();

        if len > u16::max_value() as usize {
            return Err(WriteError::DataLengthOverflow);
        }

        target.write_i64::<NetworkEndian>(self.timestamp)?;
        target.write_u16::<NetworkEndian>(self.id)?;
        target.write_u64::<NetworkEndian>(self.tag)?;
        target.write_u16::<NetworkEndian>(len as u16)?;

        target.write(&self.data)?;

        Ok(())
    }
}

impl Reader<EntryExpired> for EntryExpiredReader {
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<EntryExpired>> where I: io::Read {
        let (state, status) = match self.state {
            ReaderState::Timestamp => {
                let timestamp = input.read_i64::<NetworkEndian>()?;

                (ReaderState::Id(timestamp), Pending)
            }
            ReaderState::Id(timestamp) => {
                let id = input.read_u16::<NetworkEndian>()?;

                (ReaderState::Tag(timestamp, id), Pending)
            }
            ReaderState::Tag(timestamp, id) => {
                let tag = input.read_u64::<NetworkEndian>()?;

                (ReaderState::Length(timestamp, id, tag), Pending)
            }
            ReaderState::Length(timestamp, id, tag) => {
                let length = input.read_u16::<NetworkEndian>()?;

                (ReaderState::Data(timestamp, id, tag, length as u64), Pending)
            }
            ReaderState::Data(timestamp, id, tag, length) => {
                let mut buf = Vec::new();
                let bytes_read = input.take(length).read_to_end(&mut buf)?;

                if (bytes_read as u64) < length {
                    return Err(DataLengthError::new());
                }

                (ReaderState::initial(), Complete(EntryExpired::new(timestamp, id, tag, buf)))
            }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = ReaderState::initial();
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use super::*;

    #[test]
    fn test_read() {
        let input = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* id   */ 0, 7,
            /* tag  */ 0, 0, 0, 0, 0, 0, 0, 42,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3,
        ];

        let result = test_reader2!(EntryExpired::reader(), input);

        assert!(result.is_ok());
        assert_eq!(EntryExpired::new(10, 7, 42, vec![1, 2, 3]), result.unwrap());
    }

    #[test]
    fn test_read_respects_size() {
        let input = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* id   */ 0, 7,
            /* tag  */ 0, 0, 0, 0, 0, 0, 0, 32,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3, 4,
        ];

        let result = test_reader2!(EntryExpired::reader(), input);

        assert!(result.is_ok());
        assert_eq!(EntryExpired::new(10, 7, 32, vec![1, 2, 3]), result.unwrap());
    }

    #[test]
    fn test_fails_on_data_eof() {
        let input = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* id   */ 0, 7,
            /* tag  */ 0, 0, 0, 0, 0, 0, 0, 42,
            /* len  */ 0, 10,
            /* data */ 1, 2, 3,
        ];

        let result = test_reader2!(EntryExpired::reader(), input);

        assert!(result.is_err());
        assert_eq!(DataLengthError::new().description(), result.unwrap_err().description());
    }

    #[test]
    fn test_write() {
        let cmd = EntryExpired::new(10, 7, 12, vec![1, 2, 3]);
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
                /* id   */ 0, 7,
                /* tag  */ 0, 0, 0, 0, 0, 0, 0, 12,
                /* len  */ 0, 3,
                /* data */ 1, 2, 3,
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

        let cmd = EntryExpired::new(0, 7, 0, data);
        let mut target = Vec::<u8>::new();

        let result = cmd.write_to(&mut target);

        assert!(result.is_err());
        assert!(result.err().unwrap().description() == WriteError::DataLengthOverflow.description())
    }
}