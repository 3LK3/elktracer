mod error;
mod model;

use self::error::{Error, Result};
use self::model::SceneModel;

use std::collections::HashMap;
use std::sync::Arc;
use std::{fs::File, io::BufReader};

use clap::{Parser, Subcommand};
use elktracer_core::{
    Color, LambertMaterial, Material, MetalMaterial, RayHitTest, Sphere,
    TransparentMaterial, Vec3f,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Renders a scene
    Render {
        #[arg(long, short = 'f', value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        scene_file: std::path::PathBuf,
        #[arg(long, short = 'o', value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        output_file: Option<std::path::PathBuf>,
        #[arg(long, short = 'w', value_name = "WIDTH")]
        image_width: u32,
        #[arg(long, short = 'a', value_name = "ASPECT_RATIO")]
        aspect_ratio: f64,
        #[arg(long, short = 's', value_name = "ASPECT_RATIO")]
        samples_per_pixel: u16,
        #[arg(long, short = 'r', value_name = "ASPECT_RATIO")]
        max_ray_depth: u16,
    },
}

fn main() -> Result<()> {
    elktracer_core::logging::initialize();

    let cli = Args::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Render {
            scene_file,
            output_file,
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_ray_depth,
        } => {
            if !scene_file.exists() {
                log::error!("Scene file does not exist: {:?}", scene_file);
                return Err(Error::SceneFileNotFound(
                    scene_file.display().to_string(),
                ));
            }

            let file =
                File::open(scene_file).expect("Unable to open scene file");
            let reader = BufReader::new(file);
            let scene: SceneModel = serde_json::from_reader(reader)
                .expect("Unable to parse scene json");
            log::trace!("Parsed scene: {:?}", scene);

            let camera = elktracer_core::Camera::new(
                Vec3f::from(scene.camera.position),
                Vec3f::from(scene.camera.look_at),
                Vec3f::from(scene.camera.up),
                scene.camera.fov_vertical_degrees,
                scene.camera.defocus_angle,
                scene.camera.focus_distance,
            );

            let mut materials: HashMap<String, Arc<dyn Material>> =
                HashMap::new();
            for (name, material_type) in scene.materials {
                materials.insert(
                    name,
                    match material_type {
                        model::MaterialType::Lambert { albedo } => {
                            Arc::new(LambertMaterial::new(Color::from(albedo)))
                        }
                        model::MaterialType::Metal { albedo, fuzziness } => {
                            Arc::new(MetalMaterial::new(
                                Color::from(albedo),
                                fuzziness,
                            ))
                        }
                        model::MaterialType::Transparent {
                            refraction_index,
                        } => {
                            Arc::new(TransparentMaterial::new(refraction_index))
                        }
                    },
                );
            }

            let mut objects: Vec<Box<dyn RayHitTest>> = vec![];
            for object in scene.objects {
                objects.push(match object.object {
                    model::ObjectType::Sphere { radius } => {
                        Box::new(Sphere::new(
                            Vec3f::from(object.position),
                            radius,
                            materials
                                .get(&object.material)
                                .expect("Material not found")
                                .clone(),
                        ))
                    }
                });
            }

            let mut raytracer = elktracer_core::Raytracer::new();
            let image = raytracer.render_image(
                &camera,
                objects,
                elktracer_core::RenderOptions::new(
                    *image_width,
                    *aspect_ratio,
                    *samples_per_pixel,
                    *max_ray_depth,
                ),
            );

            let mut output = std::path::PathBuf::from("out.png");
            if let Some(file) = output_file {
                output = file.clone();
            }

            let _ = elktracer_core::save_to_file(
                &image,
                output,
                elktracer_core::image_rs::ImageFormat::Png,
            );
        }
    }

    Ok(())

    // let material_left = LambertMaterial::new(Color::new(0.1, 0.1, 0.9));
    // let material_right = LambertMaterial::new(Color::new(0.9, 0.1, 0.1));

    // let radius = f64::cos(PI / 4.0);

    // raytracer.add_scene_object(Sphere::new(
    //     Vec3f::new(-radius, 0.0, -1.0),
    //     radius,
    //     Box::new(material_left),
    // ));

    // raytracer.add_scene_object(Sphere::new(
    //     Vec3f::new(radius, 0.0, -1.0),
    //     radius,
    //     Box::new(material_right),
    // ));

    // let material_ground = LambertMaterial::new(Color::new(0.8, 0.8, 0.0));
    // let material_center = LambertMaterial::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = TransparentMaterial::new(1.5);
    // let material_bubble = TransparentMaterial::new(1.0 / 1.5);
    // let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // let scene = Scene::new(
    //     Camera {
    //         position: Vec3f::new(12.0, 2.0, 3.0),
    //         look_at: Vec3f::new(0.0, 0.0, 0.0),
    //         up: Vec3f::new(0.0, 1.0, 0.0),
    //         fov_vertical_degrees: 15.0,
    //         defocus_angle: 0.6,
    //         focus_distance: 10.0,
    //     },
    //     vec![
    //         Box::new(Sphere::new(
    //             Vec3f::new(0.0, -100.5, -1.0),
    //             100.0,
    //             Box::new(material_ground),
    //         )),
    //         Box::new(Sphere::new(
    //             Vec3f::new(1.0, 0.0, 1.2),
    //             0.5,
    //             Box::new(material_center),
    //         )),
    //         Box::new(Sphere::new(
    //             Vec3f::new(3.0, 0.5, 1.0),
    //             0.5,
    //             Box::new(material_left),
    //         )),
    //         Box::new(Sphere::new(
    //             Vec3f::new(3.0, 0.5, 1.0),
    //             0.4,
    //             Box::new(material_bubble),
    //         )),
    //         Box::new(Sphere::new(
    //             Vec3f::new(1.0, 0.2, -2.0),
    //             0.5,
    //             Box::new(material_right),
    //         )),
    //     ],
    // );

    // let mut raytracer = Raytracer::new(scene);

    // let image = raytracer.render_image(RenderOptions {
    //     image_width: 600,
    //     aspect_ratio: 16.0 / 9.0,
    //     max_ray_depth: 50,
    //     samples_per_pixel: 50,
    // });

    // let path = Path::new(&current_dir().unwrap()).join("out.png");

    // match elktracer_core::raytracer::image::save(
    //     &image,
    //     &path,
    //     image_rs::ImageFormat::Png,
    // ) {
    //     Ok(_) => {
    //         log::info!("Successfully rendered to {:?}", path);
    //     }
    //     Err(err) => {
    //         log::error!("Error rendering image: {}", err);
    //     }
    // }
}
