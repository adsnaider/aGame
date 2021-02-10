use std::sync::mpsc;
use std::time;

pub struct Event<T> {
    send_time: (time::SystemTime, time::Instant),
    data: T,
}

pub struct EventSender<T> {
    sender: mpsc::Sender<Event<T>>,
}

pub struct EventReceiver<T> {
    receiver: mpsc::Receiver<Event<T>>,
}

impl<T> EventSender<T> {
    fn send(&self, message: T) -> Result<(), mpsc::SendError<Event<T>>> {
        let event = Event {
            send_time: (time::SystemTime::now(), time::Instant::now()),
            data: message,
        };
        self.sender.send(event)
    }
}

impl<T> EventReceiver<T> {
    fn recv(&self) -> Result<Event<T>, mpsc::RecvError> {
        self.receiver.recv()
    }
}
