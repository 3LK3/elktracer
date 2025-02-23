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
    fn intersects(&self, ray: &Ray) -> f64 {
        let origin_center = self.center_position - ray.origin();
        let a = ray.direction().dot(ray.direction());
        let b = -2.0 * ray.direction().dot(origin_center);
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - f64::sqrt(discriminant)) / (2.0 * a)
        }
    }
}
