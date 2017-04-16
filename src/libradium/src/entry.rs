extern crate rand;

use std::ops::Add;
use rand::Rng;
use time::{Timespec, get_time};

///
/// This could be using u64 but is i64 to be compatible with the [`time`] crate
///
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct Timestamp {
    pub sec: i64,
}

impl Timestamp {
    pub fn new(sec: i64) -> Self {
        Timestamp { sec }
    }

    pub fn now() -> Self {
        Self::new(get_time().sec)
    }
}

impl From<Timespec> for Timestamp {
    fn from(value: Timespec) -> Self {
        Self::new(value.sec)
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Into<i64> for Timestamp {
    fn into(self) -> i64 {
        self.sec
    }
}

impl Add<i64> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: i64) -> Self::Output {
        Timestamp::new(self.sec + rhs)
    }
}

#[derive(Debug)]
pub struct Entry<T: Send + 'static> {
    timestamp: Timestamp,
    id: u16,
    data: T
}

impl<T: Send + 'static> Entry<T> {
    pub fn new<TS: Into<Timestamp>>(timestamp: TS, id: u16, data: T) -> Self {
        Entry {
            timestamp: timestamp.into(),
            id,
            data
        }
    }

    pub fn gen<TS: Into<Timestamp>>(timestamp: TS, data: T) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u16>();

        Self::new(timestamp, id, data)
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}