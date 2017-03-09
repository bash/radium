extern crate rand;

use rand::Rng;
use std::cmp::{Ord, Ordering};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Entry {
    timestamp: u64,
    id: u16,
}

impl Entry {
    pub fn generate(timestamp: u64) -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen::<u16>();

        Entry {
            timestamp: timestamp,
            id: random,
        }
    }

    pub fn new(timestamp: u64, id: u16) -> Self {
        Entry {
            timestamp: timestamp,
            id: id,
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id
            .cmp(&other.id)
            .then(self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
