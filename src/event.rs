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
        color: Option<String>,
        highlights: Vec<(String, String)>,
        icons: Vec<(String, String)>,
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

        let mut highlights = vec![];
        let mut icons = vec![];

        for (word, color) in self.settings.get_highlights() {
            if line.contains(word) {
                let color = self.settings.translate_color(&color);

                highlights.push((word.to_owned(), color));
            }
        }

        for (word, icon) in self.settings.get_icons() {
            if line.contains(word) {
                icons.push((word.to_owned(), icon.to_owned()))
            }
        }

        for filter in self.settings.get_filters() {
            if filter.matches(&line) {
                return Event::Announcement {
                    line,
                    group: Some(filter.group.to_owned()),
                    category: Some(filter.category.to_owned()),
                    color: filter.color.to_owned(),
                    highlights,
                    icons,
                };
            }
        }

        Event::Announcement {
            line,
            group: None,
            category: None,
            color: None,
            highlights,
            icons,
        }
    }
}
