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
use super::tokens::{SERVER, RECEIVER};

pub type EntryData = Vec<u8>;

pub struct Server {
    events: Events,
    poll: Poll,
    tcp: TcpListener,
    receiver: Receiver<Entry<EntryData>>,
    connections: Slab<Connection, Token>,
    frontend: Frontend<EntryData>,
}

impl Server {
    pub fn new(tcp: TcpListener, receiver: Receiver<Entry<EntryData>>, frontend: Frontend<EntryData>) -> io::Result<Self> {
        let poll = Poll::new()?;

        poll.register(&tcp, SERVER, Ready::readable(), PollOpt::edge())?;
        poll.register(&receiver, RECEIVER, Ready::readable(), PollOpt::edge())?;

        Ok(Server {
            events: Events::with_capacity(1024),
            poll: poll,
            tcp,
            receiver,
            connections: Slab::with_capacity(128),
            frontend,
        })
    }

    pub fn poll(&mut self) -> io::Result<()> {
        self.poll.poll(&mut self.events, None)?;

        for i in 0..self.events.len() {
            let event = self.events.get(i).unwrap();
            self.handle_event(event);
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        let token = event.token();

        if token == SERVER {
            return self.accept();
        }

        if token == RECEIVER {
            return self.notify();
        }

        let ready = event.readiness();

        if UnixReady::from(ready).is_hup() {
            return self.disconnect(token);
        }

        match self.connections.get(token) {
            Some(..) => self.message(token),
            None => warn!("Unknown token {:?}", token),
        };
    }

    fn message(&mut self, token: Token) {
        let conn = self.connections.get_mut(token).unwrap();

        let msg = {
            match conn.read_value::<Message>() {
                Ok(msg) => msg,
                Err(err) => {
                    // TODO: we need to check if this triggers a "HUP" event
                    conn.close();
                    error!("{:?}", err);
                    return;
                }
            }
        };

        let msg_type = msg.message_type();

        info!("Processing connection {:?}", token);
        info!("Received {:?}", msg_type);

        // TODO: do something with result
        match msg {
            Message::Ping => {
                conn.write_value(&Message::Pong);
            }
            Message::SetWatchMode(ref msg) => {
                conn.set_watch_mode(msg.mode());
                conn.write_value(&Message::WatchModeSet);
            },
            _ => {}
        };
    }

    fn disconnect(&mut self, token: Token) {
        self.connections.remove(token);
    }

    fn accept(&mut self) {
        let (stream, ..) = self.tcp.accept().unwrap();

        let token = {
            let entry = self.connections.vacant_entry().unwrap();

            entry.insert(Connection::new(stream)).index()
        };

        let connection = self.connections.get(token).unwrap();

        self.poll.register(connection, token, Ready::readable(), PollOpt::edge())
            .unwrap();
    }

    fn notify(&mut self) {
        let entry = self.receiver.try_recv().unwrap();

        debug!("Entry expired: {:?}", entry.id().timestamp());

        let inner = EntryExpired::new(entry.id().timestamp(), entry.id().id(), entry.consume_data());
        let msg = Message::EntryExpired(inner);

        for conn in &mut self.connections {
            if conn.watch_mode() == WatchMode::None {
                continue;
            }

            conn.write_value(&msg);
        }
    }
}