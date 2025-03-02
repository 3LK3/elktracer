pub mod color;
pub mod error;
pub mod material;
pub mod math;
pub mod profiler;
pub mod random;
pub mod ray_hit;
pub mod raytracer;
pub mod scene;
pub mod utils;

pub use image;

pub use self::error::{Error, Result};

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
