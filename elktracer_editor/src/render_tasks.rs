use bevy::{
    asset::RenderAssetUsages,
    platform::collections::HashMap,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};
use bevy_egui::{EguiContexts, egui::TextureId};
use elktracer_core::image_rs::RgbaImage;
use elktracer_json::model::SceneModel;

#[derive(Resource, Default)]
pub struct ElktracerRenderSystem {
    pub tasks: HashMap<String, Task<elktracer_core::Image>>,
    pub texture_ids: HashMap<String, TextureId>,
}

pub fn spawn_render_task(
    scene_model: SceneModel,
    // camera: elktracer_core::Camera,
    // materials: Vec<MaterialModel>,
    // scene_objects: Vec<ObjectModel>,
    render_options: elktracer_core::RenderOptions,
) -> Task<elktracer_core::Image> {
    let task_pool = AsyncComputeTaskPool::get();
    task_pool.spawn(async move {
        let mut raytracer = elktracer_core::Raytracer::new();

        // let mut core_materials: HashMap<
        //     String,
        //     Arc<dyn elktracer_core::Material>,
        // > = HashMap::new();

        // for material in materials.iter() {
        //     core_materials.insert(
        //         material.id.clone(),
        //         match material.material_type {
        //             crate::model::MaterialType::Lambert { albedo } => {
        //                 Arc::new(elktracer_core::LambertMaterial::new(
        //                     Color::from(albedo),
        //                 ))
        //             }
        //             crate::model::MaterialType::Metal { albedo, fuzziness } => {
        //                 Arc::new(elktracer_core::MetalMaterial::new(
        //                     Color::from(albedo),
        //                     fuzziness,
        //                 ))
        //             }
        //             crate::model::MaterialType::Transparent {
        //                 refraction_index,
        //             } => Arc::new(elktracer_core::TransparentMaterial::new(
        //                 refraction_index,
        //             )),
        //         },
        //     );
        // }

        // let objects: Vec<Box<dyn RayHitTest>> = scene_objects
        //     .iter()
        //     .map(|scene_object| match scene_object.object_type {
        //         crate::model::ObjectType::Sphere { radius } => {
        //             Box::new(elktracer_core::Sphere::new(
        //                 scene_object.position,
        //                 radius,
        //                 core_materials
        //                     .get(&scene_object.material_id)
        //                     .expect("Material not found")
        //                     .clone(),
        //             )) as Box<dyn RayHitTest>
        //         }
        //     })
        //     .collect();

        let camera = elktracer_core::Camera::from(scene_model.camera.clone());
        let objects = elktracer_json::get_scene_objects(&scene_model);

        raytracer.render_image(&camera, objects, &render_options)
    })
}

pub fn handle_elktracer_render_tasks(
    mut render_system: ResMut<ElktracerRenderSystem>,
    mut images: ResMut<Assets<Image>>,
    mut contexts: EguiContexts,
) {
    let mut finished_images: HashMap<String, Handle<Image>> = HashMap::new();

    render_system.tasks.retain(|task_id, task| {
        let status = block_on(future::poll_once(task));
        let should_retain_task = status.is_none();

        if let Some(image) = status {
            info!("Finished {}", task_id);

            let image_handle = images.add(Image::from_dynamic(
                elktracer_core::image_rs::DynamicImage::from(RgbaImage::from(
                    image,
                )),
                true,
                RenderAssetUsages::all(),
            ));
            finished_images.insert(task_id.clone(), image_handle);
        }

        should_retain_task
    });

    for (key, handle) in finished_images {
        render_system
            .texture_ids
            .insert(key, contexts.add_image(handle));
    }
}

// fn poll_task(
//     task: &mut Option<Task<elktracer_core::Image>>,
//     on_finished: fn(elktracer_core::Image),
// ) -> bool {
//     if let Some(running_task) = task {
//         let status = block_on(future::poll_once(running_task));
//         let should_retain_task = status.is_none();

//         if let Some(image) = status {
//             on_finished(image);
//         }

//         return should_retain_task;
//     }
//     false
// }
