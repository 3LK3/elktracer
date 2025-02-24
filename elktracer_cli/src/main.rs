use std::{env::current_dir, path::Path};

use elktracer_core::raytracer::Raytracer;

fn main() {
    elktracer_core::logging::initialize();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let path = Path::new(&current_dir().unwrap()).join("out.png");

    let mut raytracer = Raytracer::new(image_width, aspect_ratio, 100, 50);
    raytracer.render_image(&path);
}
