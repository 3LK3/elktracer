use bevy_egui::egui::{self, Button, Color32, Grid, Stroke};
use elktracer_json::model::{MaterialModel, MaterialType};

use super::{ui_for_number, ui_for_string};

pub fn ui_for_materials(ui: &mut egui::Ui, materials: &mut Vec<MaterialModel>) {
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.add_space(6.0);

        if ui.button("Add Lambert").clicked() {
            materials.push(MaterialModel {
                id: "New Lambert Material".to_string(),
                material_type: MaterialType::Lambert {
                    albedo: [1.0, 1.0, 1.0],
                },
            });
        }
        if ui.button("Add Metal").clicked() {
            materials.push(MaterialModel {
                id: "New Metal Material".to_string(),
                material_type: MaterialType::Metal {
                    albedo: [1.0, 1.0, 1.0],
                    fuzziness: 1.0,
                },
            });
        }
        if ui.button("Add Transparent").clicked() {
            materials.push(MaterialModel {
                id: "New Transparent Material".to_string(),
                material_type: MaterialType::Transparent {
                    refraction_index: 1.0,
                },
            });
        }
    });

    let mut to_be_removed: Option<usize> = None;

    for (index, material) in materials.iter_mut().enumerate() {
        let mut remove = false;
        ui_for_list_item(ui, index, material, &mut remove);
        if remove {
            to_be_removed = Some(index);
        }
    }

    if let Some(remove) = to_be_removed {
        materials.remove(remove);
    }
}

fn ui_for_list_item(
    ui: &mut egui::Ui,
    index: usize,
    material: &mut MaterialModel,
    should_be_removed: &mut bool,
) {
    egui::Frame::new()
        .inner_margin(6.0)
        .outer_margin(6.0)
        .stroke(Stroke::new(1.0, Color32::from_rgb(150, 150, 150)))
        .corner_radius(2.0)
        .show(ui, |ui| {
            // Set a fixed width
            let frame_width = 200.0; // Set the desired fixed width
            ui.allocate_ui_with_layout(
                egui::Vec2::new(frame_width, ui.available_size().y),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.horizontal(|ui| {
                        match material.material_type {
                            MaterialType::Lambert { .. } => {
                                ui.label("Lambert");
                            }
                            MaterialType::Metal { .. } => {
                                ui.label("Metal");
                            }
                            MaterialType::Transparent { .. } => {
                                ui.label("Transparent");
                            }
                        }

                        ui_for_string(ui, &mut material.id, "Material ID");

                        if ui.add(Button::new("Delete")).clicked() {
                            *should_be_removed = true;
                        }
                    });

                    ui.add_space(4.0);

                    ui.vertical(|ui| {
                        Grid::new(format!("material_grid_{}", index))
                            .num_columns(2) // Define two columns: one for labels, one for widgets
                            .spacing([10.0, 5.0]) // Horizontal and vertical spacing between items
                            .show(ui, |ui| {
                                ui_for_material_type(ui, material);
                            });
                    });
                },
            );
        });
}

fn ui_for_material_type(ui: &mut egui::Ui, material: &mut MaterialModel) {
    match &mut material.material_type {
        MaterialType::Lambert { albedo } => {
            ui.label("Albedo:");
            ui.color_edit_button_rgb(albedo);
            ui.end_row();
        }
        MaterialType::Metal { albedo, fuzziness } => {
            ui.label("Albedo:");
            ui.color_edit_button_rgb(albedo);
            ui.end_row();

            ui.label("Fuzziness");
            ui_for_number(ui, fuzziness, 0.1, None);
            ui.end_row();
        }
        MaterialType::Transparent { refraction_index } => {
            ui.label("Refraction index:");
            ui_for_number(ui, refraction_index, 0.1, None);
            ui.end_row();
        }
    };
}
