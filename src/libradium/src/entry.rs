use std::ops::Add;
use rand::{Rng, thread_rng};
use time::{Timespec, get_time};

/// A `Timestamp` holds a unix timestamp in seconds.
/// It is used to mark the expiration date of an [`Entry`].
///
/// [`Entry`]: struct.Entry.html
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct Timestamp {
    pub sec: i64,
}

impl Timestamp {
    pub fn new(sec: i64) -> Self {
        Timestamp { sec }
    }

    /// Returns a `Timestamp` at the current time
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// An `EntryId` consists of a `timestamp` at which the entry expires
/// and an `id`, which ensures the `EntryId` is unique.
///
/// The `EntryId` is sorted by timestamp and then by id, allowing the [`Storage`] to iterate
/// only through the first few entries when checking for expiration.
///
/// [`Storage`]: struct.Storage.html
pub struct EntryId {
    timestamp: Timestamp,
    id: u16,
}

#[derive(Debug)]
/// An `Entry` represents a single entry stored in [`Storage`].
/// It can hold arbitrary data in its `data` field.
///
/// [`Storage`]: struct.Storage.html
pub struct Entry<T: Send + 'static> {
    id: EntryId,
    data: T,
}

impl EntryId {
    pub fn new<TS: Into<Timestamp>>(timestamp: TS, id: u16) -> Self {
        EntryId {
            timestamp: timestamp.into(),
            id,
        }
    }

    /// Creates a new `EntryId` with a random value for `id`
    pub fn gen<TS: Into<Timestamp>>(timestamp: TS) -> Self {
        let mut rng = thread_rng();
        let id = rng.gen::<u16>();

        Self::new(timestamp, id)
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl<T: Send + 'static> Entry<T> {
    pub fn new(id: EntryId, data: T) -> Self {
        Entry { id, data }
    }

    /// Convenience method that generates an [`EntryId`].
    ///
    /// [`EntryId`]: struct.EntryId.html
    pub fn gen<TS: Into<Timestamp>>(timestamp: TS, data: T) -> Self {
        Self::new(EntryId::gen(timestamp), data)
    }

    /// Returns the [`EntryId`] associated with this `Entry`
    ///
    /// [`EntryId`]: struct.EntryId.html
    pub fn id(&self) -> EntryId {
        self.id
    }

    /// Gives a reference to the data that is stored in this `Entry`
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Returns the data, consuming the `Entry`
    pub fn consume_data(self) -> T {
        self.data
    }
}

impl<T: Send + 'static> Clone for Entry<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Entry {
            id: self.id,
            data: self.data.clone(),
        }
    }
}
