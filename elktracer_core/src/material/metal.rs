use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
};

use super::Material;

pub struct MetalMaterial {
    albedo: Color,
    fuzziness: f64,
    random: rand::rngs::ThreadRng,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: f64::clamp(fuzziness, 0.0, 1.0),
            random: rand::rng(),
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &mut self,
        ray: &crate::math::ray::Ray,
        hit_point: Vec3f,
        hit_normal: Vec3f,
        _is_hit_front_face: bool,
    ) -> Option<(crate::math::ray::Ray, crate::color::Color)> {
        let reflected = ray.direction().reflect(hit_normal).unit()
            + (Vec3f::random_unit(&mut self.random) * self.fuzziness);

        let scattered = Ray::new(hit_point, reflected); //(Ray::new(hit_point, new_r), self.albedo);

        if scattered.direction().dot(hit_normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
