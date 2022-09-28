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

impl<'a> ops::Neg for &'a Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Vector3D> for &'a Vector3D {
    type Output = Vector3D;

    fn add(self, other: &'b Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> ops::Add<Vector3D> for &'a Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vector3D> for &'a Vector3D {
    type Output = Vector3D;
    fn sub(self, other: &'b Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> ops::Sub<Vector3D> for &'a Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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

impl<'a> ops::Mul<f32> for &'a Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl ops::Mul<Vector3D> for f32 {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: Vector3D) -> Self::Output {
        rhs * self
    }
}

impl<'a> ops::Mul<&'a Vector3D> for f32 {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: &'a Vector3D) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vector3D) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> ops::Mul<&'a Vector3D> for Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: &'a Vector3D) -> Self::Output {
        Vector3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> ops::Mul<Vector3D> for &'a Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
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

    pub fn is_near_zero(&self) -> bool {
        self.norm() < 10e-8
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
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }
}

// normal vec is unit
pub fn reflect(vec: &Vector3D, normal: &Vector3D) -> Vector3D {
    vec - 2.0 * normal * vec.dot(normal)
}

pub fn refract(vec: &Vector3D, normal: &Vector3D, refraction_index: f32) -> Vector3D {
    let cos_theta_incoming = (-vec).dot(normal).min(1.0);
    let r_out_perp = refraction_index * (vec + cos_theta_incoming * normal);
    let r_out_para = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_para
}

#[inline]
pub fn shlick_reflectance(cos_theta: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cos_theta).powi(5)
}

// test only utils
#[cfg(test)]
pub const MAX_TOL_F32: f32 = 1e-6;

#[cfg(test)]
pub fn assert_almost_eq(left: f32, right: f32) {
    assert!((left - right).abs() < MAX_TOL_F32)
}

#[cfg(test)]
pub fn assert_vec_eq(left: &Vector3D, right: &Vector3D) {
    for i in 0..3 {
        assert_almost_eq(left[i], right[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_create() {
        let v = Vector3D::default();
        assert_almost_eq(v.x, 0.0);
        assert_almost_eq(v.y, 0.0);
        assert_almost_eq(v.z, 0.0);
    }

    #[test]
    fn test_vector_index() {
        let v = Vector3D::new(0.1, 0.2, 0.3);
        assert_almost_eq(v[0], 0.1);
        assert_almost_eq(v[1], 0.2);
        assert_almost_eq(v[2], 0.3);
    }

    #[test]
    fn test_vector_add_and_eq() {
        let v1 = Vector3D::new(0.5, -0.1, 0.0);
        let v2 = Vector3D::new(1.5, 2.1, 3.0);
        let v3 = v1 + v2;
        assert_vec_eq(&v3, &Vector3D::new(2.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector_negate() {
        let v1 = Vector3D::new(0.5, -0.1, 0.0);
        let v2 = -v1;
        assert_vec_eq(&v2, &Vector3D::new(-0.5, 0.1, 0.0));
    }

    #[test]
    fn test_vector_times_scalar() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let v_scaled = v * 10.0;
        assert_vec_eq(&v_scaled, &Vector3D::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn test_vector_times_vector() {
        let v1 = Vector3D::new(-1.0, 2.0, 3.0);
        let v2 = Vector3D::new(1.0, -2.0, 3.0);
        let v_mul = v1 * v2;
        assert_vec_eq(&v_mul, &Vector3D::new(-1.0, -4.0, 9.0));
    }

    #[test]
    fn test_vector_get_unit() {
        let v = Vector3D::new(4.0, -3.0, 0.0);
        let v_unit = v.unit();
        assert_vec_eq(&v_unit, &Vector3D::new(0.8, -0.6, 0.0));
    }

    #[test]
    fn test_vector_norm() {
        let v1 = Vector3D::new(3.0, -4.0, 5.0);
        assert_almost_eq(v1.norm_squared(), 50.0);
        assert_almost_eq(v1.norm(), 50.0_f32.sqrt());
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(-1.0, -2.0, 3.0);
        assert_almost_eq(v1.dot(&v2), 4.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(3.0, 2.0, 1.0);
        assert_vec_eq(&v1.cross(&v2), &Vector3D::new(-4.0, 8.0, -4.0));
        assert_vec_eq(&v2.cross(&v1), &Vector3D::new(4.0, -8.0, 4.0));
    }

    #[test]
    fn test_reflect() {
        let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
        let vec = Vector3D::new(1.0, -1.0, 0.0);
        let reflected = reflect(&vec, &normal);
        assert_vec_eq(&reflected, &Vector3D::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_refract_same_medium() {
        let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
        let vec = Vector3D::new(1.0, -1.0, 0.0).unit();
        let refracted = refract(&vec, &normal, 1.0);
        assert_vec_eq(&refracted, &Vector3D::new(1.0, -1.0, 0.0).unit());
    }

    #[test]
    fn test_refract_different_medium() {
        // based on this https://www.omnicalculator.com/physics/snells-law
        let refraction_index = 0.8 / 0.6;
        let angle_in = 30.0 * std::f32::consts::PI / 180.0;
        let angle_out = 41.8103 * std::f32::consts::PI / 180.0;

        let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
        let vec = Vector3D::new(angle_in.sin(), -angle_in.cos(), 0.0).unit();
        let refracted = refract(&vec, &normal, refraction_index);
        dbg!(refracted);

        assert_vec_eq(
            &refracted,
            &Vector3D::new(angle_out.sin(), -angle_out.cos(), 0.0).unit(),
        );
    }
}
