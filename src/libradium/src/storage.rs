use std::collections::BTreeMap;
use super::entry::{Entry, Timestamp};

type EntryId = (Timestamp, u16);

#[derive(Debug)]
pub struct Storage<T: Send + 'static> {
    entries: BTreeMap<EntryId, Entry<T>>,
}

impl<T: Send + 'static> Storage<T> {
    pub fn new() -> Self {
        Storage { entries: BTreeMap::new() }
    }

    pub fn add_entry(&mut self, entry: Entry<T>) {
        self.entries.insert((entry.timestamp(), entry.id()), entry);
    }

    pub fn has_entry(&self, entry: &Entry<T>) -> bool {
        self.entries.contains_key(&(entry.timestamp(), entry.id()))
    }

    pub fn remove_entry(&mut self, entry: &Entry<T>) -> Option<Entry<T>> {
        self.entries.remove(&(entry.timestamp(), entry.id()))
    }

    fn remove_by_id(&mut self, id: &EntryId) -> Option<Entry<T>> {
        self.entries.remove(id)
    }

    pub fn expire_entries(&mut self) -> Vec<Entry<T>> {
        let mut entries = Vec::<Entry<T>>::new();
        let now = Timestamp::now();
        let mut expired = Vec::<EntryId>::new();

        // This is the best solution I came up with
        // but it still bugs me that we have to iterate twice
        for id in self.entries.keys() {
            if id.0 > now {
                break;
            }

            expired.push(*id);
        }

        for id in expired {
            // We can safely unwrap here because we know that
            // the item we're trying to remove actually exists
            entries.push(self.remove_by_id(&id).unwrap());
        }

        return entries;
    }
}