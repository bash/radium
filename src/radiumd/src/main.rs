extern crate libradium;


use libradium::workers::{BackendWorker, BackendAction, BackendResp, SubscrAction, SubscrWorker, IntervalWorker};
use libradium::backend::Backend;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (action_tx, action_rx): (Sender<BackendAction>, Receiver<BackendAction>) = mpsc::channel();
    let (sub_tx, sub_rx): (Sender<SubscrAction>, Receiver<SubscrAction>) = mpsc::channel();

    let backend_worker = BackendWorker::new(action_rx, sub_tx.clone(), Backend::new());
    let subscr_worker = SubscrWorker::new(sub_rx);
    let int_worker = IntervalWorker::new(action_tx.clone());

    backend_worker.spawn();
    subscr_worker.spawn();
    int_worker.spawn().join();

    // thread::sleep_ms(10000);

    /*let (sub_tx, sub_rx): (Sender<SubscriptionAction>, Receiver<SubscriptionAction>) = mpsc::channel();
    let (action_tx, action_rx): (Sender<Action>, Receiver<Action>) = mpsc::channel();

    {
        // backend thread
        let sub_tx = sub_tx.clone();
        let action_rx = action_rx;

        thread::spawn(move || {
            loop {
                let action = action_rx.recv().unwrap();

                println!("action: {:?}", action);

                match action {
                    Action::CheckExpired => {
                        sub_tx.send(SubscriptionAction::Push);
                    },
                    Action::Add(conn_tx) => {
                        conn_tx.send(Response::Added);
                    }
                }
            }
        });
    }

    {
        // subscription thread
        let sub_rx = sub_rx;

        thread::spawn(move || {
            loop {
                println!("sub action: {:?}", sub_rx.recv());
            }
        });
    }

    {
        // interval thread
        let action_tx = action_tx.clone();

        thread::spawn(move || {
            loop {
                action_tx.send(Action::CheckExpired);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // connection threads
    for id in 0..2 {
        let (conn_tx, conn_rx): (Sender<Response>, Receiver<Response>) = mpsc::channel();
        let action_tx = action_tx.clone();

        thread::spawn(move || {
            action_tx.send(Action::Add(conn_tx));

            let resp = conn_rx.recv().unwrap();

            println!("resp in conn thread #{}: {:?}", id, resp);
        });
    }


    thread::sleep_ms(10000);*/

}
