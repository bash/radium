use std::thread;
use std::error;
use std::fmt;
use std::sync::Arc;

use super::storage::Storage;
use super::entry::{Entry, EntryId};
use super::command::{Command};
use super::worker::{Listener, spawn_worker};
use super::sync::{channel, Sender, Receiver, SendError};

pub type CommandResult = Result<(), CommandError>;

#[derive(Debug)]
pub enum CommandError {
    SendError,
    #[doc(hidden)]
    __NonExhaustive,
}

impl<T> From<SendError<T>> for CommandError {
    fn from(_: SendError<T>) -> Self {
        CommandError::SendError
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CommandError::SendError => write!(f, "{}", "error sending command"),
            &CommandError::__NonExhaustive => unreachable!(),
        }
    }
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match self {
            &CommandError::SendError => "error sending command",
            &CommandError::__NonExhaustive => unreachable!(),
        }
    }
}

pub struct Core<T> where T: Send + 'static {
    tx: Sender<Command<T>>,
    join_handle: Arc<thread::JoinHandle<()>>,
}

impl<T> Clone for Core<T> where T: Send + 'static {
    fn clone(&self) -> Self {
        Core {
            tx: self.tx.clone(),
            join_handle: self.join_handle.clone(),
        }
    }
}

impl<T> Core<T> where T: Send + 'static {
    pub fn spawn<L>(listener: L) -> Self where L: Listener<T> + 'static {
        let (tx, rx): (Sender<Command<T>>, Receiver<Command<T>>) = channel();
        let storage = Storage::new();
        let join_handle = spawn_worker(storage, rx, Box::new(listener));

        Core {
            tx,
            join_handle: Arc::new(join_handle),
        }
    }

    pub fn add_entry(&self, entry: Entry<T>) -> CommandResult {
        self.command(Command::AddEntry(entry))
    }

    pub fn remove_entry(&self, id: EntryId) -> CommandResult {
        self.command(Command::RemoveEntry(id))
    }

    fn command(&self, command: Command<T>) -> CommandResult {
        Ok(self.tx.send(command)?)
    }
}
