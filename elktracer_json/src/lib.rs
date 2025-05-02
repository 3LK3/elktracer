pub mod model;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
    sync::Arc,
};

use elktracer_core::{Color, RayHitTest, Vec3f};
use model::SceneModel;

pub fn get_scene_objects(scene: &SceneModel) -> Vec<Box<dyn RayHitTest>> {
    let mut core_materials: HashMap<String, Arc<dyn elktracer_core::Material>> =
        HashMap::new();

    for material in scene.materials.iter() {
        core_materials.insert(
            material.id.clone(),
            match material.material_type {
                crate::model::MaterialType::Lambert { albedo } => Arc::new(
                    elktracer_core::LambertMaterial::new(Color::from(albedo)),
                ),
                crate::model::MaterialType::Metal { albedo, fuzziness } => {
                    Arc::new(elktracer_core::MetalMaterial::new(
                        Color::from(albedo),
                        fuzziness,
                    ))
                }
                crate::model::MaterialType::Transparent {
                    refraction_index,
                } => Arc::new(elktracer_core::TransparentMaterial::new(
                    refraction_index,
                )),
            },
        );
    }

    scene
        .objects
        .iter()
        .map(|scene_object| match scene_object.object_type {
            crate::model::ObjectType::Sphere { radius } => {
                Box::new(elktracer_core::Sphere::new(
                    Vec3f::from(scene_object.position),
                    radius,
                    core_materials
                        .get(&scene_object.material_id)
                        .expect("Material not found")
                        .clone(),
                )) as Box<dyn RayHitTest>
            }
        })
        .collect()
}

pub fn load_scene_model(file_path: &PathBuf) -> SceneModel {
    let file = File::open(file_path).expect("Unable to open scene file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse scene json")
}

pub fn save_scene_model(file_path: &PathBuf, scene_model: &SceneModel) {
    let json_data = serde_json::to_string(scene_model)
        .expect("Failed to serialize scene model");
    let mut file =
        File::create(file_path).expect("Failed to create scene file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write JSON to file");
}
