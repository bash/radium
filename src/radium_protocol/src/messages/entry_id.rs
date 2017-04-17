use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{ReadFrom, WriteTo, ReadError};

pub type EntryAdded = EntryId;
pub type RemoveEntry = EntryId;

/// ts: i64 | id: u16
pub struct EntryId {
    timestamp: i64,
    id: u16,
}

impl EntryId {
    pub fn new(timestamp: i64, id: u16) -> Self {
        EntryId { timestamp, id }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl ReadFrom for EntryId {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let timestamp = source.read_i64::<NetworkEndian>()?;
        let id = source.read_u16::<NetworkEndian>()?;

        Ok(EntryId::new(timestamp, id))
    }
}

impl WriteTo for EntryId {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
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
        let msg = Message::RemoveEntry(EntryId::new(12345, 23));
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

        let msg = EntryId::read_from(&mut vec.as_mut_slice().as_ref()).unwrap();

        assert_eq!(12345, msg.timestamp());
        assert_eq!(23, msg.id());
    }
}