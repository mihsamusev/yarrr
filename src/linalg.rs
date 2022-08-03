use derive_more::{Add, AddAssign, Div, Neg, Sub, SubAssign};
use rand::Rng;
use std::ops;

#[derive(
    Debug, Copy, Clone, PartialEq, PartialOrd, Default, Add, AddAssign, Sub, SubAssign, Neg, Div,
)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Index<usize> for Vector3D {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl ops::Mul<f32> for Vector3D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl ops::MulAssign<f32> for Vector3D {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // random vector in a unit cube between
    // min and max coordinates for each axis
    pub fn random(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn unit_sphere_sample() -> Self {
        loop {
            let v = Vector3D::random(-1.0, 1.0);
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }
    #[inline]
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn norm_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn unit(self) -> Self {
        let scale = 1.0 / self.norm();
        self * scale
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, &other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
