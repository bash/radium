extern crate libradium;
extern crate time;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};
use time::precise_time_ns;

use libradium::entry::Entry;
use libradium::command::Listener;
use libradium::frontend::Frontend;

struct TestListener {
    tx: Sender<Output>,
}

enum Output {
    Expired(Entry),
    Tick,
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
    let (tx_listener, rx_listener): (Sender<Output>, Receiver<Output>) = mpsc::channel();
    let listener = Box::new(TestListener { tx: tx_listener });
    let (frontend, _) = Frontend::build(listener);

    let now = precise_time_ns();

    frontend.add_entry(Entry::gen(now)).unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 2))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 4))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 6))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 6))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 6))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 6))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 1000000000 * 10))
        .unwrap();

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entry) => {
                print!("({:?})", (entry.timestamp() as f64) / 1000000000.);
                io::stdout().flush().unwrap();
            }
            Output::Tick => {
                print!(".");
            }
        };
    }
}
