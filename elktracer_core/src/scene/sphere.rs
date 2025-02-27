use crate::{
    material::Material,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
    ray_hit::{RayHitDetails, RayHitTest},
};

pub struct Sphere {
    center_position: Vec3f,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(
        center_position: Vec3f,
        radius: f64,
        material: Box<dyn Material>,
    ) -> Self {
        Self {
            center_position,
            radius,
            material,
        }
    }
}

impl RayHitTest for Sphere {
    fn does_hit(
        &mut self,
        ray: &Ray,
        ray_t: &Interval,
    ) -> Option<RayHitDetails> {
        let origin_center = self.center_position - ray.origin();
        let a = ray.direction().magnitude_squared();
        let h = ray.direction().dot(origin_center);

        let discriminant = h * h
            - a * (origin_center.magnitude_squared()
                - self.radius * self.radius);
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        Some(RayHitDetails::from(
            point,
            root,
            ray,
            (point - self.center_position) / self.radius,
            &mut *self.material,
        ))
    }
}
