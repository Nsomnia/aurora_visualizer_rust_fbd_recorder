use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const FAVORITES_FILE: &str = "favorites.txt";

pub struct Favorites {
    presets: HashSet<String>,
}

impl Favorites {
    pub fn new() -> Self {
        let presets = if Path::new(FAVORITES_FILE).exists() {
            let file = File::open(FAVORITES_FILE).unwrap();
            let reader = BufReader::new(file);
            reader.lines().map(|line| line.unwrap()).collect()
        } else {
            HashSet::new()
        };

        Self { presets }
    }

    pub fn add(&mut self, preset_path: &str) {
        if self.presets.insert(preset_path.to_string()) {
            self.save();
        }
    }

    pub fn remove(&mut self, preset_path: &str) {
        if self.presets.remove(preset_path) {
            self.save();
        }
    }

    pub fn contains(&self, preset_path: &str) -> bool {
        self.presets.contains(preset_path)
    }

    pub fn is_empty(&self) -> bool {
        self.presets.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.presets.iter()
    }

    fn save(&self) {
        let mut file = File::create(FAVORITES_FILE).unwrap();
        for preset in &self.presets {
            writeln!(file, "{}", preset).unwrap();
        }
    }
}
