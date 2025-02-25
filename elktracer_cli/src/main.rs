use std::{env::current_dir, path::Path};

use elktracer_core::{
    color::Color,
    material::{lambert::LambertMaterial, metal::MetalMaterial},
    math::vector3::Vec3f,
    raytracer::Raytracer,
    scene::sphere::Sphere,
};

fn main() {
    elktracer_core::logging::initialize();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 600;
    let samples_per_pixel = 20;
    let max_ray_depth = 20;

    let path = Path::new(&current_dir().unwrap()).join("out.png");

    let mut raytracer = Raytracer::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_ray_depth,
    );

    let material_ground = LambertMaterial::new(Color::new(0.8, 0.8, 0.0));
    let material_center = LambertMaterial::new(Color::new(0.1, 0.2, 0.5));
    let material_left = MetalMaterial::new(Color::new(0.8, 0.8, 0.8));
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(0.0, 0.0, -1.7),
        0.5,
        Box::new(material_center),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(-1.0, 0.0, -1.5),
        0.5,
        Box::new(material_left),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(1.0, 0.0, -1.5),
        0.5,
        Box::new(material_right),
    ));

    raytracer.render_image(&path);
}
