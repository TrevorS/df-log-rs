extern crate eframe;
extern crate notify;
extern crate regex;
extern crate serde;
extern crate serde_json;

mod app;
mod event;
mod gamelog;
mod highlighter;
mod settings;

use std::path::PathBuf;

use app::App;
use gamelog::Gamelog;
use settings::Settings;

const SETTINGS_PATH: &str = "./settings.json";

fn main() {
    let settings = Settings::new(PathBuf::from(SETTINGS_PATH)).unwrap();

    let mut gamelog = Gamelog::new(settings.clone());
    let rx = gamelog.connect().expect("Failed to read gamelog.txt!");

    let df_app = App::new(settings, rx);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(df_app), native_options);
}
