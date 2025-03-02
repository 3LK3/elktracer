pub mod diagnostics;
pub mod render_options;

pub trait UiWindow {
    fn update(
        &mut self,
        delta_time: std::time::Duration,
        ui: &mut elkengine_core::imgui::Ui,
    );
}
