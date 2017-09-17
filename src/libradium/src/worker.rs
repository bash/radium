use std::thread;
use std::time::Instant;
use std::time::Duration;
use super::entry::{Entry, EntryId};
use super::storage::Storage;
use super::sync::Receiver;

///
/// Minimum duration between expiration checks in seconds
///
const CHECK_INTERVAL: u64 = 1;

///
/// Duration of sleep between loop turns (in milliseconds)
///
const SLEEP_DURATION: u64 = 100;

pub trait Listener<T: Send + 'static>: Send {
    fn on_expired(&self, entry: Vec<Entry<T>>);
}

#[derive(Debug)]
pub enum Command<T: Send + 'static> {
    AddEntry(Entry<T>),
    RemoveEntry(EntryId),
}

#[derive(Debug)]
enum Action {
    ReceiveCommand,
    CheckExpired,
    Sleep,
}

pub struct Worker<T: Send + 'static> {
    storage: Storage<T>,
    receiver: Receiver<Command<T>>,
    listener: Box<Listener<T>>,
    last_checked: Option<Instant>,
}

pub fn spawn_worker<T: Send + 'static>(
    storage: Storage<T>,
    receiver: Receiver<Command<T>>,
    listener: Box<Listener<T>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let worker = Worker::new(storage, receiver, listener);

        worker.run();
    })
}

impl<T: Send + 'static> Worker<T> {
    pub fn new(
        storage: Storage<T>,
        receiver: Receiver<Command<T>>,
        listener: Box<Listener<T>>,
    ) -> Worker<T> {
        Worker {
            storage,
            receiver,
            listener,
            last_checked: None,
        }
    }

    pub fn run(mut self) {
        let sleep_dur = Duration::from_millis(SLEEP_DURATION);

        loop {
            match self.determine_action() {
                Action::ReceiveCommand => self.handle_incoming(),
                Action::CheckExpired => self.check_expired(),
                Action::Sleep => thread::sleep(sleep_dur),
            }
        }
    }

    fn determine_action(&self) -> Action {
        if self.receiver.has_incoming() {
            return Action::ReceiveCommand;
        }

        if self.needs_checking() {
            return Action::CheckExpired;
        }

        Action::Sleep
    }

    fn handle_incoming(&mut self) {
        let incoming = self.receiver.recv();

        match incoming {
            Err(_) => panic!("channel disconnected"),
            Ok(command) => self.handle_command(command),
        }
    }

    fn handle_command(&mut self, command: Command<T>) {
        match command {
            Command::AddEntry(entry) => self.storage.add_entry(entry),
            Command::RemoveEntry(id) => self.storage.remove_entry(id),
        }
    }

    fn check_expired(&mut self) {
        self.last_checked = Some(Instant::now());
        self.listener.on_expired(self.storage.expire_entries());
    }

    fn needs_checking(&self) -> bool {
        match self.last_checked {
            None => true,
            Some(value) => {
                return value.elapsed().as_secs() >= CHECK_INTERVAL;
            }
        }
    }
}
