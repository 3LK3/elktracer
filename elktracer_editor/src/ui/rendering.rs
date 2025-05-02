use bevy::prelude::*;
use bevy_egui::egui::{self, DragValue, Grid};
use elktracer_json::model::SceneModel;

use crate::render_tasks::{ElktracerRenderSystem, spawn_render_task};

const NUMBER_INPUT_SIZE: [f32; 2] = [60.0, 18.0];

pub fn ui_for_rendering_options(
    ui: &mut egui::Ui,
    world: &mut World,
    scene_model: &SceneModel,
    render_options: &mut elktracer_core::RenderOptions,
) {
    let mut render_system: Mut<ElktracerRenderSystem> = world.resource_mut();
    let key = "Rendering";

    ui.add_enabled_ui(!render_system.tasks.contains_key(key), |ui| {
        Grid::new("rendering_options_grid")
            .num_columns(2) // Define two columns: one for labels, one for widgets
            .spacing([10.0, 5.0]) // Horizontal and vertical spacing between items
            .show(ui, |ui| {
                ui.label("Image Width:");
                ui.add_sized(
                    NUMBER_INPUT_SIZE,
                    DragValue::new(&mut render_options.image_width)
                        .range(0..=u32::MAX),
                );
                ui.end_row();

                ui.label("Aspect Ratio:");
                ui.add_sized(
                    NUMBER_INPUT_SIZE,
                    DragValue::new(&mut render_options.aspect_ratio)
                        .range(0.0..=f64::MAX),
                );
                ui.end_row();

                ui.label("Samples Per Pixel:");
                ui.add_sized(
                    NUMBER_INPUT_SIZE,
                    DragValue::new(&mut render_options.samples_per_pixel)
                        .range(0..=u16::MAX),
                );
                ui.end_row();

                ui.label("Max. Ray Depth:");
                ui.add_sized(
                    NUMBER_INPUT_SIZE,
                    DragValue::new(&mut render_options.max_ray_depth)
                        .range(0..=u16::MAX),
                );
                ui.end_row();

                if ui
                    .add_sized(
                        egui::Vec2::new(ui.available_width(), 18.0),
                        egui::Button::new("Render"),
                    )
                    .clicked()
                {
                    let task =
                        spawn_render_task(scene_model.clone(), *render_options);
                    info!("Insert {} task", key);
                    render_system.tasks.insert(key.to_string(), task);
                }
                ui.end_row();
            });
    });

    if let Some(preview_texture) = render_system.texture_ids.get(key) {
        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            *preview_texture,
            [
                // ui.available_width(),
                // ui.available_width() / (render_options.aspect_ratio as f32),
                render_options.image_width as f32,
                (render_options.image_width as f64
                    / render_options.aspect_ratio) as f32,
            ],
        )));
    }
}
