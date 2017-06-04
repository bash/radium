use super::connection::Connection;
use std::io;
use mio::{Token, Events, Poll, PollOpt, Ready};
use mio::tcp::TcpListener;
use mio_channel::Receiver;

use super::pool::Pool;
use super::entry::Entry;

pub const RECEIVER: Token = Token(10_000_001);
pub const SERVER: Token = Token(10_000_000);

pub struct Server {
    events: Events,
    poll: Poll,
    tcp: TcpListener,
    receiver: Receiver<Vec<Entry>>,
    pool: Pool,
}

impl Server {
    pub fn new(tcp: TcpListener, receiver: Receiver<Vec<Entry>>, pool: Pool) -> io::Result<Self> {
        let poll = Poll::new()?;

        poll.register(&tcp, SERVER, Ready::readable(), PollOpt::edge())?;
        poll.register(&receiver, RECEIVER, Ready::readable(), PollOpt::edge())?;

        Ok(Server {
            events: Events::with_capacity(1024),
            poll: poll,
            tcp,
            receiver,
            pool,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.poll()?;
        }
    }

    fn poll(&mut self) -> io::Result<()> {
        self.poll.poll(&mut self.events, None)?;

        for i in 0..self.events.len() {
            let event = self.events.get(i).unwrap();
            self.handle_event(event.token());
        }

        Ok(())
    }

    fn handle_event(&mut self, token: Token) {
        // TODO: proper error handling
        match token {
            SERVER => self.accept(),
            RECEIVER => self.pool.push_expired(
                self.receiver.try_recv().unwrap()
            ).unwrap(),
            _ => {
                // TODO
            }
        };
    }

    fn accept(&mut self) {
        let (stream, ..) = {
            match self.tcp.accept() {
                Ok(val) => val,
                Err(..) => {
                    // TODO: log more details
                    error!("Unable to accept client");
                    return;
                }
            }
        };

        // TODO: proper error handling
        self.pool.register(Connection::new(stream)).unwrap();
    }
}