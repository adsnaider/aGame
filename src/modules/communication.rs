use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Sender<M> {
    sender: mpsc::Sender<M>,
}
pub struct Receiver<M> {
    receiver: Arc<Mutex<mpsc::Receiver<M>>>,
}

pub struct Channel<M> {
    sender: Sender<M>,
    receiver: Receiver<M>,
}

impl<M> Clone for Receiver<M> {
    fn clone(&self) -> Self {
        let rx = Arc::clone(&self.receiver);
        Receiver { receiver: rx }
    }
}

impl<M> Channel<M> {
    pub fn new() -> Channel<M> {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        Channel {
            sender: Sender { sender: tx },
            receiver: Receiver { receiver: rx },
        }
    }

    pub fn receiver(&self) -> &Receiver<M> {
        &self.receiver
    }
}
