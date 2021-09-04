use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::event::{Event, EventReceiver, EventSender};
use crate::settings::Settings;

pub struct Gamelog {
    settings: Settings,
}

impl Gamelog {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn connect(&mut self) -> io::Result<EventReceiver> {
        let (es, er): (EventSender, EventReceiver) = mpsc::channel();

        let path = self.settings.get_gamelog_path();
        let file = File::open(&path)?;

        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
            watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

            // TODO: Read from start of log if asked to.
            let mut reader = BufReader::new(file);
            reader.seek(SeekFrom::End(0)).unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => {
                        if let DebouncedEvent::Write(_) = event {
                            // This really only works if the gamelog is only ever appended to. I think that's true?
                            let mut buffer = String::new();
                            reader.read_to_string(&mut buffer).unwrap();

                            for line in buffer.lines() {
                                let line = line.trim();

                                if !line.is_empty() {
                                    let event = Event::new(line);
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
