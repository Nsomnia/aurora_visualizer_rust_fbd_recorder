use std::sync::Arc;

use egui::{PaintCallback, Vec2};
use egui_glow::{
    glow::{
        self, HasContext, FRAMEBUFFER, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, NEAREST,
    },
    CallbackFn, Painter,
};
use projectm::core::ProjectM;

pub struct ProjectMVisualizer {
    projectm: Arc<ProjectM>,
    texture: Option<glow::Texture>,
    fbo: Option<glow::Framebuffer>,
    size: Vec2,
}

impl ProjectMVisualizer {
    pub fn new(projectm: Arc<ProjectM>) -> Self {
        Self {
            projectm,
            texture: None,
            fbo: None,
            size: Vec2::ZERO,
        }
    }

    fn recreate_fbo(&mut self, painter: &Painter, size: Vec2) {
        unsafe {
            let gl = painter.gl();
            if self.texture.is_none() {
                self.texture = Some(gl.create_texture().unwrap());
            }
            gl.bind_texture(TEXTURE_2D, self.texture);
            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                glow::RGBA as i32,
                size.x as i32,
                size.y as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);

            if self.fbo.is_none() {
                self.fbo = Some(gl.create_framebuffer().unwrap());
            }
            gl.bind_framebuffer(FRAMEBUFFER, self.fbo);
            gl.framebuffer_texture_2d(
                FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                TEXTURE_2D,
                self.texture,
                0,
            );
            gl.bind_framebuffer(FRAMEBUFFER, None);

            self.size = size;
            self.projectm
                .set_window_size(size.x as usize, size.y as usize);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, painter: &mut Painter) {
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        let size = rect.size();

        if self.size != size {
            self.recreate_fbo(painter, size);
        }

        let fbo = self.fbo.unwrap();
        let texture = self.texture.unwrap();
        let projectm = self.projectm.clone();

        let callback = PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |_info, painter| {
                unsafe {
                    let gl = painter.gl();
                    // TODO: Save/restore viewport and other GL state if needed
                    gl.bind_framebuffer(FRAMEBUFFER, Some(fbo));
                    projectm.render_frame();
                    gl.bind_framebuffer(FRAMEBUFFER, None);
                }
            })),
        };
        ui.painter().add(callback);

        let texture_id = painter.register_native_texture(texture);
        ui.image(egui::ImageSource::Texture(egui::load::SizedTexture::new(
            texture_id,
            size,
        )));
    }
}