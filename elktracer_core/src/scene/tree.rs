use crate::{
    math::{interval::Interval, ray::Ray},
    ray_hit::{RayHitDetails, RayHitTest},
};

pub struct SceneTree {
    objects: Vec<Box<dyn RayHitTest>>,
}

impl SceneTree {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add<T: RayHitTest + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl RayHitTest for SceneTree {
    fn does_hit(&self, ray: &Ray, ray_t: &Interval) -> Option<RayHitDetails> {
        let mut hit_result: Option<RayHitDetails> = None;
        let mut closest = ray_t.max();

        for object in &self.objects {
            if let Some(hit) =
                object.does_hit(ray, &Interval::new(ray_t.min(), closest))
            {
                closest = hit.t();
                hit_result = Some(hit);
            }
        }

        hit_result
    }
}
