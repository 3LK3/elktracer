pub mod camera;
pub mod material;
pub mod preview;
pub mod rendering;
pub mod scene_objects;

use std::ops::RangeInclusive;

use bevy_egui::egui::{self, Response, emath};

// pub struct NumberWidget<'a, T: emath::Numeric> {
//     value: &'a mut T,
//     range: Option<RangeInclusive<T>>,
// }

// impl<'a, T: emath::Numeric> NumberWidget<'a, T> {
//     pub fn new(value: &'a mut T, range: Option<RangeInclusive<T>>) -> Self {
//         Self { value, range }
//     }
// }

// impl<'a, T: emath::Numeric> Widget for NumberWidget<'a, T> {
//     fn ui(self, ui: &mut egui::Ui) -> Response {
//         let mut drag_value = egui::DragValue::new(self.value).speed(1.0);
//         if let Some(range) = self.range {
//             drag_value = drag_value.range(range);
//         }
//         ui.add(drag_value)
//     }
// }

pub fn ui_for_number<T: emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut T,
    speed: f64,
    range: Option<RangeInclusive<T>>,
) -> Response {
    let mut drag_value = egui::DragValue::new(value).speed(speed);
    if let Some(range) = range {
        drag_value = drag_value.range(range);
    }
    ui.add(drag_value)
}

pub fn ui_for_vector(
    ui: &mut egui::Ui,
    value: &mut [f64; 3],
    speed: f64,
    range: Option<RangeInclusive<f64>>,
) {
    ui.horizontal(|ui| {
        ui.label("X:");
        let mut x = value[0];
        if ui_for_number(ui, &mut x, speed, range.clone()).changed() {
            value[0] = x;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Y:");
        let mut y = value[1];
        if ui_for_number(ui, &mut y, speed, range.clone()).changed() {
            value[1] = y;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Z:");
        let mut z = value[2];
        if ui_for_number(ui, &mut z, speed, range.clone()).changed() {
            value[2] = z;
        }
    });
}

pub fn ui_for_string(
    ui: &mut egui::Ui,
    value: &mut String,
    placeholder: &str,
) -> Response {
    ui.add(egui::TextEdit::singleline(value).hint_text(placeholder))
}
