//! Main entry point for the ProjectM Rust frontend
//! Version: 0.1.11

mod cli;
mod config;
mod core;
mod ui;
mod recording;
mod presets;
mod overlay;
mod playlist;
mod utils;
mod speech;

use clap::Parser;
use cli::Cli;
use config::Settings;
use std::thread;
use std::time::Duration;
use utils::debug::DebugLogger;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize debug logger if requested
    if cli.debug {
        DebugLogger::init()?;
    }
    
    println!("Starting ProjectM Rust frontend...");
    println!("Press Ctrl+C to exit");
    
    // Load configuration
    let settings = Settings::load()?;
    println!("Configuration loaded");
    
    // Initialize core components
    let projectm_core = core::ProjectMCore::new(&settings)?;
    println!("ProjectM core initialized");
    
    let mut audio_engine = core::audio_engine::AudioEngine::new(&settings)?;
    println!("Audio engine initialized");
    
    // Initialize UI
    let mut ui = ui::UserInterface::new(&settings)?;
    println!("UI initialized");
    
    // Initialize recording system
    let mut recorder = recording::Recorder::new(&settings)?;
    println!("Recorder initialized");
    
    // Initialize preset manager
    let _preset_manager = presets::PresetManager::new(&settings)?;
    println!("Preset manager initialized");
    
    // Initialize overlay system
    let _overlay_system = overlay::OverlaySystem::new(&settings)?;
    println!("Overlay system initialized");
    
    // Initialize playlist
    let _playlist = playlist::PlaylistManager::new(&settings)?;
    println!("Playlist manager initialized");
    
    // Start audio capture
    audio_engine.start_capture()?;
    println!("Audio capture started");
    
    // Main application loop
    let mut frame_count = 0;
    let frame_duration = Duration::from_millis(16); // ~60 FPS
    
    println!("Entering main loop...");
    
    loop {
        // Process events
        let should_continue = ui.process_events()?;
        if !should_continue {
            println!("Exiting application");
            break;
        }
        
        // Get audio data
        let audio_data = audio_engine.get_audio_data()?;
        
        // Feed audio data to ProjectM
        projectm_core.feed_audio_data(&audio_data, 0)?; // Channel 0 (left)
        projectm_core.feed_audio_data(&audio_data, 1)?; // Channel 1 (right)
        
        // Render visualization
        projectm_core.render()?;
        ui.render()?;
        
        // Handle recording if active
        if recorder.is_recording() {
            recorder.capture_frame()?;
        }
        
        // Add a simple frame counter for debugging (less frequent now)
        frame_count += 1;
        if frame_count % 600 == 0 { // Every 10 seconds at 60 FPS
            println!("Rendered {} frames", frame_count);
        }
        
        // Limit frame rate
        thread::sleep(frame_duration);
    }
    
    Ok(())
}