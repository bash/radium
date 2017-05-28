extern crate libradium;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate mio;
extern crate mio_channel;
extern crate radium_protocol;
extern crate slab;

#[macro_use]
mod macros;
mod actions;
mod connection;
mod server;
mod logger;
mod pool;
mod entry;
mod worker;

use getopts::Options;
use libradium::{Frontend, Listener, Timestamp};
use logger::Logger;
use mio_channel::{channel, Sender};
use mio::tcp::TcpListener;
use pool::Pool;
use std::env;
use std::net::SocketAddr;

use self::server::Server;
use self::entry::{Entry, EntryData};

struct EntryListener {
    sender: Sender<Entry>
}

impl Listener<EntryData> for EntryListener {
    fn on_expired(&self, entry: Entry) {
        self.sender.send(entry).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("H", "host", "sets the host to listen on", "HOST");
    opts.optopt("P", "port", "set port to listen on", "PORT");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    let host = match matches.opt_str("H") {
        Some(val) => { val }
        None => "127.0.0.1".to_string()
    };

    let port = match matches.opt_str("P") {
        Some(val) => { val }
        None => "3126".to_string()
    };

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();

    println!("Listening on {:?}", addr);

    let tcp = TcpListener::bind(&addr).unwrap();

    // TODO: cli flags --host, --port, --verbose

    let (sender, receiver) = channel();
    let (frontend, _) = Frontend::build(Box::new(EntryListener { sender }));

    Logger::init().unwrap();

    frontend
        .add_entry(Entry::gen(Timestamp::now() + 10, vec![1, 2, 3]))
        .unwrap();

    frontend
        .add_entry(Entry::gen(Timestamp::now() + 13, vec![20, 30, 40, 7]))
        .unwrap();

    let pool = Pool::build(frontend, 4);
    let mut server: Server = Server::new(tcp, receiver, pool).unwrap();

    server.run().unwrap();
}