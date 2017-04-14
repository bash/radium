use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender};

use super::backend::BackendAction;

pub struct IntervalWorker {
    action_tx: Sender<BackendAction>,
}

impl IntervalWorker {
    pub fn new(action_tx: Sender<BackendAction>) -> IntervalWorker {
        IntervalWorker {
            action_tx,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                // TODO: find optimal sleep duration
                thread::sleep(Duration::from_secs(1));

                self.action_tx.send(BackendAction::CheckExpired).unwrap();
            }
        })
    }
}