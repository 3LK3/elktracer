use image::RgbImage;

use crate::{
    color::Color,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
    profile_scope,
    ray_hit::RayHitTest,
    scene::{camera::Camera, tree::SceneTree},
};

pub struct Raytracer {
    scene_tree: SceneTree,
    background_gradient_start: Color,
    background_gradient_end: Color,
    camera: Camera,
}

pub struct CameraRenderOptions {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub position: Vec3f,
    pub look_at: Vec3f,
    pub up: Vec3f,
    pub fov_vertical_degrees: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            scene_tree: SceneTree::new(),
            background_gradient_start: Color::new(0.3, 0.6, 0.9),
            background_gradient_end: Color::new(1.0, 1.0, 1.0),
            camera: Camera::new(),
        }
    }

    pub fn add_scene_object<T: RayHitTest + 'static>(&mut self, object: T) {
        self.scene_tree.add(object);
    }

    pub fn render_image(
        &mut self,
        options: &CameraRenderOptions,
        samples_per_pixel: u16,
        max_ray_depth: u16,
    ) -> RgbImage {
        profile_scope!("Raytracer::render_image");

        self.camera.update_viewport(options);

        let pixel_samples_scale = Self::pixel_samples_scale(samples_per_pixel);

        log::info!(
            "Rendering image\n  - size: {}x{}",
            self.camera.image_width(),
            self.camera.image_height()
        );

        let mut rgb_image = image::RgbImage::new(
            self.camera.image_width(),
            self.camera.image_height(),
        );

        for y in 0..self.camera.image_height() {
            // log::debug!("{}/{}", y, self.image_height - 1);
            for x in 0..self.camera.image_width() {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..samples_per_pixel {
                    let ray = &self.camera.get_ray(x, y);
                    color += self.calculate_color(ray, max_ray_depth);
                }

                rgb_image.put_pixel(
                    x,
                    y,
                    (color * pixel_samples_scale).as_rgb(),
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
                        * self.calculate_color(&result.0, depth - 1);
                }
                None => return Color::new(0.0, 0.0, 0.0),
            }
        }

        let a: f64 = (ray.direction().unit().y() + 1.0) * 0.5;
        self.background_gradient_end * (1.0 - a)
            + self.background_gradient_start * a
    }

    fn pixel_samples_scale(samples_per_pixel: u16) -> f64 {
        1.0 / (samples_per_pixel as f64)
    }
}

impl Default for Raytracer {
    fn default() -> Self {
        Self::new()
    }
}
