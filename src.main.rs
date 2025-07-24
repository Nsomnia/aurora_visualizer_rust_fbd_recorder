mod config;
mod projectm_widget;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

use eframe::{egui, App, CreationContext};
use egui::{CentralPanel, Vec2};
use egui_glow::Painter;
use projectm::core::ProjectM;
use projectm::playlist::Playlist;
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};

use crate::config::Config;
use crate::projectm_widget::ProjectMVisualizer;

struct MusicVisualizerApp {
    path: Option<PathBuf>,
    _stream: Option<OutputStream>,
    sink: Option<Arc<Sink>>,
    projectm: Arc<ProjectM>,
    playlist: Playlist,
    visualizer: ProjectMVisualizer,
    painter: Painter,
    source: Option<Decoder<BufReader<File>>>,
}

impl MusicVisualizerApp {
    fn new(cc: &CreationContext, config: &Config) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let projectm = Arc::new(ProjectM::create());
        projectm.set_window_size(config.width as usize, config.height as usize);
        let playlist = Playlist::create(&projectm);

        playlist.add_path(config.preset_path.to_str().unwrap(), true);

        let visualizer = ProjectMVisualizer::new(projectm.clone());
        let painter = Painter::new(cc.gl.clone().unwrap(), "", None).unwrap();

        Self {
            path: None,
            _stream: Some(_stream),
            sink: Some(Arc::new(sink)),
            projectm,
            playlist,
            visualizer,
            painter,
            source: None,
        }
    }
}

impl App for MusicVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(source) = &mut self.source {
            let samples: Vec<i16> = source.by_ref().take(512).collect();
            if !samples.is_empty() {
                self.projectm.pcm_add_int16(&samples, 2);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg"])
                            .pick_file()
                        {
                            self.path = Some(path);
                        }
                    }
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.visualizer.ui(ui, &mut self.painter);

            ui.heading("Music Visualizer");
            ui.separator();

            if let Some(path) = &self.path {
                ui.label(format!("Current file: {}", path.display()));
                if ui.button("Play").clicked() {
                    if let Some(sink) = &self.sink {
                        sink.stop();
                        if let Ok(file) = File::open(path.clone()) {
                            let source = Decoder::new(BufReader::new(file)).unwrap();
                            self.source = Some(source);
                        }
                        if let Ok(file) = File::open(path.clone()) {
                            let source = Decoder::new(BufReader::new(file)).unwrap();
                            sink.append(source);
                            sink.play();
                        }
                    }
                }

                if ui.button("Stop").clicked() {
                    if let Some(sink) = &self.sink {
                        sink.stop();
                    }
                }
            }

            ui.separator();

            if ui.button("Previous Preset").clicked() {
                self.playlist.play_prev();
            }

            if ui.button("Next Preset").clicked() {
                self.playlist.play_next();
            }
        });
    }

    fn on_close_event(&mut self) -> bool {
        self.painter.destroy();
        true
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let config = Config::default();

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.width as f32, config.height as f32])
            .with_title("Music Visualizer"),
        ..Default::default()
    };

    eframe::run_native(
        "Music Visualizer",
        options,
        Box::new(move |cc| Box::new(MusicVisualizerApp::new(cc, &config))),
    )
}
