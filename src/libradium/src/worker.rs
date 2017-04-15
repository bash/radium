use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use super::entry::Entry;
use super::storage::Storage;

///
/// Minimum duration between expiration checks in seconds
///
const CHECK_INTERVAL: u64 = 1;

///
/// Receive timeout for incoming messages in milliseconds
///
const RECV_TIMEOUT: u64 = 500;

pub trait Listener: Send {
    fn on_expired(&self, entry: Entry);
    fn on_tick(&self) {}
}

#[derive(Debug)]
pub enum Command {
    AddEntry(Entry),
    RemoveEntry(Entry),
}

pub struct Worker {
    storage: Storage,
    receiver: Receiver<Command>,
    listener: Box<Listener>,
    last_checked: Option<Instant>,
}

pub fn spawn(storage: Storage,
             receiver: Receiver<Command>,
             listener: Box<Listener>)
             -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let worker = Worker::new(storage, receiver, listener);

        worker.run();
    })
}

impl Worker {
    pub fn new(storage: Storage, receiver: Receiver<Command>, listener: Box<Listener>) -> Worker {
        Worker {
            storage,
            receiver,
            listener,
            last_checked: None,
        }
    }

    pub fn run(mut self) -> thread::JoinHandle<()> {
        self.check_expired();

        loop {
            let incoming = self.receiver
                .recv_timeout(Duration::from_millis(RECV_TIMEOUT));

            match incoming {
                Err(err) => self.handle_error(err),
                Ok(command) => self.handle_command(command),
            }

            if self.needs_checking() {
                self.listener.on_tick();
                self.check_expired();
            }
        }
    }

    fn handle_error(&self, err: RecvTimeoutError) {
        match err {
            RecvTimeoutError::Timeout => {}
            RecvTimeoutError::Disconnected => panic!("channel disconnected"),
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::AddEntry(entry) => {
                self.storage.add_entry(entry);
            }
            Command::RemoveEntry(entry) => {
                self.storage.remove_entry(&entry);
            }
        }
    }

    fn check_expired(&mut self) {
        self.last_checked = Some(Instant::now());

        for entry in self.storage.expired_entries() {
            self.listener.on_expired(entry);
            self.storage.remove_entry(&entry);
        }
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
