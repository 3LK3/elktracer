use std::{env::current_dir, path::Path};

use elktracer_core::{
    color::Color,
    material::{
        lambert::LambertMaterial, metal::MetalMaterial,
        transparent::TransparentMaterial,
    },
    math::vector3::Vec3f,
    raytracer::{CameraRenderOptions, Raytracer},
    scene::sphere::Sphere,
};

fn main() {
    elktracer_core::logging::initialize();

    let mut raytracer = Raytracer::new();

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

    let material_ground = LambertMaterial::new(Color::new(0.8, 0.8, 0.0));
    let material_center = LambertMaterial::new(Color::new(0.1, 0.2, 0.5));
    let material_left = TransparentMaterial::new(1.5);
    let material_bubble = TransparentMaterial::new(1.0 / 1.5);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(1.0, 0.0, 1.2),
        0.5,
        Box::new(material_center),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(3.0, 0.5, 1.0),
        0.5,
        Box::new(material_left),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(3.0, 0.5, 1.0),
        0.4,
        Box::new(material_bubble),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(1.0, 0.2, -2.0),
        0.5,
        Box::new(material_right),
    ));

    let camera_options = CameraRenderOptions {
        aspect_ratio: 16.0 / 9.0,
        // image_width: 1280,
        image_width: 600,
        position: Vec3f::new(12.0, 2.0, 3.0),
        look_at: Vec3f::new(0.0, 0.0, 0.0),
        up: Vec3f::new(0.0, 1.0, 0.0),
        fov_vertical_degrees: 15.0,
        defocus_angle: 0.6,
        focus_distance: 10.0,
    };

    let samples_per_pixel = 50;
    let max_ray_depth = 50;

    let image = raytracer.render_image(
        &camera_options,
        samples_per_pixel,
        max_ray_depth,
    );

    let path = Path::new(&current_dir().unwrap()).join("out.png");
    match image.save_with_format(&path, elktracer_core::image::ImageFormat::Png)
    {
        Ok(_) => {
            log::info!("Successfully rendered to {:?}", path);
        }
        Err(err) => {
            log::error!("Error rendering image: {}", err);
        }
    }
}
