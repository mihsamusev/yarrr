use crate::linalg::Vector3D;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vector3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn get_normal(&self, ray: &Ray) -> Ray {
        todo!()
    }
}
