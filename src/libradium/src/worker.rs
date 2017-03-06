use threadpool::ThreadPool;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use num_cpus;
use ::backend::{Backend, SharedBackend};
use ::messages::{action, Action};
use ::io::Writable;

fn get_num_workers () -> usize {
    let cores = num_cpus::get();

    match cores {
        1 => 1,
        _ => cores - 1
    }
}

pub struct Worker {
    pool: ThreadPool,
    backend: SharedBackend
}

impl Worker {
    pub fn new(backend: Backend) -> Self {
        Worker {
            pool: ThreadPool::new(get_num_workers()),
            backend: Arc::new(Mutex::new(backend))
        }
    }

    pub fn handle_client(&self, mut stream: TcpStream) {
        let backend = self.backend.clone();

        self.pool.execute(move || {
            let action = action(&mut stream).unwrap();
            let resp = action.perform(backend);

            resp.write_to(&mut stream).unwrap();
        });
    }
}