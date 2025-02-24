use std::{env::current_dir, path::Path};

use elktracer_core::raytracer::Raytracer;

fn main() {
    elktracer_core::logging::initialize();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 20;
    let max_ray_depth = 10;

    let path = Path::new(&current_dir().unwrap()).join("out.png");

    let mut raytracer = Raytracer::new(image_width, aspect_ratio, samples_per_pixel, max_ray_depth);
    raytracer.render_image(&path);
}
