use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub preset_path: PathBuf,
    pub texture_path: PathBuf,
    pub frame_rate: u32,
    pub preset_duration: f64,
    pub beat_sensitivity: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            preset_path: PathBuf::from("/usr/share/projectM/presets/"),
            texture_path: PathBuf::from(""),
            frame_rate: 60,
            preset_duration: 10.0,
            beat_sensitivity: 1.0,
        }
    }
}

pub fn load_config() -> Config {
    Config::default()
}
