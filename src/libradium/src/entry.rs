#![feature(ordering_chaining)]

extern crate rand;

use rand::Rng;
use std::cmp::{Ord, Ordering};

#[derive(Eq, PartialEq, Debug)]
pub struct EntryId {
    timestamp: i64,
    random: u16
}

impl EntryId {
    pub fn new(timestamp: i64) -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen::<u16>();

        EntryId {
            timestamp: timestamp,
            random: random
        }
    }
}

impl Ord for EntryId {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.timestamp == other.timestamp {
            return self.random.cmp(&other.random);
        }

        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for EntryId {
    fn partial_cmp(&self, other: &EntryId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Entry {
    id: EntryId
}

impl Entry {
    pub fn new(id: EntryId) -> Self {
        return Entry {
            id: id
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

