use std::ops::{Add, Mul};

use image::Rgb;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

// Uses values between 0 and 1
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        let mut color = Self { r, g, b };
        color.clamp();
        color
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        Rgb([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }

    fn clamp(&mut self) {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_be_correct() {
        let color = Color::new(0.0, 0.5, 1.0);

        assert_eq!(color.r, 0.0);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 1.0);
    }

    #[test]
    fn as_rgb_should_be_correct() {
        let color = Color::new(0.0, 0.5, 1.0);
        let rgb = color.as_rgb();

        assert_eq!(rgb.0[0], 0);
        assert_eq!(rgb.0[1], 127);
        assert_eq!(rgb.0[2], 255);
    }

    #[test]
    fn as_rgb_should_clamp_correctly() {
        let color = Color::new(2.0, -0.5, 1.1);
        let rgb = color.as_rgb();

        assert_eq!(rgb.0[0], 255);
        assert_eq!(rgb.0[1], 0);
        assert_eq!(rgb.0[2], 255);
    }
}
