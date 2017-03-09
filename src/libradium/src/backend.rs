use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use super::entry::Entry;

pub type SharedBackend = Arc<Mutex<Backend>>;

#[derive(Debug)]
pub struct Backend {
    entries: BTreeSet<Entry>,
    listeners: Vec<TcpStream>,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            entries: BTreeSet::new(),
            listeners: Vec::new(),
        }
    }

    pub fn add_item(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub fn has_item(&self, entry: &Entry) -> bool {
        self.entries.contains(entry)
    }

    pub fn add_listener(&mut self, listener: TcpStream) {
        self.listeners.push(listener);
    }
}
