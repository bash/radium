extern crate libradium;
#[macro_use]
extern crate log;
extern crate radium_protocol;
extern crate slab;

#[macro_use]
mod macros;
mod logger;
mod entry;

use libradium::{Core, Listener};
use logger::Logger;
use std::env;
use std::net::SocketAddr;
use std::sync::mpsc::{Sender, channel};

use self::entry::{Entry, EntryData};

struct EntryListener {
    sender: Sender<Vec<Entry>>
}

impl Listener<EntryData> for EntryListener {
    fn on_expired(&self, entry: Vec<Entry>) {
        self.sender.send(entry).unwrap();
    }
}

impl EntryListener {
    pub fn new(sender: Sender<Vec<Entry>>) -> Self {
        EntryListener { sender }
    }
}

fn main() {
    let (sender, receiver) = channel();
    let core = Core::spawn(EntryListener::new(sender));

    Logger::init().unwrap();

    // TODO
}