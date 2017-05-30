use std::io;
use std::net::Shutdown;
use mio::{Evented, Poll, Token, Ready, PollOpt};
use mio::tcp::TcpStream;
use slab::{Slab, IterMut};
use radium_protocol::{WatchMode, ReaderController, Reader, Message, MessageReader, ReaderStatus};
pub use self::AddConnResult::{Added, Rejected};

pub struct Connection {
    sock: TcpStream,
    watch_mode: WatchMode,
    reader: ReaderController<Message, MessageReader>
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
            reader: ReaderController::new(Message::reader())
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
            ReaderStatus::Ended => { panic!("State should not exist") }
            ReaderStatus::Pending => { Ok(None) }
            ReaderStatus::Complete(val) => {
                self.reader.rewind();
                Ok(Some(val))
            }
        }
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