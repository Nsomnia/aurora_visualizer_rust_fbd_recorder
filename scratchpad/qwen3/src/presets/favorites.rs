//! Favorites management for presets
//! Version: 0.1.0

use anyhow::Result;
use std::path::PathBuf;

pub struct FavoritesManager {
    favorites: Vec<PathBuf>,
}

impl FavoritesManager {
    pub fn new() -> Result<Self> {
        // Load favorites from file
        
        Ok(Self {
            favorites: vec![],
        })
    }
    
    pub fn add_favorite(&mut self, preset: PathBuf) -> Result<()> {
        // Add preset to favorites
        
        if !self.favorites.contains(&preset) {
            self.favorites.push(preset);
        }
        
        Ok(())
    }
    
    pub fn remove_favorite(&mut self, preset: &PathBuf) -> Result<()> {
        // Remove preset from favorites
        
        self.favorites.retain(|p| p != preset);
        
        Ok(())
    }
    
    pub fn is_favorite(&self, preset: &PathBuf) -> bool {
        self.favorites.contains(preset)
    }
    
    pub fn get_favorites(&self) -> &Vec<PathBuf> {
        &self.favorites
    }
}
