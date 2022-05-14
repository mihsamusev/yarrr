use crate::prelude::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Self {
        Self {
            origin,
            direction: direction.unit(),
        }
    }

    #[inline]
    pub fn at(&self, t: f32) -> Vector3D {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub point: Vector3D,
    pub t: f32,
    pub normal: Vector3D,
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3D, t: f32, normal: Vector3D) -> Self {
        Self {
            point,
            t,
            normal,
            is_front_face: true,
        }
    }

    #[inline]
    pub fn set_ray_facing_normal(&mut self, ray: &Ray) {
        self.is_front_face = ray.direction.dot(&self.normal) < 0.0;
        if !self.is_front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, f_max: f32) -> Option<HitRecord>;
}
