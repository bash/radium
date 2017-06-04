use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{WriteTo, WriteResult, Reader, ReaderStatus, MessageInner, Message, HasReader};
use ReaderStatus::{Pending, Complete};

/// ts: i64 | id: u16
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EntryAdded {
    timestamp: i64,
    id: u16,
}

#[derive(Debug)]
enum ReaderState {
    Timestamp,
    Id(i64),
}

#[derive(Debug)]
pub struct EntryAddedReader {
    state: ReaderState,
}

impl EntryAdded {
    pub fn new(timestamp: i64, id: u16) -> Self {
        EntryAdded { timestamp, id }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl MessageInner for EntryAdded {
    fn wrap(self) -> Message {
        Message::EntryAdded(self)
    }
}

impl HasReader for EntryAdded {
    type Reader = EntryAddedReader;

    fn reader() -> Self::Reader {
        EntryAddedReader { state: ReaderState::Timestamp }
    }
}

impl WriteTo for EntryAdded {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        target.write_i64::<NetworkEndian>(self.timestamp)?;
        target.write_u16::<NetworkEndian>(self.id)?;

        Ok(())
    }
}

impl Reader<EntryAdded> for EntryAddedReader {
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<EntryAdded>> where I: io::Read {
        let (state, status) = match self.state {
            ReaderState::Timestamp => {
                let timestamp = input.read_i64::<NetworkEndian>()?;

                (ReaderState::Id(timestamp), Pending)
            }
            ReaderState::Id(timestamp) => {
                let id = input.read_u16::<NetworkEndian>()?;
                let inner = EntryAdded::new(timestamp, id);

                (ReaderState::Timestamp, Complete(inner))
            }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = ReaderState::Timestamp;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::WriteTo;

    #[test]
    fn test_write() {
        let msg = EntryAdded::new(12345, 23);
        let mut vec = Vec::<u8>::new();

        assert!(msg.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
                /* id  */ 0, 23,
            ],
            vec
        );
    }

    #[test]
    fn test_read() {
        let input = vec![
            /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
            /* id  */ 0, 23,
        ];

        let result = test_reader2!(EntryAdded::reader(), input);

        assert!(result.is_ok());
        assert_eq!(EntryAdded::new(12345, 23), result.unwrap());
    }
}