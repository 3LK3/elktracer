use rand::Rng;

use crate::math::{ray::Ray, vector3::Vec3f};

pub struct Camera {
    position: Vec3f,
    // fov_vertical: f64,
    image_width: u32,
    image_height: u32,
    // Viewport properties
    viewport_upper_left_pixel: Vec3f,
    viewport_pixel_delta_x: Vec3f,
    viewport_pixel_delta_y: Vec3f,
    // Misc
    random: rand::rngs::ThreadRng,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        Self {
            position: Vec3f::zero(),
            // fov_vertical,
            image_width,
            image_height,
            viewport_upper_left_pixel: Vec3f::zero(),
            viewport_pixel_delta_x: Vec3f::zero(),
            viewport_pixel_delta_y: Vec3f::zero(),
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

    pub fn reset_viewport(
        &mut self,
        position: Vec3f,
        look_at: Vec3f,
        up: Vec3f,
        fov_vertical_degrees: f64,
    ) {
        let look_direction = position - look_at;

        let focal_length = look_direction.magnitude();
        let viewport_height: f64 = 2.0
            * (fov_vertical_degrees.to_radians() / 2.0).tan()
            * focal_length;
        let viewport_width: f64 = viewport_height
            * (self.image_width as f64 / self.image_height as f64);

        let w = look_direction.unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);

        let viewport_edge_x = u * viewport_width;
        let viewport_edge_y = -v * viewport_height;

        let pixel_delta_x = viewport_edge_x / (self.image_width as f64);
        let pixel_delta_y = viewport_edge_y / (self.image_height as f64);

        let viewport_upper_left = position
            - (w * focal_length)
            - viewport_edge_x / 2.0
            - viewport_edge_y / 2.0;

        let viewport_ul =
            viewport_upper_left + (pixel_delta_x + pixel_delta_y) * 0.5;

        self.viewport_upper_left_pixel = viewport_ul;
        self.viewport_pixel_delta_x = pixel_delta_x;
        self.viewport_pixel_delta_y = pixel_delta_y;
        self.position = position;
    }
}
