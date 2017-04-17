use std::io;
use byteorder::{WriteBytesExt, BigEndian};

/// ts: i64 | id: u16
pub struct RemoveEntry {
    timestamp: i64,
    id: u16,
}

impl RemoveEntry {
    pub fn new(timestamp: i64, id: u16) -> Self {
        RemoveEntry { timestamp, id }
    }

    pub fn write_to<W: io::Write>(&self, target: &mut W) -> io::Result<()> {
        target.write_i64::<BigEndian>(self.timestamp)?;
        target.write_u16::<BigEndian>(self.id)?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::Message;

    #[test]
    fn test_remove_entry() {
        let cmd = Message::RemoveEntry(RemoveEntry::new(12345, 23));
        let mut vec = Vec::<u8>::new();

        assert!(cmd.write_to(&mut vec).is_ok());

        assert_eq!(
        vec![
            /* cmd */ 2,
            /* ts  */ 0, 0, 0, 0, 0, 0, 48, 57,
            /* id  */ 0, 23,
        ],
        vec
        );
    }
}