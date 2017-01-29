use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{Error};

pub struct Server<A: ToSocketAddrs> {
    addr: A
}

impl <A: ToSocketAddrs> Server<A> {
    pub fn new (addr: A) -> Self {
        Server {
            addr: addr
        }
    }

    pub fn bind (addr: A) -> Result<(), Error> {
        Server::new(addr).listen()
    }

    pub fn listen (&self) -> Result<(), Error> {
        let listener = try!(TcpListener::bind(&self.addr));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("{:?}", stream);
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        return Ok(());
    }
}