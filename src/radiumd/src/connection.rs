use std::io;
use std::net::Shutdown;
use mio::{Evented, Poll, Token, Ready, PollOpt};
use mio::tcp::TcpStream;
use slab::{Slab, IterMut};
use std::collections::VecDeque;
use radium_protocol::{WatchMode, Message, MessageReader, WriteValueExt};
use radium_protocol::errors::WriteError;
use radium_protocol::reader::{ReaderController, ReaderStatus, HasReader};
pub use self::AddConnResult::{Added, Rejected};

#[derive(Debug)]
pub struct Connection {
    sock: TcpStream,
    watch_mode: WatchMode,
    reader: ReaderController<Message, MessageReader>,
    write_queue: VecDeque<Message>,
}

pub enum AddConnResult<'a> {
    Added(&'a Connection, Token),
    Rejected(Connection),
}

pub struct Connections {
    inner: Slab<Connection, Token>
}

impl Connection {
    pub fn new(sock: TcpStream) -> Self {
        Connection {
            sock,
            watch_mode: WatchMode::None,
            reader: ReaderController::new(Message::reader()),
            write_queue: VecDeque::new(),
        }
    }

    pub fn set_watch_mode(&mut self, mode: WatchMode) {
        self.watch_mode = mode;
    }

    pub fn watch_mode(&self) -> WatchMode {
        self.watch_mode
    }

    pub fn close(&self) -> io::Result<()> {
        self.sock.shutdown(Shutdown::Both)
    }

    pub fn read_message(&mut self) -> io::Result<Option<Message>> {
        match self.reader.resume(&mut self.sock)? {
            ReaderStatus::Pending => { Ok(None) }
            ReaderStatus::Complete(val) => Ok(Some(val))
        }
    }

    pub fn write_message(&mut self, msg: Message) -> io::Result<()> {
        let result = self.sock.write_value(&msg);

        if let Err(WriteError::IoError(ref err)) = result {
            if err.kind() == io::ErrorKind::WouldBlock {
                self.write_queue.push_back(msg);
            }
        }

        result?;
        Ok(())
    }

    pub fn resume_write(&mut self, ready: Ready) -> io::Result<()> {
        if ready.is_writable() {
            if let Some(msg) = self.write_queue.pop_front() {
                self.sock.write_value(&msg)?;
            }
        }

        Ok(())
    }
}

impl Evented for Connection {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        poll.register(&self.sock, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        poll.reregister(&self.sock, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        poll.deregister(&self.sock)
    }
}

impl io::Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.sock.read(buf)
    }
}

impl io::Write for Connection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sock.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sock.flush()
    }
}

impl Connections {
    pub fn with_capacity(capacity: usize) -> Self {
        Connections {
            inner: Slab::with_capacity(capacity)
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<Connection, Token> {
        self.inner.iter_mut()
    }

    pub fn get_conn_mut(&mut self, token: Token) -> Option<&mut Connection> {
        self.inner.get_mut(token)
    }

    pub fn remove_conn(&mut self, token: Token) -> Option<Connection> {
        self.inner.remove(token)
    }

    pub fn add_conn(&mut self, conn: Connection) -> AddConnResult {
        let token = {
            let vacant = match self.inner.vacant_entry() {
                None => { return Rejected(conn) },
                Some(vacant) => vacant
            };

            vacant.insert(conn).index()
        };

        // We know that `token` must exist in our slab, because we just inserted it
        let conn_ref = self.inner
            .get(token)
            .expect("Filled slot in Entry");

        Added(conn_ref, token)
    }
}