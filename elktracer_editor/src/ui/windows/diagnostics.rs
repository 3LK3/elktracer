use elkengine_core::imgui;

use super::UiWindow;

pub struct DiagnosticsWindow {
    current_time_seconds: f64,
    frame_count: u32,
    frames_last_second: u32,
}

impl DiagnosticsWindow {
    pub fn new() -> Self {
        Self {
            current_time_seconds: 0.0,
            frame_count: 0,
            frames_last_second: 0,
        }
    }
}

impl Default for DiagnosticsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindow for DiagnosticsWindow {
    fn update(
        &mut self,
        delta_time: std::time::Duration,
        ui: &mut elkengine_core::imgui::Ui,
    ) {
        self.frame_count += 1;
        self.current_time_seconds += delta_time.as_secs_f64();

        if self.current_time_seconds >= 1.0 {
            self.frames_last_second = self.frame_count;
            // log::trace!("Frames last second: {}", self.frames_last_second);

            self.frame_count = 0;
            self.current_time_seconds -= 1.0;
        }

        ui.window("Diagnostics")
            .size([200.0, 60.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped(format!(
                    "Frame per sec.: {}",
                    self.frames_last_second
                ));
            });
    }
}
