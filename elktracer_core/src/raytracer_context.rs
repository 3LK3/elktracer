use crate::{
    camera::Camera,
    math::{ray::Ray, vector3::Vec3f},
    random,
};

pub struct RaytracerContext {
    camera_position: Vec3f,
    image_width: u32,
    image_height: u32,
    // Defocus / Depth of field
    defocus_angle: f64,
    defocus_disk_x: Vec3f,
    defocus_disk_y: Vec3f,
    // Viewport properties
    viewport_upper_left_pixel: Vec3f,
    viewport_pixel_delta_x: Vec3f,
    viewport_pixel_delta_y: Vec3f,
}

impl RaytracerContext {
    pub fn new() -> Self {
        Self {
            camera_position: Vec3f::zero(),
            image_width: 0,
            image_height: 0,
            defocus_angle: 0.0,
            defocus_disk_x: Vec3f::zero(),
            defocus_disk_y: Vec3f::zero(),
            viewport_upper_left_pixel: Vec3f::zero(),
            viewport_pixel_delta_x: Vec3f::zero(),
            viewport_pixel_delta_y: Vec3f::zero(),
        }
    }

    pub fn get_ray(&mut self, x: u32, y: u32) -> Ray {
        let offset = (
            // -0.5..0.5
            random::random_f64_0_1() - 0.5,
            random::random_f64_0_1() - 0.5,
        );

        let pixel_sample = self.viewport_upper_left_pixel
            + (self.viewport_pixel_delta_x * (x as f64 + offset.0))
            + (self.viewport_pixel_delta_y * (y as f64 + offset.1));

        let origin = if self.defocus_angle <= 0.0 {
            self.camera_position
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(origin, pixel_sample - origin)
    }

    fn defocus_disk_sample(&mut self) -> Vec3f {
        let p = Vec3f::random_in_unit_disk();
        self.camera_position
            + (self.defocus_disk_x * p.x())
            + (self.defocus_disk_y * p.y())
    }

    pub fn update_viewport(
        &mut self,
        image_width: u32,
        aspect_ratio: f64,
        camera: &Camera,
    ) {
        self.image_width = image_width;
        self.image_height = (image_width as f64 / aspect_ratio) as u32;

        let look_direction = camera.position - camera.look_at;

        let viewport_height: f64 = 2.0
            * (camera.fov_vertical_degrees.to_radians() / 2.0).tan()
            * camera.focus_distance;
        let viewport_width: f64 = viewport_height
            * (self.image_width as f64 / self.image_height as f64);

        let w = look_direction.unit();
        let u = camera.up.cross(w).unit();
        let v = w.cross(u);

        let viewport_edge_x = u * viewport_width;
        let viewport_edge_y = -v * viewport_height;

        let pixel_delta_x = viewport_edge_x / (self.image_width as f64);
        let pixel_delta_y = viewport_edge_y / (self.image_height as f64);

        let viewport_upper_left = camera.position
            - (w * camera.focus_distance)
            - viewport_edge_x / 2.0
            - viewport_edge_y / 2.0;

        self.viewport_upper_left_pixel =
            viewport_upper_left + (pixel_delta_x + pixel_delta_y) * 0.5;
        self.viewport_pixel_delta_x = pixel_delta_x;
        self.viewport_pixel_delta_y = pixel_delta_y;

        let defocus_radius = camera.focus_distance
            * (camera.defocus_angle / 2.0).to_radians().tan();

        self.defocus_disk_x = u * defocus_radius;
        self.defocus_disk_x = v * defocus_radius;
        self.defocus_angle = camera.defocus_angle;

        self.camera_position = camera.position;
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }
}

impl Default for RaytracerContext {
    fn default() -> Self {
        Self::new()
    }
}
