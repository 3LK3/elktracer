use std::fmt;

use crate::{
    camera::Camera,
    color::Color,
    math::{interval::Interval, ray::Ray},
    ray_hit::{RayHitDetails, RayHitTest},
    raytracer_context::RaytracerContext,
};

pub mod image {
    use image::{ImageError, RgbaImage};

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

        pub fn data(&self) -> Vec<u8> {
            self.pixel_data.to_vec()
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

    impl From<&Image> for RgbaImage {
        fn from(val: &Image) -> Self {
            RgbaImage::from_vec(val.width, val.height, val.pixel_data.clone())
                .expect("Failed to create image buffer from vector")
        }
    }

    impl From<Image> for RgbaImage {
        fn from(val: Image) -> Self {
            RgbaImage::from(&val)
        }
    }

    pub fn save_to_file<P: AsRef<std::path::Path>>(
        image: &Image,
        path: P,
        format: image::ImageFormat,
    ) -> Result<(), ImageError> {
        // let buffer: image::RgbaImage = ImageBuffer::from_vec(
        //     image.width,
        //     image.height,
        //     image.pixel_data.clone(),
        // )
        // .expect("Failed to create image buffer from vector");
        RgbaImage::from(image).save_with_format(path, format)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u16,
    pub max_ray_depth: u16,
}

impl RenderOptions {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u16,
        max_ray_depth: u16,
    ) -> Self {
        Self {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_ray_depth,
        }
    }
}

impl fmt::Display for RenderOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RenderOptions {{ image_width: {}, aspect_ratio: {:.2}, samples_per_pixel: {}, max_ray_depth: {} }}",
            self.image_width,
            self.aspect_ratio,
            self.samples_per_pixel,
            self.max_ray_depth
        )
    }
}

pub struct Raytracer {
    background_gradient_start: Color,
    background_gradient_end: Color,
    raytracer_context: RaytracerContext,
    objects: Vec<Box<dyn RayHitTest>>,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            background_gradient_start: Color::new(0.3, 0.6, 0.9),
            background_gradient_end: Color::new(1.0, 1.0, 1.0),
            raytracer_context: RaytracerContext::new(),
            objects: Vec::new(),
        }
    }

    pub fn render_image(
        &mut self,
        camera: &Camera,
        objects: Vec<Box<dyn RayHitTest>>,
        options: &RenderOptions,
    ) -> image::Image {
        self.objects = objects;

        self.raytracer_context.update_viewport(
            options.image_width,
            options.aspect_ratio,
            camera,
        );

        let pixel_samples_scale =
            Self::pixel_samples_scale(options.samples_per_pixel);

        log::info!("Rendering image with {}", options);

        let mut rgb_image = image::Image::new(
            self.raytracer_context.image_width(),
            self.raytracer_context.image_height(),
        );

        for y in 0..self.raytracer_context.image_height() {
            for x in 0..self.raytracer_context.image_width() {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..options.samples_per_pixel {
                    let ray = &self.raytracer_context.get_ray(x, y);
                    color += self.calculate_color(ray, options.max_ray_depth);
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

        if let Some(ray_hit) =
            self.does_hit_object(ray, &Interval::new(0.001, f64::INFINITY))
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

    fn does_hit_object(
        &mut self,
        ray: &Ray,
        ray_t: &crate::math::interval::Interval,
    ) -> Option<crate::ray_hit::RayHitDetails> {
        let mut hit_result: Option<RayHitDetails> = None;
        let mut closest = ray_t.max();

        for object in self.objects.iter_mut() {
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

impl Default for Raytracer {
    fn default() -> Self {
        Self::new()
    }
}
