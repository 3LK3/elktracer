use super::{RayHitDetails, SceneObject};
use crate::math::{ray::Ray, vector3::Vec3f};

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
    fn intersects(
        &self,
        ray: &Ray,
        t_range: (f64, f64),
    ) -> Option<RayHitDetails> {
        let origin_center = self.center_position - ray.origin();
        let a = ray.direction().magnitude_squared();
        let h = ray.direction().dot(origin_center);
        let c = origin_center.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - discriminant_sqrt) / a;
        if root <= t_range.0 || t_range.1 <= root {
            root = (h + discriminant_sqrt) / a;
            if root <= t_range.0 || t_range.1 <= root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(RayHitDetails::new(
            point,
            (point - self.center_position) / self.radius,
            root,
        ))
    }
}
