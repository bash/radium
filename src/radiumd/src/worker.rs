use std::io;

use libradium::Frontend;
use mio_channel::Receiver;
use mio::{Poll, Token, Ready, PollOpt, Events, Event};
use mio::unix::UnixReady;
use radium_protocol::{Message, ReadValue, WriteValue, WatchMode, ReadError};
use radium_protocol::messages::{EntryExpired, ErrorMessage, ErrorCode};

use super::actions::Action;
use super::connection::{Connection, Connections, Added, Rejected};
use super::entry::{Entry, EntryData};

pub const MESSAGE_TOKEN: Token = Token(10_000_000);
pub const DEFAULT_WORKER_CONNECTIONS: usize = 128;

pub enum WorkerMessage {
    Connection(Connection),
    Push(Entry)
}

pub enum WorkerError {
    ReadError(ReadError),
    IoError(io::Error)
}

pub struct Worker {
    id: usize,
    connections: Connections,
    poll: Poll,
    receiver: Receiver<WorkerMessage>,
    frontend: Frontend<EntryData>,
}

impl From<io::Error> for WorkerError {
    fn from(err: io::Error) -> Self {
        WorkerError::IoError(err)
    }
}

impl From<ReadError> for WorkerError {
    fn from(err: ReadError) -> Self {
        WorkerError::ReadError(err)
    }
}

impl Worker {
    pub fn new(id: usize, poll: Poll, receiver: Receiver<WorkerMessage>, frontend: Frontend<EntryData>) -> Self {
        Worker {
            id,
            connections: Connections::with_capacity(env_var!("RADIUM_WORKER_CONNECTIONS", DEFAULT_WORKER_CONNECTIONS)),
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
        let unix_ready = UnixReady::from(ready);

        if unix_ready.is_hup() || unix_ready.is_error() {
            self.disconnect(token, None).unwrap();
            return;
        }

        if token == MESSAGE_TOKEN {
            let msg = self.receiver.try_recv().unwrap();

            return match msg {
                WorkerMessage::Connection(conn) => self.accept(conn),
                WorkerMessage::Push(entry) => self.push(entry)
            }
        }

        if let Err(..) = self.handle_msg(token) {
            self.disconnect(token, Some(ErrorCode::ConnectionFailure)).unwrap();
        }
    }

    fn handle_msg(&mut self, token: Token) -> Result<(), WorkerError> {
        if let Some(conn) = self.connections.get_conn_mut(token) {
            let msg: Message = conn.read_value::<Message>()?;

            let msg_type = msg.message_type();

            let resp: Message = match msg.process(conn, &mut self.frontend) {
                Ok(resp) => resp,
                Err(err) => err.into()
            };

            debug!("worker {}, conn {} | {:?} -> {:?}", self.id, token.0, msg_type, resp.message_type());

            conn.write_value(&resp)?;
        }

        Ok(())
    }

    fn accept(&mut self, conn: Connection) {
        let result = match self.connections.add_conn(conn) {
            Added(conn_ref, token) => {
                self.poll.register(conn_ref, token, Ready::readable(), PollOpt::edge())
            }
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

    fn disconnect(&mut self, token: Token, code: Option<ErrorCode>) -> io::Result<()> {
        let conn = self.connections.remove_conn(token);

        match conn {
            Some(mut conn) => {
                self.poll.deregister(&conn)?;

                // We're intentionally ignoring the result here
                // don't need the guarantee that the error code has come through
                if let Some(code) = code {
                    let _ = conn.write_value(&ErrorMessage::new(code));
                }

                Ok(())
            }
            None => { Ok(()) }
        }
    }
}