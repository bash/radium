use std::net::{TcpListener, ToSocketAddrs};
use std::io::Error;
use worker::Worker;

pub struct Server {
    worker: Worker,
}

impl Server {
    pub fn new(worker: Worker) -> Self {
        Server { worker: worker }
    }

    pub fn bind<A: ToSocketAddrs>(&mut self, addr: A) -> Result<(), Error> {
        let listener = TcpListener::bind(&addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.worker.handle_client(stream);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        return Ok(());
    }
}
