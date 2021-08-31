use std::sync::mpsc;

use crate::settings::Settings;

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

pub struct EventFactory {
    settings: Settings,
}

impl EventFactory {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn create(&self, line: &str) -> Event {
        let line = String::from(line);
        let filters = self.settings.get_filters();

        for filter in filters {
            if filter.matches(&line) {
                return Event::Announcement {
                    line,
                    group: Some(filter.group.to_owned()),
                    category: Some(filter.category.to_owned()),
                };
            }
        }

        Event::Announcement {
            line,
            group: None,
            category: None,
        }
    }
}
