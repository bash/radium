extern crate libradium;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};

use libradium::{Entry, Timestamp, Listener, Frontend};

struct User {
    age: u16,
}

impl User {
    pub fn new(age: u16) -> Self {
        User { age }
    }

    pub fn age(&self) -> u16 {
        self.age
    }
}

struct TestListener {
    tx: Sender<Output>,
}

enum Output {
    Expired(Entry<User>),
    Tick,
}

impl Listener<User> for TestListener {
    fn on_expired(&self, entry: Entry<User>) {
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

    frontend
        .add_entry(Entry::gen(now, User::new(10)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 2, User::new(20)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 4, User::new(30)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 6, User::new(40)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 6, User::new(50)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 6, User::new(60)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 6, User::new(70)))
        .unwrap();

    frontend
        .add_entry(Entry::gen(now + 10, User::new(80)))
        .unwrap();

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entry) => {
                print!("({:?}, {})", entry.id().timestamp().sec, entry.data().age());
                io::stdout().flush().unwrap();
            }
            Output::Tick => {
                print!(".");
            }
        };
    }
}
