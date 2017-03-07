use threadpool::ThreadPool;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use num_cpus;
use super::backend::{Backend, SharedBackend};
use super::messages::{action, Action};
use super::connection::ConnectionMode;
use super::io::{Readable, Writable};
use super::messages::WrappedAction;

fn get_num_workers () -> usize {
    let cores = num_cpus::get();

    match cores {
        1 => 1,
        _ => cores - 1
    }
}

fn receive_actions(mut stream: TcpStream, backend: SharedBackend) {
    loop {
        let backend = backend.clone();
        let action = action(&mut stream).unwrap();

        println!("Receive {:?}", action);

        if let WrappedAction::Close(_) = action {
            return;
        }

        let resp = action.perform(backend);

        resp.write_to(&mut stream).unwrap();
    }
}

fn register_listener(stream: TcpStream, backend: SharedBackend) {
    let mut backend = backend.lock().unwrap();

    backend.add_listener(stream);
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
            let mode = ConnectionMode::read_from(&mut stream).unwrap();

            println!("{:?}", mode);

            match mode {
                ConnectionMode::Action => receive_actions(stream, backend),
                ConnectionMode::Listen => register_listener(stream, backend)
            }
        });
    }
}