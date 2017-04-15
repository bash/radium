use super::entry::Entry;

pub trait Listener: Send {
    fn on_expired(&self, entry: Entry);
    fn on_tick(&self) {}
}

#[derive(Debug)]
pub enum Command {
    AddEntry(Entry),
    RemoveEntry(Entry)
}