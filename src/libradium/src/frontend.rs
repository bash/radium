use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, SendError};
use std::thread;

use super::storage::Storage;
use super::entry::Entry;
use super::worker;
use super::command::{Command, Listener};

pub type CommandResult = Result<(), SendError<Command>>;

pub struct Frontend {
    tx: Sender<Command>,
}

impl Frontend {
    pub fn new(tx: Sender<Command>) -> Frontend {
        Frontend { tx }
    }

    pub fn build(listener: Box<Listener>) -> (Frontend, thread::JoinHandle<()>) {
        let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let storage = Storage::new();
        let handle = worker::spawn(storage, rx, listener);

        (Self::new(tx), handle)
    }

    pub fn add_entry(&self, entry: Entry) -> CommandResult {
        self.command(Command::AddEntry(entry))
    }

    pub fn remove_entry(&self, entry: Entry) -> CommandResult {
        self.command(Command::RemoveEntry(entry))
    }

    fn command(&self, command: Command) -> CommandResult {
        self.tx.send(command)
    }
}
