use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use super::super::{ReadFrom, WriteTo, ReadError};

/// ts: i64 | id: u16
#[derive(Debug)]
pub struct EntryAdded {
    timestamp: i64,
    id: u16,
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

impl ReadFrom for EntryAdded {
    fn read_from<R: io::Read>(source: &mut R) -> Result<Self, ReadError> {
        let timestamp = source.read_i64::<NetworkEndian>()?;
        let id = source.read_u16::<NetworkEndian>()?;

        Ok(EntryAdded::new(timestamp, id))
    }
}

impl WriteTo for EntryAdded {
    fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_i64::<NetworkEndian>(self.timestamp)?;
        target.write_u16::<NetworkEndian>(self.id)?;

        Ok(())
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
        let mut vec: Vec<u8> = vec![
            /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
            /* id  */ 0, 23,
        ];

        let msg = EntryAdded::read_from(&mut vec.as_mut_slice().as_ref()).unwrap();

        assert_eq!(12345, msg.timestamp());
        assert_eq!(23, msg.id());
    }
}