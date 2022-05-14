use crate::prelude::*;

pub struct Sphere {
    pub center: Vector3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let sphere_dir = ray.origin - self.center;
        // components of quadratic eq
        let a = ray.direction.norm_squared();
        let half_b = sphere_dir.dot(&ray.direction);
        let c = sphere_dir.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // closest intersection
        let t = (-half_b - discriminant.sqrt()) / a;
        if (t < t_min) & (t > t_max) {
            return None;
        }
        let point = ray.at(t);
        let normal = (point - self.center).unit();
        let mut record = HitRecord::new(point, t, normal);
        record.set_ray_facing_normal(&ray);
        return Some(record);
    }
}

pub fn hit_all<T: Hittable>(bodies: &[T], ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut record = None;
    let mut t_closest = t_max;
    for b in bodies {
        if let Some(temp_record) = b.hit(ray, t_min, t_closest) {
            t_closest = temp_record.t;
            record = Some(temp_record);
        }
    }
    record
}
