use elkengine_core::imgui;
use elktracer_core::{
    math::vector3::Vec3f,
    raytracer::{CameraRenderOptions, Raytracer},
};

use super::UiWindow;

pub struct RenderOptionsWindow {
    raytracer: Box<Raytracer>,
    camera_options: CameraRenderOptions,
}

impl RenderOptionsWindow {
    pub fn new() -> Self {
        Self {
            raytracer: Box::new(Raytracer::new()),
            camera_options: CameraRenderOptions {
                aspect_ratio: 16.0 / 9.0,
                image_width: 600,
                position: Vec3f::new(12.0, 2.0, 3.0),
                look_at: Vec3f::new(0.0, 0.0, 0.0),
                up: Vec3f::new(0.0, 1.0, 0.0),
                fov_vertical_degrees: 15.0,
                defocus_angle: 0.6,
                focus_distance: 10.0,
            },
        }
    }
}

impl Default for RenderOptionsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindow for RenderOptionsWindow {
    fn update(
        &mut self,
        _delta_time: std::time::Duration,
        ui: &mut elkengine_core::imgui::Ui,
    ) {
        ui.window("Render_Options")
            .size([300.0, 200.0], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("Generate image") {
                    log::info!("Generating image ...");

                    let _image = self.raytracer.render_image(
                        &self.camera_options,
                        20,
                        20,
                    );
                }
            });
    }
}
