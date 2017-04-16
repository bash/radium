extern crate libradium;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};

use libradium::entry::{Entry, Timestamp};
use libradium::worker::Listener;
use libradium::frontend::Frontend;

struct Data {
    age: u16
}

impl Data {
    pub fn new(age: u16) -> Self {
        Data { age }
    }

    pub fn age(&self) -> u16 {
        self.age
    }
}

struct TestListener {
    tx: Sender<Output>,
}

enum Output {
    Expired(Entry<Data>),
    Tick,
}

impl Listener<Data> for TestListener {
    fn on_expired(&self, entry: Entry<Data>) {
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

    frontend.add_entry(Entry::gen(now, Data::new(10))).unwrap();
    frontend.add_entry(Entry::gen(now + 2, Data::new(20))).unwrap();
    frontend.add_entry(Entry::gen(now + 4, Data::new(30))).unwrap();
    frontend.add_entry(Entry::gen(now + 6, Data::new(40))).unwrap();
    frontend.add_entry(Entry::gen(now + 6, Data::new(50))).unwrap();
    frontend.add_entry(Entry::gen(now + 6, Data::new(60))).unwrap();
    frontend.add_entry(Entry::gen(now + 6, Data::new(70))).unwrap();
    frontend.add_entry(Entry::gen(now + 10, Data::new(80))).unwrap();

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entry) => {
                print!("({:?}, {})", entry.timestamp().sec, entry.data().age());
                io::stdout().flush().unwrap();
            }
            Output::Tick => {
                print!(".");
            }
        };
    }
}
