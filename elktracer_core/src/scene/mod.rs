use crate::math::ray::Ray;

pub mod sphere;

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> bool;
}
