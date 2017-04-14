use std::fmt::Debug;
use super::entry::Entry;

pub trait Listener: Debug + Send {
    fn on_expired(&self, entry: Entry);
    fn on_tick(&self) {}
}

#[derive(Debug)]
pub enum Command {
    AddListener(Box<Listener>),
    AddEntry(Entry),
    RemoveEntry(Entry)
}