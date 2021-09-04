use std::sync::mpsc;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug)]
pub enum EventType {
    Announcement,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub line: String,
}

impl Event {
    pub fn new(line: &str) -> Self {
        Self {
            event_type: EventType::Announcement,
            line: line.into(),
        }
    }
}
