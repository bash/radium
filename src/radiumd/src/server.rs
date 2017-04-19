use super::connection::Connection;
use mio::{Token, Events, Event, Poll, PollOpt, Ready};
use mio::unix::UnixReady;
use mio::tcp::TcpListener;
use mio::channel::{channel, Sender, Receiver};
use slab::Slab;
use radium_protocol::{Message, ReadFrom, WriteTo, WatchMode};
use radium_protocol::messages::EntryExpired;

use libradium::{Entry, Timestamp, Listener, Frontend};

const RECEIVER: Token = Token(10_000_001);

pub type EntryData = Vec<u8>;

pub struct Server {
    listener: TcpListener,
    token: Token,
    connections: Slab<Connection, Token>,
    events: Events,
    poll: Poll,
    frontend: Frontend<EntryData>,
    receiver: Receiver<Entry<EntryData>>,
}

impl Server {
    pub fn new(listener: TcpListener, frontend: Frontend<EntryData>, receiver: Receiver<Entry<EntryData>>) -> Self {
        Server {
            listener,
            token: Token(10_000_000),
            connections: Slab::with_capacity(128),
            events: Events::with_capacity(1024),
            poll: Poll::new().unwrap(),
            frontend,
            receiver,
        }
    }

    pub fn run(&mut self) {
        self.poll
            .register(&self.listener, self.token, Ready::readable(), PollOpt::edge())
            .unwrap();

        self.poll
            .register(&self.receiver, RECEIVER, Ready::readable(), PollOpt::edge())
            .unwrap();

        loop {
            let cnt = self.poll.poll(&mut self.events, None).unwrap();
            let mut i = 0;

            while i < cnt {
                let event = self.events.get(i).unwrap();

                self.handle_event(event);

                i += 1;
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        let token = event.token();

        if token == self.token {
            return self.accept();
        }

        if token == RECEIVER {
            return self.notify();
        }

        let ready = event.readiness();

        if UnixReady::from(ready).is_hup() {
            // TODO: move somewhere else
            self.connections.remove(token).unwrap();
            return;
        }

        // TODO: we should not unwrap() here
        let conn = self.connections.get_mut(token).unwrap();
        let msg = Message::read_from(conn).unwrap();
        let msg_type = msg.message_type();

        println!("Processing connection {:?}", token);
        println!("Received {:?}", msg_type);

        // TODO: do something with result
        match msg {
            Message::Ping => Message::Pong.write_to(conn),
            Message::SetWatchMode(ref msg) => {
                conn.set_watch_mode(msg.mode());
                Message::WatchModeSet.write_to(conn)
            },
            _ => Ok(())
        };
    }

    fn accept(&mut self) {
        println!("New connection");

        let (stream, ..) = self.listener.accept().unwrap();

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

        println!("{:?}", entry.id().timestamp());

        let inner = EntryExpired::new(entry.id().timestamp(), entry.id().id(), entry.consume_data());
        let msg = Message::EntryExpired(inner);

        for conn in &mut self.connections {
            if conn.watch_mode() == WatchMode::None {
                continue;
            }

            msg.write_to(conn);
        }
    }
}