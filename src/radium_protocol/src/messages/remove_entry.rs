use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{WriteTo, WriteResult, Message, MessageInner};
use super::super::reader::{Reader, ReaderStatus, HasReader};
use super::super::reader::ReaderStatus::{Pending, Complete};

/// ts: i64 | id: u16
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RemoveEntry {
    timestamp: i64,
    id: u16,
}

#[derive(Debug)]
enum ReaderState {
    Timestamp,
    Id(i64),
}

#[derive(Debug)]
pub struct RemoveEntryReader {
    state: ReaderState,
}

impl RemoveEntry {
    pub fn new(timestamp: i64, id: u16) -> Self {
        RemoveEntry { timestamp, id }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl MessageInner for RemoveEntry {
    fn wrap(self) -> Message {
        Message::RemoveEntry(self)
    }
}

impl HasReader for RemoveEntry {
    type Reader = RemoveEntryReader;

    fn reader() -> Self::Reader {
        RemoveEntryReader { state: ReaderState::Timestamp }
    }
}

impl Reader<RemoveEntry> for RemoveEntryReader {
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<RemoveEntry>>
        where I: io::Read
    {
        let (state, status) = match self.state {
            ReaderState::Timestamp => {
                let timestamp = input.read_i64::<NetworkEndian>()?;

                (ReaderState::Id(timestamp), Pending)
            }
            ReaderState::Id(timestamp) => {
                let id = input.read_u16::<NetworkEndian>()?;

                (ReaderState::Timestamp, Complete(RemoveEntry::new(timestamp, id)))
            }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = ReaderState::Timestamp;
    }
}

impl WriteTo for RemoveEntry {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        target.write_i64::<NetworkEndian>(self.timestamp)?;
        target.write_u16::<NetworkEndian>(self.id)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::Message;
    use super::super::super::WriteTo;

    #[test]
    fn test_write() {
        let msg = Message::RemoveEntry(RemoveEntry::new(12345, 23));
        let mut vec = Vec::<u8>::new();

        assert!(msg.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* cmd */ 4,
                /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
                /* id  */ 0, 23,
            ],
            vec
        );
    }

    #[test]
    fn test_reader() {
        let input = vec![
            /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
            /* id  */ 0, 23,
        ];

        let result = test_reader!(RemoveEntry::reader(), input);

        assert!(result.is_ok());
        assert_eq!(RemoveEntry::new(12345, 23), result.unwrap());
    }
}
