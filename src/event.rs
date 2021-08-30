use std::sync::mpsc;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug)]
pub struct Event {
    message: String,
}

impl Event {
    pub fn new(message: &str) -> Self {
        let message = String::from(message);

        Self { message }
    }
}