use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::event::{Event, EventReceiver, EventSender};
use crate::settings::Settings;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StartLocation {
    Beginning,
    BeginningOfFortress,
    End,
}

impl From<StartLocation> for SeekFrom {
    fn from(location: StartLocation) -> SeekFrom {
        match location {
            StartLocation::Beginning | StartLocation::BeginningOfFortress => SeekFrom::Start(0),
            StartLocation::End => SeekFrom::End(0),
        }
    }
}

pub struct Gamelog {
    settings: Settings,
}

impl Gamelog {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn connect(&mut self, start: StartLocation) -> io::Result<EventReceiver> {
        let (es, er): (EventSender, EventReceiver) = mpsc::channel();

        let path = self.settings.get_gamelog_path();
        let file = File::open(&path)?;

        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            // Get correct starting position.
            let position: SeekFrom = start.into();

            let mut reader = BufReader::new(file);
            reader.seek(position).unwrap();

            // Parse log and send intitial event.
            let mut head = vec![];

            for line in reader.by_ref().lines() {
                let line = line.unwrap();

                if start == StartLocation::BeginningOfFortress
                    && line.contains("** Loading Fortress **")
                {
                    head = vec![];
                }

                head.push(line.trim().to_owned());
            }

            let event = Event::initial_log(head);
            es.send(event).unwrap();

            // Watch for new writes to the log.
            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
            watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

            let mut buffer = String::new();

            loop {
                match rx.recv() {
                    Ok(event) => {
                        if let DebouncedEvent::Write(_) = event {
                            // This really only works if the gamelog is only ever appended to. I think that's true?
                            reader.read_to_string(&mut buffer).unwrap();

                            for line in buffer.lines() {
                                let line = line.trim();

                                if !line.is_empty() {
                                    let event = Event::announcement(line);
                                    // TODO: I think I can handle this better by creating my own errors.
                                    es.send(event).unwrap()
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // TODO: Implement an event that we can gracefully exit on.
                        panic!("{}", e);
                    }
                }
            }
        });

        Ok(er)
    }
}
