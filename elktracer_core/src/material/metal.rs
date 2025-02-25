use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
};

use super::Material;

pub struct MetalMaterial {
    albedo: Color,
}

impl MetalMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &mut self,
        ray: &crate::math::ray::Ray,
        hit_point: Vec3f,
        hit_normal: Vec3f,
    ) -> Option<(crate::math::ray::Ray, crate::color::Color)> {
        let reflected = ray.direction().reflect(hit_normal);
        Some((Ray::new(hit_point, reflected), self.albedo))
    }
}
