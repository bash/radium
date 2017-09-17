use super::entry::{Entry, EntryId};

#[derive(Debug)]
pub enum Command<T: Send + 'static> {
    AddEntry(Entry<T>),
    RemoveEntry(EntryId),
}
