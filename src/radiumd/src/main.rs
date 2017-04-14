extern crate libradium;
extern crate time;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};
use time::precise_time_ns;

use libradium::storage::Storage;
use libradium::entry::Entry;
use libradium::worker::Worker;
use libradium::command::{Command, Listener};

#[derive(Debug)]
struct TestListener {
    tx: Sender<Output>
}

enum Output {
    Expired(Entry),
    Tick
}

impl Listener for TestListener {
    fn on_expired(&self, entry: Entry) {
        self.tx.send(Output::Expired(entry)).unwrap();
    }

    fn on_tick(&self) {
        self.tx.send(Output::Tick).unwrap();
    }
}

fn main() {
    let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
    let (tx_listener, rx_listener): (Sender<Output>, Receiver<Output>) = mpsc::channel();
    let mut storage = Storage::new();

    let now = precise_time_ns();

    storage.add_entry(Entry::generate(now));
    storage.add_entry(Entry::generate(now + 1000000000 * 2));
    storage.add_entry(Entry::generate(now + 1000000000 * 4));
    storage.add_entry(Entry::generate(now + 1000000000 * 6));
    storage.add_entry(Entry::generate(now + 1000000000 * 6));
    storage.add_entry(Entry::generate(now + 1000000000 * 6));
    storage.add_entry(Entry::generate(now + 1000000000 * 6));
    storage.add_entry(Entry::generate(now + 1000000000 * 6));
    storage.add_entry(Entry::generate(now + 1000000000 * 10));

    let worker = Worker::new(storage, rx, TestListener { tx: tx_listener });

    worker.spawn();

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entry) => {
                print!("({:?})", (entry.timestamp() as f64) / 1000000000.);
                io::stdout().flush().unwrap();
            },
            Output::Tick => {
                print!(".");
            }
        };
    }
}
