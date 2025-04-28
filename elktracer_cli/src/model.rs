use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneModel {
    pub camera: CameraModel,
    pub materials: HashMap<String, MaterialType>,
    pub objects: Vec<ObjectModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectModel {
    pub name: String,
    pub position: [f64; 3],
    pub material: String,
    pub object: ObjectType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ObjectType {
    Sphere { radius: f64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum MaterialType {
    Lambert {
        albedo: [f64; 3],
    },
    Metal {
        albedo: [f64; 3],
        fuzziness: f64,
    },
    #[serde(rename_all = "kebab-case")]
    Transparent {
        refraction_index: f64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CameraModel {
    pub position: [f64; 3],
    pub look_at: [f64; 3],
    pub up: [f64; 3],

    pub fov_vertical_degrees: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}
