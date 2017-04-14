use std::net::{TcpListener, ToSocketAddrs, TcpStream};
use std::sync::mpsc::{Sender, Receiver};
use threadpool::ThreadPool;

struct ConnWorker {
    pool: ThreadPool
}

impl ConnWorker {
    pub fn new() -> ConnWorker {
        ConnWorker {
            pool
        }
    }

    pub fn bind<A: ToSocketAddrs>(&self, addr: A) -> Result<(), Error> {
        let listener = TcpListener::bind(&addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        return Ok(());
    }

    fn handle_connection(&self, stream: TcpStream) {

    }
}