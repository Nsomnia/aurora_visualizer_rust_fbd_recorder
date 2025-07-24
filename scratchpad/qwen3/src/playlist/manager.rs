//! Playlist manager for audio files
//! Version: 0.1.0

use crate::config::Settings;
use anyhow::Result;
use std::path::PathBuf;

pub struct PlaylistManager {
    _settings: Settings,
    files: Vec<PathBuf>,
    current_index: usize,
}

impl PlaylistManager {
    pub fn new(settings: &Settings) -> Result<Self> {
        // Initialize playlist manager
        
        Ok(Self {
            _settings: settings.clone(),
            files: vec![],
            current_index: 0,
        })
    }
    
    pub fn add_file(&mut self, file: PathBuf) -> Result<()> {
        // Add file to playlist
        
        self.files.push(file);
        Ok(())
    }
    
    pub fn remove_file(&mut self, index: usize) -> Result<()> {
        // Remove file from playlist
        
        if index < self.files.len() {
            self.files.remove(index);
            if self.current_index >= self.files.len() && !self.files.is_empty() {
                self.current_index = self.files.len() - 1;
            }
        }
        
        Ok(())
    }
    
    pub fn next_track(&mut self) -> Result<()> {
        // Move to next track
        
        if !self.files.is_empty() {
            self.current_index = (self.current_index + 1) % self.files.len();
        }
        
        Ok(())
    }
    
    pub fn previous_track(&mut self) -> Result<()> {
        // Move to previous track
        
        if !self.files.is_empty() {
            if self.current_index > 0 {
                self.current_index -= 1;
            } else {
                self.current_index = self.files.len() - 1;
            }
        }
        
        Ok(())
    }
    
    pub fn get_current_file(&self) -> Option<&PathBuf> {
        self.files.get(self.current_index)
    }
}
