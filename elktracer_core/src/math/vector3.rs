use std::{
    f64,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::random::{self};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3f {
    // TODO make it an array? does that impact performance?
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    fn random_range_m1_1() -> Self {
        Self::new(
            random::random_f64_m1_1(),
            random::random_f64_m1_1(),
            random::random_f64_m1_1(),
        )
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Self::random_range_m1_1();
            let lensq = p.magnitude_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3f) -> Self {
        let on_unit_sphere = Self::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3f::new(
                random::random_f64_m1_1(),
                random::random_f64_m1_1(),
                0.0,
            );
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn refract(&self, n: Vec3f, etai_over_etat: f64) -> Vec3f {
        let cos_theta = f64::min(-self.dot(n), 1.0);
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel =
            n * -f64::sqrt(f64::abs(1.0 - r_out_perp.magnitude_squared()));

        r_out_perp + r_out_parallel
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.magnitude_squared())
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, other: Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3f) -> Vec3f {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn is_near_zero(&self) -> bool {
        f64::abs(self.x) < 1e-8
            && f64::abs(self.y) < 1e-8
            && f64::abs(self.z) < 1e-8
    }

    pub fn reflect(&self, normal: Vec3f) -> Vec3f {
        *self - normal * self.dot(normal) * 2.0
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3f {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<u32> for Vec3f {
    type Output = Self;

    fn mul(self, scalar: u32) -> Self {
        self * (scalar as f64)
    }
}

impl Div<f64> for Vec3f {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_be_correct() {
        let vec = Vec3f::new(32.5, 44.3, 61.0);

        assert_eq!(vec.x, 32.5);
        assert_eq!(vec.y, 44.3);
        assert_eq!(vec.z, 61.0);
    }

    #[test]
    fn add_should_be_correct() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let b = Vec3f::new(1.0, 0.0, -1.0);

        let c = a + b;

        assert_eq!(c.x, 3.0);
        assert_eq!(c.y, 3.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn subtract_should_be_correct() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let b = Vec3f::new(1.0, 0.0, -1.0);

        let c = a - b;

        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 3.0);
        assert_eq!(c.z, 5.0);
    }

    #[test]
    fn multiply_scalar_f64_should_be_correct() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let c = a * 5.0;

        assert_eq!(c.x, 10.0);
        assert_eq!(c.y, 15.0);
        assert_eq!(c.z, 20.0);
    }

    #[test]
    fn multiply_scalar_u32_should_be_correct() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let c = a * 5;

        assert_eq!(c.x, 10.0);
        assert_eq!(c.y, 15.0);
        assert_eq!(c.z, 20.0);
    }

    #[test]
    fn divide_scalar_should_be_correct() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let c = a / 2.0;

        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.5);
        assert_eq!(c.z, 2.0);
    }

    #[test]
    fn magnitude_should_be_correct() {
        {
            let a = Vec3f::new(1.0, -2.0, 2.0);
            assert_eq!(a.magnitude(), 3.0);
        }

        {
            let a = Vec3f::new(6.0, -3.0, 2.0);
            assert_eq!(a.magnitude(), 7.0);
        }
    }

    #[test]
    fn unit_should_be_correct() {
        {
            let actual = Vec3f::new(1.0, -2.0, 2.0).unit();
            let expected = Vec3f::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn dot_should_be_correct() {
        {
            let a = Vec3f::new(1.0, 3.0, -5.0);
            let b = Vec3f::new(4.0, -2.0, -1.0);

            assert_eq!(a.dot(b), 3.0);
        }
    }
}
