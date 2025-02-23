use std::{env::current_dir, path::Path, u32};

use crate::{
    color::Color,
    math::{ray::Ray, vector3::Vec3f},
    scene::{sphere::Sphere, SceneObject},
};

pub struct Raytracer {
    image_width: u32,
    image_height: u32,
    sphere: Sphere,
    background_gradient_start: Color,
    background_gradient_end: Color,
}

impl Raytracer {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        Self {
            image_width,
            image_height: image_height.clamp(1, u32::MAX),
            sphere: Sphere::new(Vec3f::new(0.0, 0.0, -1.0), 0.5),
            background_gradient_start: Color::new(0.3, 0.6, 0.9),
            background_gradient_end: Color::new(1.0, 1.0, 1.0),
        }
    }

    pub fn render_image(&self) -> () {
        log::info!(
            "Rendering image\n  - size: {}x{}",
            self.image_width,
            self.image_height
        );

        // distance from eye to viewport
        let focal_length = 1.0;
        // the eye of the tiger or viewer
        let camera_center = Vec3f::zero();

        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height
            * (self.image_width as f64 / self.image_height as f64);

        let viewport_edge_x = Vec3f::new(viewport_width, 0.0, 0.0);
        let viewport_edge_y = Vec3f::new(0.0, -viewport_height, 0.0);

        let pixel_delta_x = viewport_edge_x / (self.image_width as f64);
        let pixel_delta_y = viewport_edge_y / (self.image_height as f64);

        let viewport_upper_left = camera_center
            - Vec3f::new(0.0, 0.0, focal_length)
            - viewport_edge_x / 2.0
            - viewport_edge_y / 2.0;
        let upper_left_pixel =
            viewport_upper_left + (pixel_delta_x + pixel_delta_y) * 0.5;

        log::info!("Upper left: {:?}", upper_left_pixel);

        let mut rgb_image =
            image::RgbImage::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            log::trace!("Progress in height: {}/{}", y, self.image_height);
            for x in 0..self.image_width {
                let pixel_center = upper_left_pixel
                    + (pixel_delta_x * x)
                    + (pixel_delta_y * y);
                let ray = Ray::new(camera_center, pixel_center - camera_center);

                let color = self.calculate_color(&ray);

                rgb_image.put_pixel(x, y, color.as_rgb());
            }
        }

        let path = Path::new(&current_dir().unwrap()).join("out.png");
        match rgb_image.save_with_format(&path, image::ImageFormat::Png) {
            Ok(_) => {
                log::info!("Successfully rendered to {:?}", path);
            }
            Err(err) => {
                log::error!("Error rendering image: {}", err);
            }
        }
    }

    fn calculate_color(&self, ray: &Ray) -> Color {
        if self.sphere.intersect(ray) {
            return Color::new(0.0, 1.0, 0.0);
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
