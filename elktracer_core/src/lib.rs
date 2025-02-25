pub mod color;
pub mod logging;
pub mod material;
pub mod math;
pub mod profiler;
pub mod raytracer;
pub mod ray_hit;
pub mod scene;
pub mod utils;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! profile_scope {
        ($name:expr) => {
            let _scope_profiler = $crate::profiler::ScopeProfiler::new($name);
        };
    }
}
