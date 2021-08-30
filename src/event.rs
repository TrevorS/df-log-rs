use std::sync::mpsc;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug)]
pub enum Event {
    Announcement {
        line: String,
        group: Option<String>,
        category: Option<String>,
    },
}

impl Event {
    pub fn new(line: &str) -> Self {
        let line = String::from(line);

        let group = None;
        let category = None;

        Self::Announcement {
            line,
            group,
            category,
        }
    }
}
