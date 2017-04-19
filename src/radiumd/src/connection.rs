use std::io;
use mio::{Evented, Poll, Token, Ready, PollOpt};
use mio::tcp::TcpStream;
use radium_protocol::WatchMode;

pub struct Connection {
    sock: TcpStream,
    watch_mode: WatchMode,
}

impl Connection {
    pub fn new(sock: TcpStream) -> Self {
        Connection {
            sock,
            watch_mode: WatchMode::None
        }
    }

    pub fn set_watch_mode(&mut self, mode: WatchMode) {
        self.watch_mode = mode;
    }

    pub fn watch_mode(&self) -> WatchMode {
        self.watch_mode
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