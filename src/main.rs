extern crate notify;
extern crate regex;
extern crate serde;
extern crate serde_json;

mod event;
mod gamelog;
mod settings;

use std::path::PathBuf;

use gamelog::Gamelog;
use settings::Settings;

const SETTINGS_PATH: &str = "./settings.json";

fn main() {
    let settings = Settings::new(PathBuf::from(SETTINGS_PATH)).unwrap();
    dbg!(&settings);

    let filename = settings.get_gamelog_path();
    let mut gamelog = Gamelog::new(filename);

    let rx = gamelog.connect().expect("Failed to read gamelog.txt!");

    loop {
        match rx.recv() {
            Ok(event) => {
                dbg!(event);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
