pub mod lambert;
pub mod metal;
pub mod transparent;

use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
};

pub trait Material {
    fn scatter(
        &mut self,
        ray: &Ray,
        hit_point: Vec3f,
        hit_normal: Vec3f,
        is_hit_front_face: bool,
    ) -> Option<(Ray, Color)>;
}
