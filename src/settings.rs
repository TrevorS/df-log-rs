use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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

    pub fn get_filters(&self) -> &Vec<Filter> {
        &self.filters
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct General {
    gamelog_path: PathBuf,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Filter {
    pub group: String,
    pub category: String,
    expressions: Vec<String>,
    #[serde(skip)]
    compiled: Vec<Regex>,
}

impl Filter {
    pub fn matches(&self, line: &str) -> bool {
        self.compiled.iter().any(|r| r.is_match(line))
    }
}
