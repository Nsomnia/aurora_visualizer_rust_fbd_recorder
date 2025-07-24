//! Main application window
//! Version: 0.1.2

use crate::config::Settings;
use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

pub struct UserInterface {
    _settings: Settings,
    sdl_context: sdl2::Sdl,
    _window: Window,
    event_pump: sdl2::EventPump,
}

impl UserInterface {
    pub fn new(settings: &Settings) -> Result<Self> {
        // Initialize SDL
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize SDL: {}", e))?;
        let video_subsystem = sdl_context.video()
            .map_err(|e| anyhow::anyhow!("Failed to initialize SDL video subsystem: {}", e))?;
        
        // Create window
        let window = video_subsystem
            .window(
                "ProjectM Rust Frontend",
                settings.display.width,
                settings.display.height,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create window: {}", e))?;
        
        let event_pump = sdl_context.event_pump()
            .map_err(|e| anyhow::anyhow!("Failed to create event pump: {}", e))?;
        
        Ok(Self {
            _settings: settings.clone(),
            sdl_context,
            _window: window,
            event_pump,
        })
    }
    
    pub fn process_events(&mut self) -> Result<bool> {
        // Process SDL events with a timeout to avoid blocking indefinitely
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(false); // Exit application
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    // Next preset
                    println!("Next preset requested");
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    // Previous preset
                    println!("Previous preset requested");
                }
                _ => {}
            }
        }
        
        Ok(true) // Continue running
    }
    
    pub fn render(&self) -> Result<()> {
        // In a real implementation, we would render here
        // For now, just return Ok
        Ok(())
    }
}