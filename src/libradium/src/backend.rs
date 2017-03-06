use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use ::entry::Entry;

pub type SharedBackend = Arc<Mutex<Backend>>;

#[derive(Debug)]
pub struct Backend {
    items: BTreeSet<Entry>
}

impl Backend {
    pub fn add(&mut self, entry: Entry) {
        self.items.insert(entry);
    }

    pub fn items(&self) -> &BTreeSet<Entry> {
        &self.items
    }

    pub fn new() -> Self {
        Backend {
            items: BTreeSet::new()
        }
    }
}