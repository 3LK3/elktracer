use bevy::prelude::*;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::ui_for_world;
use elktracer_json::model::SceneModel;

use crate::ui::{
    camera::ui_for_camera_options, material::ui_for_materials,
    preview::ui_for_preview, rendering::ui_for_rendering_options,
    scene_objects::ui_for_scene_objects,
};

#[derive(Debug)]
pub enum GuiWindow {
    SceneView,
    SceneObjects,
    Materials,
    Camera,
    Rendering,
    Preview,
    Debug,
}

pub struct Application<'a> {
    pub world: &'a mut World,
    pub viewport_rect: &'a mut egui::Rect,
    pub scene_model: &'a mut SceneModel,
    // pub camera: &'a mut elktracer_core::Camera,
    // pub materials: &'a mut Vec<MaterialModel>,
    // pub scene_objects: &'a mut Vec<ObjectModel>,
    pub render_options: &'a mut elktracer_core::RenderOptions,
    pub preview_render_options: &'a mut elktracer_core::RenderOptions,
}

impl egui_dock::TabViewer for Application<'_> {
    type Tab = GuiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        match window {
            GuiWindow::SceneView => {
                *self.viewport_rect = ui.clip_rect();
            }
            GuiWindow::SceneObjects => ui_for_scene_objects(
                ui,
                &self.scene_model.materials,
                &mut self.scene_model.objects,
            ),
            GuiWindow::Materials => {
                ui_for_materials(ui, &mut self.scene_model.materials)
            }
            GuiWindow::Camera => {
                ui_for_camera_options(ui, &mut self.scene_model.camera)
            }
            GuiWindow::Rendering => ui_for_rendering_options(
                ui,
                self.world,
                self.scene_model,
                self.render_options,
            ),
            GuiWindow::Debug => ui_for_world(self.world, ui),
            GuiWindow::Preview => ui_for_preview(
                ui,
                self.world,
                self.scene_model,
                self.preview_render_options,
            ),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, GuiWindow::SceneView)
    }
}
