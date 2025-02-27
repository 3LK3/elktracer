use std::{env::current_dir, path::Path};

use elktracer_core::{
    color::Color,
    material::{
        lambert::LambertMaterial, metal::MetalMaterial,
        transparent::TransparentMaterial,
    },
    math::vector3::Vec3f,
    raytracer::Raytracer,
    scene::sphere::Sphere,
};

fn main() {
    elktracer_core::logging::initialize();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 1280;
    let samples_per_pixel = 100;
    let max_ray_depth = 100;

    let mut raytracer = Raytracer::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_ray_depth,
    );

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

    let image = raytracer.render_image(
        Vec3f::new(12.0, 2.0, 3.0),
        Vec3f::new(0.0, 0.0, 0.0),
        Vec3f::new(0.0, 1.0, 0.0),
        15.0,
        0.6,
        10.0,
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
