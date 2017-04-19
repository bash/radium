use std::io;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{ReadFrom, WriteTo, ReadError, EntryWriteError};

/// ts: i64 | len: u16 | data: (len < 2**16)
pub struct AddEntry {
    timestamp: i64,
    data: Vec<u8>,
}

impl AddEntry {
    pub fn new<T: Into<i64>>(timestamp: T, data: Vec<u8>) -> Self {
        AddEntry {
            timestamp: timestamp.into(),
            data
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl ReadFrom for AddEntry {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let timestamp = source.read_i64::<NetworkEndian>()?;
        let length = source.read_u16::<NetworkEndian>()? as u64;

        let mut buf = Vec::new();
        let bytes_read = source.take(length).read_to_end(&mut buf)?;

        if (bytes_read as u64) < length {
            return Err(ReadError::UnexpectedEof);
        }

        Ok(AddEntry::new(timestamp, buf))
    }
}

impl WriteTo for AddEntry {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        let len = self.data.len();

        if len > u16::max_value() as usize {
            return Err(io::Error::new(io::ErrorKind::Other, EntryWriteError::DataLengthOverflow));
        }

        target.write_i64::<NetworkEndian>(self.timestamp)?;
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
            /* len  */ 0, 3,
            /* data */ 1, 2, 3
        ];

        let msg = AddEntry::read_from(&mut source.as_mut_slice().as_ref()).unwrap();

        assert_eq!(10, msg.timestamp());
        assert_eq!(&[1, 2, 3], msg.data());
        assert_eq!(3, msg.data().len());
    }

    #[test]
    fn test_read_respects_size() {
        let mut source: Vec<u8> = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* len  */ 0, 3,
            /* data */ 1, 2, 3, 4
        ];

        let msg = AddEntry::read_from(&mut source.as_mut_slice().as_ref()).unwrap();

        assert_eq!(10, msg.timestamp());
        assert_eq!(&[1, 2, 3], msg.data());
        assert_eq!(3, msg.data().len());
    }

    #[test]
    fn test_fails_on_data_eof() {
        let mut source: Vec<u8> = vec![
            /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
            /* len  */ 0, 10,
            /* data */ 1, 2, 3
        ];

        let result = AddEntry::read_from(&mut source.as_mut_slice().as_ref());

        assert_eq!(ReadError::UnexpectedEof.description(), result.err().unwrap().description());
    }

    #[test]
    fn test_write() {
        let cmd = AddEntry::new(10, vec![1, 2, 3]);
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
            vec![
                /* ts   */ 0, 0, 0, 0, 0, 0, 0, 10,
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

        let cmd = AddEntry::new(0, data);
        let mut target = Vec::<u8>::new();

        let result = cmd.write_to(&mut target);

        assert!(result.is_err());
        assert!(result.err().unwrap().description() == EntryWriteError::DataLengthOverflow.description())
    }
}