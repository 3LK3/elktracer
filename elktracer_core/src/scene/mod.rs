use crate::math::ray::Ray;

pub mod sphere;

pub trait SceneObject {
    fn intersects(&self, ray: &Ray) -> f64;
}
