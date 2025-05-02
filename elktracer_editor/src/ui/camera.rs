use bevy_egui::egui::{self, Grid};
use elktracer_json::model::CameraModel;

use super::{ui_for_number, ui_for_vector};

pub fn ui_for_camera_options(ui: &mut egui::Ui, camera: &mut CameraModel) {
    egui::Frame::new().outer_margin(6.0).show(ui, |ui| {
        Grid::new("vector_options_grid")
            .num_columns(2) // Define two columns: one for labels, one for widgets
            .spacing([10.0, 5.0]) // Horizontal and vertical spacing between items
            .show(ui, |ui| {
                ui_for_vector_attributes(ui, camera);
            });

        ui.add_space(10.0);

        Grid::new("number_options_grid")
            .num_columns(2) // Define two columns: one for labels, one for widgets
            .spacing([10.0, 5.0]) // Horizontal and vertical spacing between items
            .show(ui, |ui| {
                ui_for_number_attributes(ui, camera);
            });
    });
}

fn ui_for_vector_attributes(ui: &mut egui::Ui, camera: &mut CameraModel) {
    ui.label("Position");
    ui_for_vector(ui, &mut camera.position, 0.1, None);
    ui.end_row();

    ui.label("Look At");
    ui_for_vector(ui, &mut camera.look_at, 0.1, None);
    ui.end_row();

    ui.label("Up");
    ui_for_vector(ui, &mut camera.up, 0.1, None);
    ui.end_row();
}

fn ui_for_number_attributes(ui: &mut egui::Ui, camera: &mut CameraModel) {
    ui.label("Vertical field of view °");
    ui_for_number(ui, &mut camera.fov_vertical_degrees, 0.1, Some(0.0..=360.0));
    ui.end_row();

    ui.label("Defocus angle °");
    ui_for_number(ui, &mut camera.defocus_angle, 0.1, Some(0.0..=360.0));
    ui.end_row();

    ui.label("Focus distance");
    ui_for_number(ui, &mut camera.focus_distance, 0.1, Some(0.0..=f64::MAX));
    ui.end_row();
}
