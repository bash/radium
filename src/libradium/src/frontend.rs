use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, SendError};
use std::thread;

use super::storage::Storage;
use super::entry::Entry;
use super::worker;
use super::worker::{Command, Listener};

pub type CommandResult<T> = Result<(), SendError<Command<T>>>;

pub struct Frontend<T: Send + 'static> {
    tx: Sender<Command<T>>,
}

impl<T: Send + 'static> Frontend<T> {
    pub fn new(tx: Sender<Command<T>>) -> Self {
        Frontend { tx }
    }

    pub fn build(listener: Box<Listener<T>>) -> (Self, thread::JoinHandle<()>) {
        let (tx, rx): (Sender<Command<T>>, Receiver<Command<T>>) = mpsc::channel();
        let storage = Storage::new();
        let handle = worker::spawn(storage, rx, listener);

        (Self::new(tx), handle)
    }

    pub fn add_entry(&self, entry: Entry<T>) -> CommandResult<T> {
        self.command(Command::AddEntry(entry))
    }

    pub fn remove_entry(&self, entry: Entry<T>) -> CommandResult<T> {
        self.command(Command::RemoveEntry(entry))
    }

    fn command(&self, command: Command<T>) -> CommandResult<T> {
        self.tx.send(command)
    }
}
