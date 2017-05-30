use std::io;
use std::error::Error;
use std::fmt;

use libradium::Frontend;
use mio_channel::Receiver;
use mio::{Poll, Token, Ready, PollOpt, Events, Event};
use mio::unix::UnixReady;
use radium_protocol::{Message, WriteValueExt, ErrorCode};
use radium_protocol::errors::{ReadError, WriteError};
use radium_protocol::messages::{EntryExpired, ErrorMessage};

use super::actions::Action;
use super::connection::{Connection, Connections, Added, Rejected};
use super::entry::{Entry, EntryData};

pub const MESSAGE_TOKEN: Token = Token(10_000_000);
pub const DEFAULT_WORKER_CONNECTIONS: usize = 128;

pub enum WorkerMessage {
    Connection(Connection),
    Push(Entry)
}

#[derive(Debug)]
pub enum WorkerError {
    ReadError(ReadError),
    WriteError(WriteError),
    IoError(io::Error)
}

type WorkerResult<T> = Result<T, WorkerError>;

pub struct Worker {
    id: usize,
    connections: Connections,
    poll: Poll,
    receiver: Receiver<WorkerMessage>,
    frontend: Frontend<EntryData>,
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for WorkerError {
    fn description(&self) -> &str {
        match self {
            &WorkerError::ReadError(ref err) => err.description(),
            &WorkerError::WriteError(ref err) => err.description(),
            &WorkerError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &WorkerError::ReadError(ref err) => err.cause(),
            &WorkerError::WriteError(ref err) => err.cause(),
            &WorkerError::IoError(ref err) => err.cause(),
        }
    }
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

impl From<WriteError> for WorkerError {
    fn from(err: WriteError) -> Self {
        WorkerError::WriteError(err)
    }
}

impl Worker {
    pub fn new(id: usize, poll: Poll, receiver: Receiver<WorkerMessage>, frontend: Frontend<EntryData>) -> Self {
        let connections = env_var!("RADIUM_WORKER_CONNECTIONS", DEFAULT_WORKER_CONNECTIONS);

        Worker {
            id,
            connections: Connections::with_capacity(connections),
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
            println!("Disconnecting because ready {:?}", unix_ready);
            self.disconnect(token, None).unwrap();
            return;
        }

        if token == MESSAGE_TOKEN {
            let msg = self.receiver.try_recv().unwrap();

            return match msg {
                WorkerMessage::Connection(conn) => { self.accept(conn) }
                WorkerMessage::Push(entry) => { self.push(entry) }
            }
        }

        if let Err(..) = self.handle_msg(token) {
            self.disconnect(token, Some(ErrorCode::ConnectionFailure)).unwrap();
        }
    }

    fn handle_msg(&mut self, token: Token) -> WorkerResult<()> {
        if let Some(conn) = self.connections.get_conn_mut(token) {
            let msg: Message = match conn.read_message()? {
                Some(msg) => { msg }
                None => { return Ok(()) }
            };

            debug!("worker {}, conn {} | {:?}", self.id, token.0, msg);

            let msg_type = msg.message_type();

            let resp: Message = match msg.process(conn, &mut self.frontend) {
                Ok(resp) => { resp }
                Err(err) => { err.into() }
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
            Rejected(conn) => { conn.close() }
        };

        // TODO: idk what to do with this result
        result.unwrap();
    }

    fn push(&mut self, entry: Entry) {
        let id = entry.id();
        let tag = entry.data().tag();
        let inner = EntryExpired::new(id.timestamp(), id.id(), tag, entry.consume_data().consume_data());
        let msg = Message::EntryExpired(inner);

        let conns = self.connections
            .iter_mut()
            .filter(|conn| conn.watch_mode().matches_tag(tag));

        for conn in conns {
            let _ = conn.write_value(&msg);
        }
    }

    fn disconnect(&mut self, token: Token, code: Option<ErrorCode>) -> WorkerResult<()> {
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