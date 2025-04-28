use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
};

use super::Material;

pub struct LambertMaterial {
    albedo: Color,
}

impl LambertMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertMaterial {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_point: Vec3f,
        hit_normal: Vec3f,
        _is_hit_front_face: bool,
    ) -> Option<(Ray, Color)>
    where
        Self: Sized,
    {
        let mut scatter_direction = hit_normal + Vec3f::random_unit();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_normal;
        }

        let scattered = Ray::new(hit_point, scatter_direction);

        Some((scattered, self.albedo))
    }
}
