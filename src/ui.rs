use eframe::egui;
use crate::main_app::MusicVisualizerApp;

pub fn draw_ui(ctx: &egui::Context, app: &mut MusicVisualizerApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Add Audio File(s)").clicked() {
                    if let Some(files) = rfd::FileDialog::new()
                        .add_filter("Audio", &["wav"])
                        .pick_files()
                    {
                        app.playback.add_files(files);
                    }
                }
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();
            app.playback.ui(ui);
            ui.separator();
            if ui.button("Previous Preset").clicked() {
                app.playlist.play_prev();
            }
            if ui.button("Next Preset").clicked() {
                app.playlist.play_next();
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        app.visualizer.ui(ui, &mut app.painter);
    });
}
