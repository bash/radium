extern crate mio;
extern crate slab;
#[macro_use]
extern crate log;
extern crate libradium;
extern crate radium_protocol;

mod connection;
mod server;
mod logger;
mod pool;
mod entry;

use std::io::Write;
use mio::tcp::TcpListener;
use libradium::{Frontend, Entry, Listener, Timestamp};
use logger::Logger;
use pool::Pool;

#[allow(deprecated)]
use mio::channel::{channel, Sender};
use self::server::Server;
use self::entry::EntryData;

macro_rules! sock_addr {
    ($a:expr, $b:expr, $c:expr, $d:expr, $port:expr) => {
        ::std::net::SocketAddr::new(::std::net::IpAddr::V4(::std::net::Ipv4Addr::new($a, $b, $c, $d)), $port);
    }
}

struct EntryListener {
    sender: Sender<Entry<EntryData>>
}


impl Listener<EntryData> for EntryListener {
    fn on_expired(&self, entry: Entry<EntryData>) {
        // TODO: should we unwrap here?
        self.sender.send(entry).unwrap()
    }
}

fn main() {
    let addr = sock_addr!(127, 0, 0, 1, 3126);
    let tcp = TcpListener::bind(&addr).unwrap();

    let (sender, receiver) = channel();
    let (frontend, _) = Frontend::build(Box::new(EntryListener { sender }));

    Logger::init();

    frontend
        .add_entry(Entry::gen(Timestamp::now() + 5, vec![1, 2, 3]))
        .unwrap();

    frontend
        .add_entry(Entry::gen(Timestamp::now() + 6, vec![20, 30, 40, 7]))
        .unwrap();

    // let mut server = Server::new(tcp, frontend, receiver);
    // server.run();

    let mut server = Server::new(tcp, receiver, frontend).unwrap();

    loop {
        server.poll();
    }
}