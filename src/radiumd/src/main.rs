extern crate libradium;

use libradium::server::Server;
use libradium::backend::Backend;
use libradium::worker::Worker;

fn main() {
    let backend = Backend::new();
    let worker = Worker::new(backend);
    let mut server = Server::new(worker);

    server.bind(("127.0.0.1", 3126)).unwrap();
}