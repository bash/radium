use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use super::entry::{Entry, EntryId};
use super::storage::Storage;

///
/// Minimum duration between expiration checks in seconds
///
const CHECK_INTERVAL: u64 = 1;

///
/// Receive timeout for incoming messages in milliseconds
///
const RECV_TIMEOUT: u64 = 500;

pub trait Listener<T: Send + 'static>: Send {
    fn on_expired(&self, entry: Entry<T>);
    fn on_tick(&self) {}
}

#[derive(Debug)]
pub enum Command<T: Send + 'static> {
    AddEntry(Entry<T>),
    RemoveEntry(EntryId),
}

pub struct Worker<T: Send + 'static> {
    storage: Storage<T>,
    receiver: Receiver<Command<T>>,
    listener: Box<Listener<T>>,
    last_checked: Option<Instant>,
}

pub fn spawn_worker<T: Send + 'static>(storage: Storage<T>,
                                       receiver: Receiver<Command<T>>,
                                       listener: Box<Listener<T>>)
                                       -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let worker = Worker::new(storage, receiver, listener);

        worker.run();
    })
}

impl<T: Send + 'static> Worker<T> {
    pub fn new(storage: Storage<T>,
               receiver: Receiver<Command<T>>,
               listener: Box<Listener<T>>)
               -> Worker<T> {
        Worker {
            storage,
            receiver,
            listener,
            last_checked: None,
        }
    }

    pub fn run(mut self) {
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

    fn handle_command(&mut self, command: Command<T>) {
        match command {
            Command::AddEntry(entry) => {
                self.storage.add_entry(entry);
            }
            Command::RemoveEntry(id) => {
                self.storage.remove_entry(id);
            }
        }
    }

    fn check_expired(&mut self) {
        self.last_checked = Some(Instant::now());

        for entry in self.storage.expire_entries() {
            self.listener.on_expired(entry);
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
