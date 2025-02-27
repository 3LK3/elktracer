use image::RgbImage;

use crate::{
    color::Color,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
    profile_scope,
    ray_hit::RayHitTest,
    scene::{camera::Camera, tree::SceneTree},
};

pub struct Raytracer {
    image_width: u32,
    image_height: u32,
    scene_tree: SceneTree,
    background_gradient_start: Color,
    background_gradient_end: Color,
    samples_per_pixel: u16,
    pixel_samples_scale: f64,
    max_ray_depth: u16,
    camera: Camera,
}

impl Raytracer {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u16,
        max_ray_depth: u16,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        Self {
            image_width,
            image_height: image_height.clamp(1, u32::MAX),
            scene_tree: SceneTree::new(),
            background_gradient_start: Color::new(0.3, 0.6, 0.9),
            background_gradient_end: Color::new(1.0, 1.0, 1.0),
            samples_per_pixel,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            max_ray_depth,
            camera: Camera::new(image_width, image_height),
        }
    }

    pub fn add_scene_object<T: RayHitTest + 'static>(&mut self, object: T) {
        self.scene_tree.add(object);
    }

    pub fn render_image(
        &mut self,
        camera_position: Vec3f,
        camera_look_at: Vec3f,
        camera_up: Vec3f,
        camera_fov_vertical_degrees: f64,
        camera_defocus_angle: f64,
        camera_focus_distance: f64,
    ) -> RgbImage {
        profile_scope!("Raytracer::render_image");

        self.camera.reset_viewport(
            camera_position,
            camera_look_at,
            camera_up,
            camera_fov_vertical_degrees,
            camera_focus_distance,
            camera_defocus_angle,
        );

        log::info!(
            "Rendering image\n  - size: {}x{}",
            self.image_width,
            self.image_height
        );

        let mut rgb_image =
            image::RgbImage::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            // log::debug!("{}/{}", y, self.image_height - 1);
            for x in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let ray = &self.camera.get_ray(x, y);
                    color += self.calculate_color(ray, self.max_ray_depth);
                }

                rgb_image.put_pixel(
                    x,
                    y,
                    (color * self.pixel_samples_scale).as_rgb(),
                );
            }
        }

        rgb_image
    }

    fn calculate_color(&mut self, ray: &Ray, depth: u16) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(ray_hit) = self
            .scene_tree
            .does_hit(ray, &Interval::new(0.001, f64::INFINITY))
        {
            match ray_hit.material.scatter(
                ray,
                ray_hit.point(),
                ray_hit.normal(),
                ray_hit.is_front_face(),
            ) {
                Some(result) => {
                    return result.1
                        * self.calculate_color(&result.0, depth - 1)
                }
                None => return Color::new(0.0, 0.0, 0.0),
            }
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
        let raytracer = Raytracer::new(400, aspect_ratio, 100, 50);
        assert_eq!(raytracer.image_width, 400);
        assert_eq!(raytracer.image_height, 225);
    }
}
