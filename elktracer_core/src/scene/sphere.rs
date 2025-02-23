use crate::math::{ray::Ray, vector3::Vec3f};

use super::SceneObject;

pub struct Sphere {
    center_position: Vec3f,
    radius: f64,
}

impl Sphere {
    pub fn new(center_position: Vec3f, radius: f64) -> Self {
        Self {
            center_position,
            radius,
        }
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let origin_center = self.center_position - ray.origin();
        let a = ray.direction().dot(ray.direction());
        let b = -2.0 * ray.direction().dot(origin_center);
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
        (b * b - 4.0 * a * c) >= 0.0
    }
}
