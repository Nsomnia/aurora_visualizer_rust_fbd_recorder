use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const BLOCKLIST_FILE: &str = "preset_blocklist.txt";

pub struct PresetBlocklist {
    blocked_presets: HashSet<String>,
}

impl PresetBlocklist {
    pub fn new() -> Self {
        let blocked_presets = if Path::new(BLOCKLIST_FILE).exists() {
            let file = File::open(BLOCKLIST_FILE).unwrap();
            let reader = BufReader::new(file);
            reader.lines().map(|line| line.unwrap()).collect()
        } else {
            HashSet::new()
        };

        Self { blocked_presets }
    }

    pub fn add(&mut self, preset_path: &str) {
        if self.blocked_presets.insert(preset_path.to_string()) {
            self.save();
        }
    }

    pub fn contains(&self, preset_path: &str) -> bool {
        self.blocked_presets.contains(preset_path)
    }

    fn save(&self) {
        let mut file = File::create(BLOCKLIST_FILE).unwrap();
        for preset in &self.blocked_presets {
            writeln!(file, "{}", preset).unwrap();
        }
    }
}
