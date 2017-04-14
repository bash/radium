use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use super::super::backend::Backend;
use super::subscription::SubscrAction;

#[derive(Debug)]
pub enum BackendResp {
    Added
}

#[derive(Debug)]
pub enum BackendAction {
    CheckExpired,
    Add(Sender<BackendResp>)
}

pub struct BackendWorker {
    action_rx: Receiver<BackendAction>,
    sub_tx: Sender<SubscrAction>,
    backend: Backend
}

impl BackendWorker {
    pub fn new(action_rx: Receiver<BackendAction>, sub_tx: Sender<SubscrAction>, backend: Backend) -> BackendWorker {
        BackendWorker {
            action_rx,
            sub_tx,
            backend,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let action = self.action_rx.recv().unwrap();

                println!("action: {:?}", action);

                match action {
                    BackendAction::CheckExpired => {
                        self.sub_tx.send(SubscrAction::Push).unwrap();
                    },
                    BackendAction::Add(conn_tx) => {
                        conn_tx.send(BackendResp::Added).unwrap();
                    }
                }
            }
        })
    }
}