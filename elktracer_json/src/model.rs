use elktracer_core::{Camera, Vec3f};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SceneModel {
    pub camera: CameraModel,
    pub materials: Vec<MaterialModel>,
    pub objects: Vec<ObjectModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CameraModel {
    pub position: [f64; 3],
    pub look_at: [f64; 3],
    pub up: [f64; 3],

    pub fov_vertical_degrees: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

impl From<CameraModel> for Camera {
    fn from(value: CameraModel) -> Self {
        Camera {
            position: Vec3f::from(value.position),
            look_at: Vec3f::from(value.look_at),
            up: Vec3f::from(value.up),
            fov_vertical_degrees: value.fov_vertical_degrees,
            defocus_angle: value.defocus_angle,
            focus_distance: value.focus_distance,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ObjectType {
    Sphere { radius: f64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ObjectModel {
    pub id: String,
    pub position: [f64; 3],
    #[serde(rename = "material")]
    pub material_id: String,
    #[serde(rename = "object")]
    pub object_type: ObjectType,
}

impl ObjectModel {
    pub fn new(
        id: &str,
        position: [f64; 3],
        material_id: &str,
        object_type: ObjectType,
    ) -> Self {
        Self {
            id: id.to_string(),
            position,
            material_id: material_id.to_string(),
            object_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MaterialType {
    Lambert {
        albedo: [f32; 3],
    },
    Metal {
        albedo: [f32; 3],
        fuzziness: f64,
    },
    #[serde(rename_all = "kebab-case")]
    Transparent {
        refraction_index: f64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialModel {
    pub id: String,
    #[serde(rename = "material")]
    pub material_type: MaterialType,
}

impl MaterialModel {
    pub fn new(id: &str, material_type: MaterialType) -> Self {
        Self {
            id: id.to_string(),
            material_type,
        }
    }
}
