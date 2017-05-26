extern crate libradium;
#[macro_use]
extern crate log;
extern crate mio;
extern crate mio_channel;
extern crate radium_protocol;
extern crate slab;

mod actions;
mod connection;
mod server;
mod logger;
mod pool;
mod entry;
mod worker;

use libradium::{Frontend, Listener, Timestamp};
use logger::Logger;
use mio_channel::{channel, Sender};
use mio::tcp::TcpListener;
use pool::Pool;

use self::server::Server;
use self::entry::{Entry, EntryData};

macro_rules! sock_addr {
    ($a:expr, $b:expr, $c:expr, $d:expr, $port:expr) => {
        ::std::net::SocketAddr::new(::std::net::IpAddr::V4(::std::net::Ipv4Addr::new($a, $b, $c, $d)), $port);
    }
}

struct EntryListener {
    sender: Sender<Entry>
}

impl Listener<EntryData> for EntryListener {
    fn on_expired(&self, entry: Entry) {
        self.sender.send(entry).unwrap();
    }
}

fn main() {
    let addr = sock_addr!(127, 0, 0, 1, 3126);
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