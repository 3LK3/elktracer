use elkengine_core::{glow, imgui};

pub mod diagnostics;
pub mod render_options;

pub trait UiWindow {
    fn update(
        &mut self,
        delta_time: std::time::Duration,
        ui: &mut elkengine_core::imgui::Ui,
        glow_context: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
    );
}
