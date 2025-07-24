use std::sync::{Arc, Mutex};

use egui_glow::Painter;
use projectm::core::ProjectM;
use projectm::playlist::Playlist;

use crate::config;
use crate::playback::Playback;
use crate::preset_blocklist::PresetBlocklist;
use crate::projectm_widget::ProjectMVisualizer;

pub struct MusicVisualizerApp {
    pub projectm: Arc<ProjectM>,
    pub playlist: Playlist,
    pub visualizer: ProjectMVisualizer,
    pub painter: Painter,
    pub playback: Playback,
    pub preset_blocklist: Arc<Mutex<PresetBlocklist>>,
}

impl MusicVisualizerApp {
    pub fn new(cc: &eframe::CreationContext, config: &config::Config) -> Self {
        let projectm = Arc::new(ProjectM::create());
        projectm.set_window_size(config.width as usize, config.height as usize);
        let playlist = Playlist::create(&projectm);

        let preset_blocklist = Arc::new(Mutex::new(PresetBlocklist::new()));
        let blocklist_clone = preset_blocklist.clone();

        projectm.set_preset_switch_failed_event_callback(move |preset_filename, message| {
            if message.contains("failed to find texture") {
                println!(
                    "Blocking preset due to missing texture: {}",
                    preset_filename
                );
                blocklist_clone.lock().unwrap().add(&preset_filename);
            }
        });

        playlist.add_path(config.preset_path.to_str().unwrap(), true);

        let visualizer = ProjectMVisualizer::new(projectm.clone());
        let painter = Painter::new(cc.gl.clone().unwrap(), "", None).unwrap();
        let playback = Playback::new(projectm.clone());

        Self {
            projectm,
            playlist,
            visualizer,
            painter,
            playback,
            preset_blocklist,
        }
    }
}
