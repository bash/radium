use std::net::{TcpListener, ToSocketAddrs};
use std::io::Error;

pub struct Server {
    // handler: ConnectionHandler,
}

impl Server {
    /*pub fn new(handler: ConnectionHandler) -> Self {
        Server { handler: worker }
    }*/

    pub fn bind<A: ToSocketAddrs>(&mut self, addr: A) -> Result<(), Error> {
        let listener = TcpListener::bind(&addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // self.handler.handle_conn(stream);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        return Ok(());
    }
}
