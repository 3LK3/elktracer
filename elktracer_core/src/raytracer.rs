use std::{path::Path, u32};

use crate::{
    color::Color,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
    profile_scope,
    ray_hit::RayHitTest,
    scene::{camera::Camera, sphere::Sphere, tree::SceneTree},
};

pub struct Raytracer {
    image_width: u32,
    image_height: u32,
    scene_tree: SceneTree,
    background_gradient_start: Color,
    background_gradient_end: Color,
    samples_per_pixel: u16,
    pixel_samples_scale: f64,
    camera: Camera,
}

impl Raytracer {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let mut scene_tree = SceneTree::new();
        scene_tree.add(Sphere::new(Vec3f::new(0.0, 0.0, -1.0), 0.5));
        scene_tree.add(Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0));

        let camera = Camera::new(Vec3f::zero(), 1.0, image_width, image_height);
        let samples_per_pixel = 10;

        Self {
            image_width,
            image_height: image_height.clamp(1, u32::MAX),
            scene_tree,
            background_gradient_start: Color::new(0.3, 0.6, 0.9),
            background_gradient_end: Color::new(1.0, 1.0, 1.0),
            samples_per_pixel: 10,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            camera,
        }
    }

    pub fn render_image(&self, path: &Path) -> () {
        profile_scope!("Raytracer::render_image");

        log::info!(
            "Rendering image\n  - size: {}x{}",
            self.image_width,
            self.image_height
        );

        let mut rgb_image =
            image::RgbImage::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    color += self.calculate_color(&self.camera.get_ray(x, y));
                }

                rgb_image.put_pixel(
                    x,
                    y,
                    (color * self.pixel_samples_scale).as_rgb(),
                );
            }
        }

        match rgb_image.save_with_format(path, image::ImageFormat::Png) {
            Ok(_) => {
                log::info!("Successfully rendered to {:?}", path);
            }
            Err(err) => {
                log::error!("Error rendering image: {}", err);
            }
        }
    }

    fn calculate_color(&self, ray: &Ray) -> Color {
        if let Some(ray_hit) = self
            .scene_tree
            .does_hit(ray, &Interval::new(0.0, f64::INFINITY))
        {
            return Color::new(
                ray_hit.normal().x() + 1.0,
                ray_hit.normal().y() + 1.0,
                ray_hit.normal().z() + 1.0,
            ) * 0.7;
        }

        let a: f64 = (ray.direction().unit().y() + 1.0) * 0.5;
        self.background_gradient_end * (1.0 - a)
            + self.background_gradient_start * a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_calculate_correct_image_height() {
        let aspect_ratio = 16.0 / 9.0;
        let raytracer = Raytracer::new(400, aspect_ratio);

        assert_eq!(raytracer.image_width, 400);
        assert_eq!(raytracer.image_height, 225);
    }
}
