use crate::math::vector3::Vec3f;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3f,
    pub look_at: Vec3f,
    pub up: Vec3f,

    pub fov_vertical_degrees: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

impl Camera {
    pub fn new(
        position: Vec3f,
        look_at: Vec3f,
        up: Vec3f,
        fov_vertical_degrees: f64,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        Self {
            position,
            look_at,
            up,
            fov_vertical_degrees,
            defocus_angle,
            focus_distance,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3f::new(10.0, 2.0, 0.0),
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            15.0,
            0.5,
            10.0,
        )
    }
}
