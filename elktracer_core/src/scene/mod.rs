pub mod sphere;

use crate::math::{ray::Ray, vector3::Vec3f};

pub struct RayHitDetails {
    point: Vec3f,
    pub normal: Vec3f,
    t: f64,
}

impl RayHitDetails {
    pub fn new(point: Vec3f, normal: Vec3f, t: f64) -> Self {
        Self { point, normal, t }
    }
}

pub trait SceneObject {
    fn intersects(
        &self,
        ray: &Ray,
        t_range: (f64, f64),
    ) -> Option<RayHitDetails>;
}
