use std::sync::Arc;

use crate::{
    material::Material,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
};

pub struct RayHitDetails {
    point: Vec3f,
    t: f64,
    normal: Vec3f,
    is_front_face: bool,
    // pub material: &'a mut dyn Material,
    pub material: Arc<dyn Material>,
}

impl RayHitDetails {
    pub fn from(
        point: Vec3f,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3f,
        material: Arc<dyn Material>,
    ) -> Self {
        let is_front_face = ray.direction().dot(outward_normal) < 0.0;
        Self {
            point,
            t,
            normal: if is_front_face {
                outward_normal
            } else {
                -outward_normal
            },
            is_front_face,
            material,
        }
    }

    pub fn normal(&self) -> Vec3f {
        self.normal
    }

    pub fn point(&self) -> Vec3f {
        self.point
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn is_front_face(&self) -> bool {
        self.is_front_face
    }
}

pub trait RayHitTest {
    fn does_hit(
        &mut self,
        ray: &Ray,
        ray_t: &Interval,
    ) -> Option<RayHitDetails>;
}
