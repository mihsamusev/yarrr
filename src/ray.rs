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

    pub fn intesect(&self, sphere: &Sphere) -> Option<f32> {
        let sphere_dir = self.origin - sphere.center;
        // components of quadratic eq
        let a = self.direction.norm_squared();
        let half_b = sphere_dir.dot(&self.direction);
        let c = sphere_dir.norm_squared() - sphere.radius * sphere.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            // closest intersection
            let t = (-half_b - discriminant.sqrt()) / a;
            Some(t)
        }
    }
}
