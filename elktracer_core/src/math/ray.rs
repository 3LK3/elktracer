use super::vector3::Vec3f;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3f {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vec3f {
        self.origin
    }

    pub fn direction(&self) -> Vec3f {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn at_should_be_correct() {
        {
            let origin = Vec3f::new(1.0, 1.0, 0.0);
            let direction = Vec3f::new(0.7, 0.2, 0.1);
            let ray = Ray::new(origin, direction);

            let actual = ray.at(3.0);
            let expected = Vec3f::new(3.1, 1.6, 0.3);

            let epsilon = 1e-6;
            assert_approx_eq!(actual.x(), expected.x(), epsilon);
            assert_approx_eq!(actual.y(), expected.y(), epsilon);
            assert_approx_eq!(actual.z(), expected.z(), epsilon);
        }
        {
            let origin = Vec3f::new(1.0, 2.0, 3.0);
            let direction = Vec3f::new(4.0, 5.0, 6.0);
            let ray = Ray::new(origin, direction);

            let actual = ray.at(2.0);
            let expected = Vec3f::new(9.0, 12.0, 15.0);

            let epsilon = 1e-6;
            assert_approx_eq!(actual.x(), expected.x(), epsilon);
            assert_approx_eq!(actual.y(), expected.y(), epsilon);
            assert_approx_eq!(actual.z(), expected.z(), epsilon);
        }
    }
}
