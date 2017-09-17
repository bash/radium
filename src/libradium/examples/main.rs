extern crate libradium;

use std::sync::mpsc;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::{Sender, Receiver};

use libradium::{Entry, Timestamp, Core};

type Data = ();
type Entries<T> = Vec<Entry<T>>;

struct Listener {
    tx: Sender<Entries<Data>>,
}

impl libradium::Listener<Data> for Listener {
    fn on_expired(&self, entries: Entries<Data>) {
        self.tx.send(entries).unwrap();
    }
}

fn main() {
    let (tx_listener, rx_listener): (Sender<Entries<Data>>, Receiver<Entries<Data>>) =
        mpsc::channel();
    let listener = Listener { tx: tx_listener };
    let core = Core::spawn(listener);

    let now = Timestamp::now();

    for i in 0..100 {
        core.add_entry(Entry::gen(now + (i * 10), ())).unwrap();

        core
            .add_entry(Entry::gen(now + (i * 10) + 1, ()))
            .unwrap();
    }

    loop {
        let entries = rx_listener.recv().unwrap();
        let len = entries.len();

        for entry in entries {
            print!("( {:?} )", entry.id().timestamp().sec);
        }

        if len > 0 {
            println!();
        }

        let _ = io::stdout().flush();
    }
}
