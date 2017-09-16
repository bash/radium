extern crate libradium;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};

use libradium::{Entry, Timestamp, Listener, Frontend};

#[derive(Debug)]
struct User {
    age: i64,
}

impl User {
    pub fn new(age: i64) -> Self {
        User { age }
    }

    pub fn age(&self) -> i64 {
        self.age
    }
}

struct TestListener {
    tx: Sender<Output>,
}

enum Output {
    Expired(Vec<Entry<User>>),
    #[cfg(feature = "with-ticks")]
    Tick,
}

impl Listener<User> for TestListener {
    fn on_expired(&self, entries: Vec<Entry<User>>) {
        self.tx.send(Output::Expired(entries)).unwrap();
    }

    #[cfg(feature = "with-ticks")]
    fn on_tick(&self) {
        self.tx.send(Output::Tick).unwrap();
    }
}

fn main() {
    let (tx_listener, rx_listener): (Sender<Output>, Receiver<Output>) = mpsc::channel();
    let listener = Box::new(TestListener { tx: tx_listener });
    let (frontend, _) = Frontend::build(listener);

    let now = Timestamp::now();

   /* frontend
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
        .unwrap();*/

    for i in 0..100 {
        frontend
            .add_entry(Entry::gen(now + (i * 10), User::new(i)))
            .unwrap();

        frontend
            .add_entry(Entry::gen(now + (i * 10) + 1, User::new(i + 100)))
            .unwrap();
    }

    loop {
        match rx_listener.recv().unwrap() {
            Output::Expired(entries) => {
                for entry in entries {
                    print!("({:?}, {})", entry.id().timestamp().sec, entry.data().age());
                }

                // frontend.add_entry(Entry::gen(Timestamp::now() + 20, User::new(Timestamp::now().sec)));

                io::stdout().flush().unwrap();
            }
            #[cfg(feature = "with-ticks")]
            Output::Tick => {
                print!(".");
            }
        };
    }
}
