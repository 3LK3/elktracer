mod camera;
mod color;
mod error;
mod material;
mod math;
mod object;
#[cfg(not(target_arch = "wasm32"))]
mod profiler;
mod random;
mod ray_hit;
mod raytracer;
mod raytracer_context;
mod utils;

pub use camera::Camera;
pub use color::Color;
pub use math::vector3::Vec3f;
pub use ray_hit::RayHitTest;
pub use raytracer::{Raytracer, RenderOptions, image::*};

pub use material::Material;
pub use material::lambert::LambertMaterial;
pub use material::metal::MetalMaterial;
pub use material::transparent::TransparentMaterial;
pub use object::sphere::Sphere;

pub use image as image_rs;

pub mod logging {
    pub fn initialize() {
        let env = env_logger::Env::default().filter_or("ELK_LOG", "trace");
        env_logger::builder()
            .parse_env(env)
            .write_style(env_logger::WriteStyle::Always)
            .init();

        log::trace!("Logging initialized");
    }
}
