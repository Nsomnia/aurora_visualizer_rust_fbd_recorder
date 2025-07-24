//! Configuration settings management
//! Version: 0.1.0

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub audio: AudioSettings,
    pub display: DisplaySettings,
    pub presets: PresetSettings,
    pub recording: RecordingSettings,
    pub overlay: OverlaySettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioSettings {
    pub device: Option<String>,
    pub sample_rate: u32,
    pub buffer_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresetSettings {
    pub directory: PathBuf,
    pub shuffle: bool,
    pub duration: u32, // seconds
    pub favorites_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordingSettings {
    pub output_directory: PathBuf,
    pub format: String,
    pub quality: String,
    pub favorites_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OverlaySettings {
    pub enabled: bool,
    pub font_path: PathBuf,
    pub font_size: f32,
    pub position: (f32, f32),
}

impl Default for Settings {
    fn default() -> Self {
        let proj_dirs = ProjectDirs::from("org", "projectm-rs", "projectm-rs")
            .expect("Could not determine project directories");

        Self {
            audio: AudioSettings {
                device: None,
                sample_rate: 44100,
                buffer_size: 512,
            },
            display: DisplaySettings {
                width: 1280,
                height: 720,
                fullscreen: false,
                vsync: true,
            },
            presets: PresetSettings {
                directory: PathBuf::from("/usr/share/projectm/presets"),
                shuffle: false,
                duration: 15,
                favorites_only: false,
            },
            recording: RecordingSettings {
                output_directory: proj_dirs.data_dir().to_path_buf(),
                format: "mp4".to_string(),
                quality: "high".to_string(),
                favorites_only: true,
            },
            overlay: OverlaySettings {
                enabled: false,
                font_path: PathBuf::from("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
                font_size: 24.0,
                position: (10.0, 10.0),
            },
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("org", "projectm-rs", "projectm-rs")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directories"))?;
        
        let config_path = proj_dirs.config_dir().join("config.toml");
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let settings: Settings = toml::from_str(&content)?;
            Ok(settings)
        } else {
            let settings = Settings::default();
            Self::save(&settings, &config_path)?;
            Ok(settings)
        }
    }
    
    pub fn save(settings: &Self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(settings)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }
}
