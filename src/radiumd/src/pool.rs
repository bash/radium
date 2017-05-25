use super::connection::Connection;
use mio::channel::{channel, Sender, Receiver};
use std::thread;
use mio::{Poll, Token, Ready, PollOpt, Events};
use mio::unix::UnixReady;
use slab::Slab;
use radium_protocol::{Message, ReadValue};
use libradium::Entry;
use std::io;
use super::entry::EntryData;

const MESSAGE: Token = Token(10_000_000);

enum WorkerMessage {
    Connection(Connection),
    Push(Entry<EntryData>)
}

struct Worker {
    connections: Connections,
    poll: Poll,
    receiver: Receiver<WorkerMessage>
}

pub struct Connections {
    inner: Slab<Connection, Token>
}

pub struct Pool {
    next_pool: usize,
    num_threads: usize,
    threads: Vec<Sender<WorkerMessage>>,
}

impl Worker {
    pub fn new(poll: Poll, receiver: Receiver<WorkerMessage>) -> Self {
        Worker {
            connections: Connections::with_capacity(128),
            poll: poll,
            receiver: receiver
        }
    }

    pub fn spawn() -> io::Result<Sender<WorkerMessage>> {
        let (sender, receiver) = channel::<WorkerMessage>();
        let poll = Poll::new()?;
        poll.register(&receiver, MESSAGE, Ready::readable(), PollOpt::edge())?;

        let mut worker = Worker::new(poll, receiver);

        thread::spawn(move || {
            worker.run();
        });

        Ok(sender)
    }

    pub fn run(&mut self) {
        let mut events = Events::with_capacity(1024);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for i in 0..events.len() {
                let event = events.get(i).unwrap();
                let token = event.token();

                match token {
                    MESSAGE => {
                        let msg = self.receiver.try_recv().unwrap();

                        match msg {
                            WorkerMessage::Connection(conn) => {
                                let (conn_ref, token) = self.connections.add_conn(conn);

                                self.poll.register(conn_ref, token, Ready::readable(), PollOpt::edge())
                                    .unwrap();
                            },
                            WorkerMessage::Push(entry) => {
                                debug!("Entry has expired: {:?}", entry.id())
                            }
                        }
                    },
                    _ if self.connections.has_conn(token) => {
                        let ready = event.readiness();
                        let conn = self.connections.get_conn(token).unwrap();

                        if UnixReady::from(ready).is_hup() {
                            self.poll.deregister(conn);
                            return;
                            // TODO: disconnect
                        }

                        if !ready.is_readable() {
                            continue;
                        }


                        let msg = conn.read_value::<Message>().unwrap();

                        debug!("Received {:?}", msg.message_type());
                    },
                    _ => unreachable!()
                }
            }
        }
    }
}

impl Connections {
    pub fn with_capacity(capacity: usize) -> Self {
        Connections {
            inner: Slab::with_capacity(capacity)
        }
    }

    pub fn has_conn(&self, token: Token) -> bool {
        self.inner.get(token).is_some()
    }

    pub fn get_conn(&mut self, token: Token) -> Option<&mut Connection> {
        self.inner.get_mut(token)
    }

    pub fn add_conn(&mut self, conn: Connection) -> (&Connection, Token) {
        // TODO: delegate .unwrap() to caller
        let token = {
            let entry = self.inner.vacant_entry().unwrap();

            entry.insert(conn).index()
        };
        let conn_ref = self.inner.get(token).unwrap();

        (conn_ref, token)
    }
}

impl Pool {
    pub fn new(num_threads: usize) -> Pool {
        // TODO: don't unwrap here
        let threads = (0..num_threads)
            .map(|_| Worker::spawn().unwrap())
            .collect();

        Pool { threads, num_threads, next_pool: 0 }
    }

    pub fn register(&mut self, conn: Connection) {
        {
            let sender = self.threads.get(self.next_pool).unwrap();
            sender.send(WorkerMessage::Connection(conn));
        }

        self.next_pool();
    }

    pub fn push_expired(&self, entry: Entry<EntryData>) {
        for thread in &self.threads {
            // TODO: we probably shouldn't clone the entry for every thread
            thread.send(WorkerMessage::Push(entry.clone()));
        }
    }

    fn next_pool(&mut self) {
        self.next_pool += 1;

        if self.next_pool == self.num_threads {
            self.next_pool = 0;
        }
    }
}