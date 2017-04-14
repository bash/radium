use std::sync::mpsc::{Sender, Receiver};
use std::net::TcpStream;
use std::thread;

#[derive(Debug)]
pub enum SubscrAction {
    Add,
    Push
}

pub struct SubscrWorker {
    sub_rx: Receiver<SubscrAction>,
    listeners: Vec<TcpStream>,
}

impl SubscrWorker {
    pub fn new(sub_rx: Receiver<SubscrAction>) -> SubscrWorker {
        SubscrWorker {
            sub_rx,
            listeners: Vec::new()
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                println!("sub action: {:?}", self.sub_rx.recv());
            }
        })
    }
}