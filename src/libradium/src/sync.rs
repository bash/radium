use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::convert;

#[derive(Debug)]
pub struct SendError<T>(pub T);

#[derive(Debug)]
pub struct RecvError;

impl<T> convert::From<mpsc::SendError<T>> for SendError<T> {
    fn from(err: mpsc::SendError<T>) -> Self {
        SendError(err.0)
    }
}

impl convert::From<mpsc::RecvError> for RecvError {
    fn from(_: mpsc::RecvError) -> Self {
        RecvError {}
    }
}

#[derive(Debug)]
struct Inner {
    /// Counter for the currently pending messages waiting to be received
    pending: AtomicUsize,
}

#[derive(Debug)]
pub struct Sender<T> {
    tx: mpsc::Sender<T>,
    inner: Arc<Inner>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    tx: mpsc::Receiver<T>,
    inner: Arc<Inner>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = mpsc::channel();
    let inner = Arc::new(Inner::new());

    (
        Sender::new(inner.clone(), sender),
        Receiver::new(inner.clone(), receiver),
    )
}

impl Inner {
    pub fn new() -> Self {
        Inner { pending: AtomicUsize::new(0) }
    }

    pub fn inc_pending(&self) -> usize {
        self.pending.fetch_add(1, Ordering::Acquire)
    }

    pub fn dec_pending(&self) -> usize {
        self.pending.fetch_sub(1, Ordering::Acquire)
    }

    pub fn pending(&self) -> usize {
        self.pending.load(Ordering::SeqCst)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender::new(self.inner.clone(), self.tx.clone())
    }
}

impl<T> Sender<T> {
    fn new(inner: Arc<Inner>, tx: mpsc::Sender<T>) -> Self {
        Sender { inner, tx }
    }

    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.tx.send(t).and_then(|_| {
            self.inner.inc_pending();
            Ok(())
        })?;

        Ok(())
    }
}

impl<T> Receiver<T> {
    fn new(inner: Arc<Inner>, tx: mpsc::Receiver<T>) -> Self {
        Receiver { inner, tx }
    }

    pub fn has_incoming(&self) -> bool {
        self.inner.pending() > 0
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        Ok(self.tx.recv().and_then(|t| {
            self.inner.dec_pending();
            Ok(t)
        })?)
    }
}
