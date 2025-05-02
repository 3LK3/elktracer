use bevy_egui::egui::{self, Button, Color32, Grid, Stroke};
use elktracer_json::model::{MaterialModel, ObjectModel, ObjectType};

use super::{ui_for_number, ui_for_string, ui_for_vector};

pub fn ui_for_scene_objects(
    ui: &mut egui::Ui,
    materials: &Vec<MaterialModel>,
    scene_objects: &mut Vec<ObjectModel>,
) {
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.add_space(6.0);

        if ui.button("Add Sphere").clicked() {
            let mut material_id = String::new();
            if let Some(first) = materials.first() {
                material_id = first.id.clone();
            }

            scene_objects.push(ObjectModel {
                id: "New Sphere".to_string(),
                position: [0.0, 0.0, 0.0],
                material_id,
                object_type: ObjectType::Sphere { radius: 1.0 },
            });
        }
    });

    let mut to_be_removed: Option<usize> = None;

    for (index, scene_object) in scene_objects.iter_mut().enumerate() {
        let mut remove = false;

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
                        ui_for_object(
                            ui,
                            index,
                            scene_object,
                            &mut remove,
                            materials,
                        );
                    },
                );
            });

        if remove {
            to_be_removed = Some(index);
        }
    }

    if let Some(remove) = to_be_removed {
        scene_objects.remove(remove);
    }
}

fn ui_for_object(
    ui: &mut egui::Ui,
    index: usize,
    scene_object: &mut ObjectModel,
    should_be_removed: &mut bool,
    materials: &Vec<MaterialModel>,
) {
    ui.horizontal(|ui| {
        match scene_object.object_type {
            ObjectType::Sphere { .. } => {
                ui.label("Sphere");
            }
        };

        ui_for_string(ui, &mut scene_object.id, "Scene Object ID");

        if ui.add(Button::new("Delete")).clicked() {
            *should_be_removed = true;
        }
    });

    ui.add_space(4.0);

    ui.vertical(|ui| {
        Grid::new(format!("scene_object_grid_{}", index))
            .num_columns(2) // Define two columns: one for labels, one for widgets
            .spacing([10.0, 5.0]) // Horizontal and vertical spacing between items
            .show(ui, |ui| {
                ui.label("Position:");
                ui.horizontal(|ui| {
                    ui_for_vector(ui, &mut scene_object.position, 0.1, None);
                });
                ui.end_row();

                ui.label("Material:");
                material_combo_box(
                    ui,
                    index,
                    materials,
                    &mut scene_object.material_id,
                );
                ui.end_row();

                ui_for_object_type(ui, scene_object);
            });
    });
}

fn material_combo_box(
    ui: &mut egui::Ui,
    index: usize,
    materials: &Vec<MaterialModel>,
    selected_id: &mut String,
) {
    egui::ComboBox::new(format!("material_combo_box_{}", index), "")
        .selected_text(format!("{:?}", selected_id))
        .show_ui(ui, |ui| {
            for material in materials {
                ui.selectable_value(
                    selected_id,
                    material.id.clone(),
                    &material.id,
                );
            }
        });
}

fn ui_for_object_type(ui: &mut egui::Ui, scene_object: &mut ObjectModel) {
    match &mut scene_object.object_type {
        ObjectType::Sphere { radius } => {
            ui.label("Radius:");
            ui_for_number(ui, radius, 0.1, None);
            ui.end_row();
        }
    }
}
