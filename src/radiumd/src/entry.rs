use libradium;

#[derive(Clone, Debug)]
pub struct EntryData {
    tag: u64,
    data: Vec<u8>,
}

pub type Entry = libradium::Entry<EntryData>;

impl EntryData {
    pub fn new(tag: u64, data: Vec<u8>) -> Self {
        EntryData { tag, data }
    }

    pub fn tag(&self) -> u64 {
        self.tag
    }

    pub fn consume_data(self) -> Vec<u8> {
        self.data
    }
}
