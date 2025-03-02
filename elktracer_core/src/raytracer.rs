use crate::{
    color::Color,
    math::{interval::Interval, ray::Ray, vector3::Vec3f},
    profile_scope,
    ray_hit::RayHitTest,
    scene::{camera::Camera, tree::SceneTree},
};

pub mod image {
    use image::{ImageBuffer, ImageError};

    pub struct Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    impl Rgba {
        pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
    }

    pub struct Image {
        width: u32,
        height: u32,
        pixel_data: Vec<u8>,
        channels: u32,
    }

    impl Image {
        pub fn new(width: u32, height: u32) -> Self {
            let channels = 4;
            let pixel_data_length =
                Self::assert_usize(width * height * channels);
            log::trace!(
                "Image :: new :: {}x{} :: {} channels :: pixel_data [{}]",
                width,
                height,
                channels,
                pixel_data_length
            );
            Self {
                width,
                height,
                pixel_data: vec![0; pixel_data_length],
                channels,
            }
        }

        pub fn set_pixel(&mut self, x: u32, y: u32, rgba: Rgba) {
            if x >= self.width || y >= self.height {
                log::warn!(
                    "Pixel coordinated out of bounds. Given x={} and y={}, image width={} and height={}. Pixel data not updated",
                    x,
                    y,
                    self.width,
                    self.height
                );
                return;
            }

            let index = ((y * self.width + x) * self.channels) as usize;
            self.pixel_data[index] = rgba.r;
            self.pixel_data[index + 1] = rgba.g;
            self.pixel_data[index + 2] = rgba.b;
            self.pixel_data[index + 3] = rgba.a;
        }

        pub fn data(&self) -> &[u8] {
            &self.pixel_data
        }

        pub fn save<P: AsRef<std::path::Path>>(
            &self,
            path: P,
            format: image::ImageFormat,
        ) -> Result<(), ImageError> {
            let buffer: image::RgbaImage = ImageBuffer::from_vec(
                self.width,
                self.height,
                self.pixel_data.clone(),
            )
            .expect("Failed to create image buffer from vector");

            buffer.save_with_format(path, format)
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

        fn assert_usize(value: u32) -> usize {
            match usize::try_from(value) {
                Ok(result) => result,
                Err(_) => panic!("Unable to convert u32 '{}' to usize", value),
            }
        }
    }
}

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
    ) -> image::Image {
        profile_scope!("Raytracer::render_image");

        self.camera.update_viewport(options);

        let pixel_samples_scale = Self::pixel_samples_scale(samples_per_pixel);

        log::info!(
            "Rendering image\n  - size: {}x{}",
            self.camera.image_width(),
            self.camera.image_height()
        );

        let mut rgb_image = image::Image::new(
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

                let color1 = color * pixel_samples_scale;
                rgb_image.set_pixel(x, y, color1.as_rgba());
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
