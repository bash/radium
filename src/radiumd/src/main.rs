mod radiumd;

extern crate libradium;

use radiumd::server::Server;
use radiumd::backend::Backend;
use libradium::entry::{Entry, EntryId};

fn main() {
    // let server = Server::bind(("localhost", 3126));

    let mut backend = Backend::new();

    backend.add(Entry::new(EntryId::new(0)));
    backend.add(Entry::new(EntryId::new(0)));

    println!("{:?}", backend);
}