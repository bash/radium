use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{ReadFrom, WriteTo, ReadResult, WriteResult};

/// ts: i64 | id: u16
#[derive(Debug)]
pub struct RemoveEntry {
    timestamp: i64,
    id: u16,
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

impl ReadFrom for RemoveEntry {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let timestamp = source.read_i64::<NetworkEndian>()?;
        let id = source.read_u16::<NetworkEndian>()?;

        Ok(RemoveEntry::new(timestamp, id))
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
    fn test_read() {
        let mut vec: Vec<u8> = vec![
            /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
            /* id  */ 0, 23,
        ];

        let msg = RemoveEntry::read_from(&mut vec.as_mut_slice().as_ref()).unwrap();

        assert_eq!(12345, msg.timestamp());
        assert_eq!(23, msg.id());
    }
}