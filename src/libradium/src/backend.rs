use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::iter::Iterator;
use super::entry::Entry;
use time::now_utc;

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

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub fn has_entry(&self, entry: &Entry) -> bool {
        self.entries.contains(entry)
    }

    pub fn add_listener(&mut self, listener: TcpStream) {
        self.listeners.push(listener);
    }

    pub fn expired_entries(&self) -> ExpiredItems {
        ExpiredItems { iter: self.entries.iter(), timestamp: now_utc().to_timespec().sec }
    }
}

pub struct ExpiredItems<'a> {
    iter: Iter<'a, Entry>,
    timestamp: i64
}

impl<'a> Iterator for ExpiredItems<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<&'a Entry> {
        let entry = maybe!(self.iter.next());

        if self.timestamp < (entry.timestamp() as i64) {
            return None
        }

        Some(entry)
    }
}