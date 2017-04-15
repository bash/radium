extern crate libradium;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};

use libradium::entry::{Entry, Timestamp};
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

    let now = Timestamp::now();

    frontend.add_entry(Entry::gen(now)).unwrap();
    frontend.add_entry(Entry::gen(now + 2)).unwrap();
    frontend.add_entry(Entry::gen(now + 4)).unwrap();
    frontend.add_entry(Entry::gen(now + 6)).unwrap();
    frontend.add_entry(Entry::gen(now + 6)).unwrap();
    frontend.add_entry(Entry::gen(now + 6)).unwrap();
    frontend.add_entry(Entry::gen(now + 6)).unwrap();
    frontend.add_entry(Entry::gen(now + 10)).unwrap();

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entry) => {
                print!("({:?})", entry.timestamp().sec);
                io::stdout().flush().unwrap();
            }
            Output::Tick => {
                print!(".");
            }
        };
    }
}
