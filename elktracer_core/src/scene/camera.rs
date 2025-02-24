use rand::Rng;

use crate::math::{ray::Ray, vector3::Vec3f};

pub struct Camera {
    position: Vec3f,
    viewport_upper_left_pixel: Vec3f,
    viewport_pixel_delta_x: Vec3f,
    viewport_pixel_delta_y: Vec3f,
    random: rand::rngs::ThreadRng,
}

impl Camera {
    pub fn new(
        position: Vec3f,
        focal_length: f64,
        image_width: u32,
        image_height: u32,
    ) -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (image_width as f64 / image_height as f64);

        let viewport_edge_x = Vec3f::new(viewport_width, 0.0, 0.0);
        let viewport_edge_y = Vec3f::new(0.0, -viewport_height, 0.0);

        let pixel_delta_x = viewport_edge_x / (image_width as f64);
        let pixel_delta_y = viewport_edge_y / (image_height as f64);

        let viewport_upper_left = position
            - Vec3f::new(0.0, 0.0, focal_length)
            - viewport_edge_x / 2.0
            - viewport_edge_y / 2.0;

        Self {
            position,
            viewport_upper_left_pixel: viewport_upper_left
                + (pixel_delta_x + pixel_delta_y) * 0.5,
            viewport_pixel_delta_x: pixel_delta_x,
            viewport_pixel_delta_y: pixel_delta_y,
            random: rand::rng(),
        }
    }

    pub fn get_ray(&mut self, x: u32, y: u32) -> Ray {
        let offset = (
            self.random.random_range(-0.5..=0.5),
            self.random.random_range(-0.5..=0.5),
        );

        let pixel_sample = self.viewport_upper_left_pixel
            + (self.viewport_pixel_delta_x * (x as f64 + offset.0))
            + (self.viewport_pixel_delta_y * (y as f64 + offset.1));

        Ray::new(self.position, pixel_sample - self.position)
    }
}
