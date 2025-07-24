mod config;
mod playback;
mod preset_blocklist;
mod projectm_widget;
mod ui;
mod main_app;

use eframe::{egui, App};
use main_app::MusicVisualizerApp;

impl App for MusicVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        ui::draw_ui(ctx, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.painter.destroy();
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let config = config::load_config();

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