use std::collections::BTreeSet;
use libradium::entry::Entry;

#[derive(Debug)]
pub struct Backend {
    items: BTreeSet<Entry>
}

impl Backend {
    pub fn add(&mut self, entry: Entry) {
        self.items.insert(entry);
    }

    pub fn new() -> Self {
        Backend {
            items: BTreeSet::new()
        }
    }
}