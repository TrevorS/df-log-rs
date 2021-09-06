use std::sync::mpsc;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    Announcement(String),
    InitialLog(Vec<String>),
}

impl Event {
    pub fn announcement(text: &str) -> Self {
        Self::Announcement(text.into())
    }

    pub fn initial_log(lines: Vec<String>) -> Self {
        Self::InitialLog(lines)
    }
}
