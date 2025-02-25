use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
};

use super::Material;

pub struct LambertMaterial {
    albedo: Color,
    random: rand::rngs::ThreadRng,
}

impl LambertMaterial {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            random: rand::rng(),
        }
    }
}

impl Material for LambertMaterial {
    fn scatter(
        &mut self,
        _ray: &Ray,
        hit_point: Vec3f,
        hit_normal: Vec3f, // ray_hit: &RayHitDetails,
    ) -> Option<(Ray, Color)>
    where
        Self: Sized,
    {
        let mut scatter_direction =
            hit_normal + Vec3f::random_unit(&mut self.random);

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_normal;
        }

        let scattered = Ray::new(hit_point, scatter_direction);

        Some((scattered, self.albedo))
    }
}
