use crate::{color::Color, math::ray::Ray, random};

use super::Material;

pub struct TransparentMaterial {
    refraction_index: f64,
}

impl TransparentMaterial {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn get_reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_2 = r0 * r0;

        r0_2 + (1.0 - r0_2) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for TransparentMaterial {
    fn scatter(
        &self,
        ray: &crate::math::ray::Ray,
        hit_point: crate::math::vector3::Vec3f,
        hit_normal: crate::math::vector3::Vec3f,
        is_hit_front_face: bool,
    ) -> Option<(Ray, Color)> {
        let unit_direction = ray.direction().unit();

        let ri = if is_hit_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let cos_theta = f64::min(-unit_direction.dot(hit_normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract
            || self.get_reflectance(cos_theta, ri) > random::random_f64_0_1()
        {
            unit_direction.reflect(hit_normal)
        } else {
            unit_direction.refract(hit_normal, ri)
        };

        let scattered = Ray::new(hit_point, direction);

        Some((scattered, Color::new(1.0, 1.0, 1.0)))
    }
}
