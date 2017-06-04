use std::io;
use std::thread;
use libradium::Frontend;
use mio_channel::{channel, Sender, SendError};
use mio::{Poll, Ready, PollOpt};
use mio::unix::UnixReady;
use super::connection::Connection;
use super::entry::{Entry, EntryData};
use super::worker::{Worker, WorkerMessage, MESSAGE_TOKEN};

pub fn spawn_worker(id: usize, frontend: Frontend<EntryData>) -> io::Result<Sender<WorkerMessage>> {
    let (sender, receiver) = channel::<WorkerMessage>();

    let poll = Poll::new()?;
    poll.register(&receiver, MESSAGE_TOKEN, Ready::readable() | UnixReady::hup(), PollOpt::edge())?;

    let mut worker = Worker::new(id, poll, receiver, frontend);

    thread::spawn(move || {
        worker.run();
    });

    Ok(sender)
}

pub struct Pool {
    next_worker: usize,
    num_workers: usize,
    workers: Vec<Sender<WorkerMessage>>,
}

impl Pool {
    pub fn build(frontend: Frontend<EntryData>, num_workers: usize) -> Pool {
        // TODO: don't unwrap here
        let workers = (0..num_workers)
            .map(|i| spawn_worker(i, frontend.clone()).unwrap())
            .collect();

        Pool { workers, num_workers, next_worker: 0 }
    }

    pub fn register(&mut self, conn: Connection) -> Result<(), SendError<WorkerMessage>> {
        {
            let sender = self.workers.get(self.next_worker).unwrap();
            sender.send(WorkerMessage::Connection(conn))?;
        }

        self.next_worker();

        Ok(())
    }

    pub fn push_expired(&self, entry: Vec<Entry>) -> Result<(), SendError<WorkerMessage>> {
        for worker in &self.workers {
            // TODO: we probably shouldn't clone the entry for every thread
            worker.send(WorkerMessage::Push(entry.clone()))?;
        }

        Ok(())
    }

    fn next_worker(&mut self) {
        self.next_worker += 1;

        if self.next_worker == self.num_workers {
            self.next_worker = 0;
        }
    }
}