use crate::prelude::*;
use std::rc::Rc;

/// Container for simplest hittable object
/// sphere with a material
///
pub struct Sphere {
    pub center: Vector3D,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    /// hitting a single sphere can be solved in constant time
    /// solving a quadratic equation
    ///
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
        let mut t = (-half_b - discriminant.sqrt()) / a;
        if (t < t_min) | (t > t_max) {
            t = (-half_b + discriminant.sqrt()) / a;
            if (t < t_min) | (t > t_max) {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center).unit();
        let mut record = HitRecord::new(point, t, normal, &self.material);
        record.set_ray_facing_normal(ray);
        Some(record)
    }
}

/// Container for a collection of various hittable objects
///
pub struct HittableScene {
    bodies: Vec<Rc<dyn Hittable + 'static>>,
}

impl HittableScene {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: Rc<T>) {
        self.bodies.push(object);
    }
}

impl Default for HittableScene {
    fn default() -> Self {
        HittableScene::new()
    }
}

impl Hittable for HittableScene {
    /// Naive implementation will find an intersection in O(N) time
    /// where N is object count
    ///
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut t_closest = t_max;
        for body in self.bodies.iter() {
            if let Some(temp_record) = body.hit(ray, t_min, t_closest) {
                t_closest = temp_record.t;
                record = Some(temp_record);
            }
        }
        record
    }
}

/// Container for a scene containing only spheres
///
pub struct SphereScene {
    bodies: Vec<Sphere>,
}

impl SphereScene {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add(&mut self, object: Sphere) {
        self.bodies.push(object);
    }
}

impl Default for SphereScene {
    fn default() -> Self {
        SphereScene::new()
    }
}

impl Hittable for SphereScene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut t_closest = t_max;
        for body in self.bodies.iter() {
            if let Some(temp_record) = body.hit(ray, t_min, t_closest) {
                t_closest = temp_record.t;
                record = Some(temp_record);
            }
        }
        record
    }
}

#[cfg(test)]
mod tests {
    const MAX_TOL_F32: f32 = 1e-6;
    use super::*;
    use crate::{prelude::Vector3D, ray::*};
    use float_cmp::approx_eq;

    #[test]
    fn test_hit_sphere_miss() {
        let s = Sphere::new(Vector3D::new(0.0, 0.0, -5.0), 1.0, Material::None);
        let ray = Ray::new(Vector3D::new(1.0, 1.0, 0.0), Vector3D::new(0.0, 0.0, -1.0));
        let result = s.hit(&ray, 0.0, 1000.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_hit_sphere_from_outside() {
        let s = Sphere::new(Vector3D::new(0.0, 0.0, -5.0), 1.0, Material::None);
        let ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, -1.0));
        let result = s.hit(&ray, 0.0, 1000.0).unwrap();
        assert!(approx_eq!(f32, result.t, 4.0, epsilon = MAX_TOL_F32));
        assert!(result.is_front_face);

        assert!(approx_eq!(f32, result.point.x, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.point.y, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.point.z, -4.0, epsilon = MAX_TOL_F32));

        assert!(approx_eq!(f32, result.normal.x, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.normal.y, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.normal.z, 1.0, epsilon = MAX_TOL_F32));
    }

    #[test]
    fn test_hit_sphere_from_inside() {
        let s = Sphere::new(Vector3D::new(0.0, 0.0, -5.0), 0.5, Material::None);
        let ray = Ray::new(Vector3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, -1.0));
        let result = s.hit(&ray, 0.0, 1000.0).unwrap();
        assert!(approx_eq!(f32, result.t, 0.5, epsilon = MAX_TOL_F32));
        assert!(!result.is_front_face);

        assert!(approx_eq!(f32, result.point.x, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.point.y, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.point.z, -5.5, epsilon = MAX_TOL_F32));

        assert!(approx_eq!(f32, result.normal.x, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.normal.y, 0.0, epsilon = MAX_TOL_F32));
        assert!(approx_eq!(f32, result.normal.z, 1.0, epsilon = MAX_TOL_F32));
    }
}
