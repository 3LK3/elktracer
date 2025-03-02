pub mod ui;

use std::time::Duration;

use elkengine_core::{Result, application::layer::Layer};
use elkengine_core::{glow, imgui};
use ui::windows::UiWindow;
use ui::windows::diagnostics::DiagnosticsWindow;
use ui::windows::render_options::RenderOptionsWindow;

struct ElktracerLayer {
    diagnostics_window: DiagnosticsWindow,
    render_window: RenderOptionsWindow,
    last_delta_time: std::time::Duration,
}

impl ElktracerLayer {
    pub fn new() -> Self {
        Self {
            diagnostics_window: DiagnosticsWindow::new(),
            render_window: RenderOptionsWindow::new(),
            last_delta_time: Duration::ZERO,
        }
    }
}

impl Layer for ElktracerLayer {
    fn update(&mut self, _delta_time: std::time::Duration) {
        self.last_delta_time = _delta_time;
    }

    fn update_imgui(
        &mut self,
        ui: &mut imgui::Ui,
        glow_context: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
    ) {
        self.diagnostics_window.update(
            self.last_delta_time,
            ui,
            glow_context,
            textures,
        );
        self.render_window.update(
            self.last_delta_time,
            ui,
            glow_context,
            textures,
        );
    }

    fn on_attached(&self) {
        log::info!("ElktracerLayer :: attached");
    }

    fn on_detached(&self) {
        log::info!("ElktracerLayer :: detached");
    }
}

fn main() -> Result<()> {
    elktracer_core::logging::initialize();
    log::info!("Starting Elktracer Editor");

    let mut application = elkengine_core::application::Application::new();

    let layer = Box::new(ElktracerLayer::new());
    application.add_layer(layer);

    application.run()
}
