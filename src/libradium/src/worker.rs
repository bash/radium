use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;
use time::precise_time_ns;

use super::command::{Command, Listener};
use super::storage::Storage;

pub struct Worker {
    storage: Storage,
    receiver: Receiver<Command>,
    listener: Box<Listener>,
    last_checked: Option<u64>,
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
            let incoming = self.receiver.recv_timeout(Duration::from_millis(500));

            self.listener.on_tick();

            if let Err(err) = incoming {
                match err {
                    RecvTimeoutError::Timeout => {}
                    RecvTimeoutError::Disconnected => panic!("channel disconnected"),
                }
            }

            if let Ok(command) = incoming {
                match command {
                    Command::AddEntry(entry) => {
                        self.storage.add_entry(entry);
                    }
                    Command::RemoveEntry(entry) => {
                        self.storage.remove_entry(&entry);
                    }
                }
            }

            if self.needs_checking() {
                self.check_expired();
            }
        }
    }

    fn check_expired(&mut self) {
        self.last_checked = Some(precise_time_ns());

        for entry in self.storage.expired_entries() {
            self.listener.on_expired(entry);
            self.storage.remove_entry(&entry);
        }
    }

    fn needs_checking(&self) -> bool {
        match self.last_checked {
            None => true,
            Some(value) => {
                // TODO: ooh no we have a magic number here
                return (precise_time_ns() - value) >= 1000000000;
            }
        }
    }
}
