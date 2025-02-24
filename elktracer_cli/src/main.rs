use std::{env::current_dir, path::Path};

use elktracer_core::raytracer::Raytracer;

fn main() {
    elktracer_core::logging::initialize();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 640;
    let path = Path::new(&current_dir().unwrap()).join("out.png");

    let raytracer = Raytracer::new(image_width, aspect_ratio);
    raytracer.render_image(&path);
}
