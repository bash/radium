use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use super::entry::Entry;

pub type SharedBackend = Arc<Mutex<Backend>>;

#[derive(Debug)]
pub struct Backend {
    items: BTreeSet<Entry>,
    listeners: Vec<TcpStream>
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            items: BTreeSet::new(),
            listeners: Vec::new()
        }
    }

    pub fn add_item(&mut self, entry: Entry) {
        self.items.insert(entry);
    }

    pub fn add_listener(&mut self, listener: TcpStream) {
        self.listeners.push(listener);
    }

    pub fn items(&self) -> &BTreeSet<Entry> {
        &self.items
    }
}