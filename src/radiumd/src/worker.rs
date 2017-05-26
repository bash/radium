use std::io;

use libradium::Frontend;
use mio_channel::Receiver;
use mio::{Poll, Token, Ready, PollOpt, Events, Event};
use radium_protocol::{Message, ReadValue, WriteValue, WatchMode};
use radium_protocol::messages::EntryExpired;

use super::actions::Action;
use super::connection::{Connection, Connections, Added, Rejected};
use super::entry::{Entry, EntryData};

pub const MESSAGE_TOKEN: Token = Token(10_000_000);

pub enum WorkerMessage {
    Connection(Connection),
    Push(Entry)
}

pub struct Worker {
    id: usize,
    connections: Connections,
    poll: Poll,
    receiver: Receiver<WorkerMessage>,
    // TODO: remove #[allow(dead_code)]
    #[allow(dead_code)]
    frontend: Frontend<EntryData>,
}

impl Worker {
    pub fn new(id: usize, poll: Poll, receiver: Receiver<WorkerMessage>, frontend: Frontend<EntryData>) -> Self {
        Worker {
            id,
            connections: Connections::with_capacity(128),
            poll,
            receiver,
            frontend,
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::with_capacity(1024);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for i in 0..events.len() {
                let event = events.get(i).unwrap();

                self.handle_event(event);
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        let token = event.token();
        let ready = event.readiness();

        // TODO: make sure this doesn't cause unintended side effects, otherwise switch to UnixReady.hup()
        if !ready.is_readable() {
            // TODO: don't unwrap
            self.disconnect(token).unwrap();
            return;
        }

        if token == MESSAGE_TOKEN {
            let msg = self.receiver.try_recv().unwrap();

            return match msg {
                WorkerMessage::Connection(conn) => self.accept(conn),
                WorkerMessage::Push(entry) => self.push(entry)
            }
        }

        if let Some(conn) = self.connections.get_conn_mut(token) {
            let msg: Message = conn.read_value::<Message>().unwrap();

            let resp = match msg.process(conn) {
                Ok(resp) => resp,
                Err(..) => Message::Error
            };

            // TODO: don't unwrap
            conn.write_value(&resp).unwrap();

            // TODO: close connection if read or write fails

            debug!("worker {}, conn {} | {:?} -> {:?}", self.id, token.0, msg, resp);

            return;
        }

        // TODO: is there anything else we need to do here?
        error!("Unable to handle token {:?}", token);
    }

    fn accept(&mut self, conn: Connection) {
        let result = match self.connections.add_conn(conn) {
            Added(conn_ref, token) => {
                self.poll.register(conn_ref, token, Ready::readable(), PollOpt::edge())
            },
            Rejected(conn) => conn.close()
        };

        // TODO: idk what to do with this result
        result.unwrap();
    }

    fn push(&mut self, entry: Entry) {
        let id = entry.id();
        let inner = EntryExpired::new(id.timestamp(), id.id(), entry.consume_data());
        let msg = Message::EntryExpired(inner);

        for conn in self.connections.iter_mut() {
            if conn.watch_mode() == WatchMode::None {
                continue;
            }

            // TODO: error handling
            let _ = conn.write_value(&msg);
        }
    }

    fn disconnect(&mut self, token: Token) -> io::Result<()> {
        let conn = self.connections.remove_conn(token);

        match conn {
            Some(conn) => self.poll.deregister(&conn),
            None => Ok(())
        }
    }
}