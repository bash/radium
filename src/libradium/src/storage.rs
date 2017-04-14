use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::iter::Iterator;
use super::entry::Entry;
use time::precise_time_ns;

#[derive(Debug)]
pub struct Storage {
    entries: BTreeSet<Entry>
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            entries: BTreeSet::new()
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub fn has_entry(&self, entry: &Entry) -> bool {
        self.entries.contains(entry)
    }

    pub fn remove_entry(&mut self, entry: &Entry) {
        self.entries.remove(entry);
    }

    pub fn expired_entries(&self) -> Vec<Entry> {
        let iter = ExpiredItems { iter: self.entries.iter(), timestamp: precise_time_ns() };
        let mut entries = Vec::<Entry>::new();

        for entry in iter {
            entries.push(entry);
        }

        return entries;
    }
}

pub struct ExpiredItems<'a> {
    iter: Iter<'a, Entry>,
    timestamp: u64,
}

impl<'a> Iterator for ExpiredItems<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Entry> {
        let entry = maybe!(self.iter.next());

        if self.timestamp <= entry.timestamp() {
            return None;
        }

        Some(*entry)
    }
}