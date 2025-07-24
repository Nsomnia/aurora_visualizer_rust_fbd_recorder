//! Preset manager for ProjectM
//! Version: 0.1.0

use crate::config::Settings;
use anyhow::Result;
use std::path::PathBuf;

pub struct PresetManager {
    _settings: Settings,
    presets: Vec<PathBuf>,
    current_index: usize,
}

impl PresetManager {
    pub fn new(settings: &Settings) -> Result<Self> {
        let mut manager = Self {
            _settings: settings.clone(),
            presets: vec![],
            current_index: 0,
        };
        
        manager.load_presets()?;
        
        Ok(manager)
    }
    
    fn load_presets(&mut self) -> Result<()> {
        // Load presets from directory
        // This would involve scanning the preset directory recursively
        
        Ok(())
    }
    
    pub fn next_preset(&mut self) -> Result<()> {
        // Switch to next preset
        
        if !self.presets.is_empty() {
            self.current_index = (self.current_index + 1) % self.presets.len();
        }
        
        Ok(())
    }
    
    pub fn previous_preset(&mut self) -> Result<()> {
        // Switch to previous preset
        
        if !self.presets.is_empty() {
            if self.current_index > 0 {
                self.current_index -= 1;
            } else {
                self.current_index = self.presets.len() - 1;
            }
        }
        
        Ok(())
    }
    
    pub fn shuffle_enabled(&self) -> bool {
        self._settings.presets.shuffle
    }
    
    pub fn toggle_shuffle(&mut self) {
        // Toggle shuffle mode
    }
    
    pub fn get_current_preset(&self) -> Option<&PathBuf> {
        self.presets.get(self.current_index)
    }
}
