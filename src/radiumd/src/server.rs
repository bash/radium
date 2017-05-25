use super::connection::Connection;
use std::io;
use mio::{Token, Events, Event, Poll, PollOpt, Ready};
use mio::unix::UnixReady;
use mio::tcp::TcpListener;

#[allow(deprecated)]
use mio::channel::Receiver;

use slab::Slab;
use radium_protocol::{Message, ReadValue, WriteValue, WatchMode};
use radium_protocol::messages::EntryExpired;
use libradium::{Entry, Frontend};
use super::pool::Pool;
use super::entry::EntryData;

pub const RECEIVER: Token = Token(10_000_001);
pub const SERVER: Token = Token(10_000_000);

pub struct Server {
    events: Events,
    poll: Poll,
    tcp: TcpListener,
    receiver: Receiver<Entry<EntryData>>,
    frontend: Frontend<EntryData>,
    pool: Pool,
}

impl Server {
    pub fn new(tcp: TcpListener, receiver: Receiver<Entry<EntryData>>, frontend: Frontend<EntryData>) -> io::Result<Self> {
        let poll = Poll::new()?;
        let pool = Pool::new(1);

        poll.register(&tcp, SERVER, Ready::readable(), PollOpt::edge())?;
        poll.register(&receiver, RECEIVER, Ready::readable(), PollOpt::edge())?;

        Ok(Server {
            events: Events::with_capacity(1024),
            poll: poll,
            tcp,
            receiver,
            frontend,
            pool,
        })
    }

    pub fn poll(&mut self) -> io::Result<()> {
        self.poll.poll(&mut self.events, None)?;

        for i in 0..self.events.len() {
            let event = self.events.get(i).unwrap();
            self.handle_event(event.token());
        }

        Ok(())
    }

    fn handle_event(&mut self, token: Token) {
        match token {
            SERVER => self.accept(),
            RECEIVER => self.pool.push_expired(
                self.receiver.try_recv().unwrap()
            ),
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

        self.pool.register(Connection::new(stream));
    }
}