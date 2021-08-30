use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct General {
    gamelog_path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Filter {
    group: String,
    category: String,
    expressions: Vec<String>,
    #[serde(skip)]
    compiled: Vec<Regex>,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    general: General,
    filters: Vec<Filter>,
}

impl Settings {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut settings: Self = serde_json::from_reader(reader)?;

        for filter in settings.filters.iter_mut() {
            filter.compiled = filter
                .expressions
                .iter()
                .map(|e| Regex::new(&e).unwrap())
                .collect();
        }

        Ok(settings)
    }

    pub fn get_gamelog_path(&self) -> PathBuf {
        self.general.gamelog_path.clone()
    }
}
