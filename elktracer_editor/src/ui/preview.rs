use bevy::{
    ecs::world::{Mut, World},
    log::info,
};
use bevy_egui::egui::{self, Button};
use elktracer_json::model::SceneModel;

use crate::render_tasks::{ElktracerRenderSystem, spawn_render_task};

pub fn ui_for_preview(
    ui: &mut egui::Ui,
    world: &mut World,
    scene_model: &SceneModel,
    render_options: &mut elktracer_core::RenderOptions,
) {
    let mut render_system: Mut<ElktracerRenderSystem> = world.resource_mut();
    let key = "Preview";

    let button = ui.add_enabled(
        !render_system.tasks.contains_key(key),
        Button::new("Render"),
    );
    if button.clicked() {
        let task = spawn_render_task(scene_model.clone(), *render_options);
        info!("Insert {} task", key);
        render_system.tasks.insert(key.to_string(), task);
    }

    if let Some(preview_texture) = render_system.texture_ids.get(key) {
        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            *preview_texture,
            [
                // render_options.image_width as f32,
                // (render_options.image_width as f64
                //     / render_options.aspect_ratio) as f32,
                ui.available_width(),
                ui.available_width() / (render_options.aspect_ratio as f32),
            ],
        )));
    }
}
