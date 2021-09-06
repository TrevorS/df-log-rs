use std::sync::mpsc;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    Announcement,
    InitialLog,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub text: String,
}

impl Event {
    pub fn announcement(text: &str) -> Self {
        Self {
            event_type: EventType::Announcement,
            text: text.into(),
        }
    }

    pub fn initial_log(text: &str) -> Self {
        Self {
            event_type: EventType::InitialLog,
            text: text.into(),
        }
    }

    pub fn split_text(&self) -> Vec<String> {
        self.text
            .split('\n')
            .map(|l| l.trim().to_string())
            .collect()
    }
}
