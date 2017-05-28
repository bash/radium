use std::env;
use std::io;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{ReadFrom, WriteTo, ReadResult, WriteResult, ReaderStatus, Reader};
use super::super::errors::{ReadError, WriteError};

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
#[derive(Debug)]
pub struct AddEntry {
    timestamp: i64,
    tag: u64,
    data: Vec<u8>,
}

enum AddEntryReaderState {
    Timestamp,
    Tag(i64),
    Length(i64, u64),
    Data(i64, u64, u16),
    Complete(i64, u64, u16, Vec<u8>),
}

struct AddEntryReader {
    state: AddEntryReaderState,
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

    fn reader() -> AddEntryReader {
        AddEntryReader { state: AddEntryReaderState::Timestamp }
    }
}

impl<R: io::Read> Reader<AddEntry, R> for AddEntryReader {
    fn resume(&mut self, input: &mut R) -> io::Result<ReaderStatus<AddEntry>> {
        let (state, status) = match self.state {
            AddEntryReaderState::Timestamp => {
                let timestamp = input.read_i64::<NetworkEndian>()?;

                (AddEntryReaderState::Tag(timestamp), ReaderStatus::Pending)
            },
            _ => { panic!("") }
        };

        self.state = state;

        Ok(status)
    }

    fn rewind(&mut self) {
        self.state = AddEntryReaderState::Timestamp;
    }
}

impl ReadFrom for AddEntry {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let timestamp = source.read_i64::<NetworkEndian>()?;
        let tag = source.read_u64::<NetworkEndian>()?;
        let length = source.read_u16::<NetworkEndian>()? as u64;

        if length > get_max_data_bytes() {
            return Err(ReadError::LimitReached);
        }

        let mut buf = Vec::new();
        let bytes_read = source.take(length).read_to_end(&mut buf)?;

        if (bytes_read as u64) < length {
            return Err(ReadError::UnexpectedEof);
        }

        Ok(AddEntry::new(timestamp, tag, buf))
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
    use std::error::Error;
    use super::*;

    #[test]
    fn test_read() {
        let mut source: Vec<u8> = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag  */ 0, 0, 0, 0, 0, 0, 0, 42,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3
        ];

        let msg = AddEntry::read_from(&mut source.as_mut_slice().as_ref()).unwrap();

        assert_eq!(10, msg.timestamp());
        assert_eq!(42, msg.tag());
        assert_eq!(&[1, 2, 3], msg.data());
        assert_eq!(3, msg.data().len());
    }

    #[test]
    fn test_read_respects_size() {
        let mut source: Vec<u8> = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag  */ 0, 0, 0, 0, 0, 0, 255, 255,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3, 4
        ];

        let msg = AddEntry::read_from(&mut source.as_mut_slice().as_ref()).unwrap();

        assert_eq!(10, msg.timestamp());
        assert_eq!(65535, msg.tag());
        assert_eq!(&[1, 2, 3], msg.data());
        assert_eq!(3, msg.data().len());
    }

    #[test]
    fn test_fails_on_data_eof() {
        let mut source: Vec<u8> = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* tag */  0, 0, 0, 0, 0, 0, 0, 123,
            /* len  */ 0, 10,
            /* data */ 1, 2, 3
        ];

        let result = AddEntry::read_from(&mut source.as_mut_slice().as_ref());

        assert_eq!(ReadError::UnexpectedEof.description(), result.err().unwrap().description());
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