use crate::prelude::*;

/// Ray primitive with origin and direction
///
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

    /// Ray primitive with origin and direction
    ///
    #[inline]
    pub fn at(&self, t: f32) -> Vector3D {
        self.origin + self.direction * t
    }
}

/// Structure that describes ray to body intersection
///
pub struct HitRecord<'a> {
    pub point: Vector3D,
    pub t: f32,
    pub normal: Vector3D,
    pub is_front_face: bool,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vector3D, t: f32, normal: Vector3D, material: &'a Material) -> Self {
        Self {
            point,
            t,
            normal,
            material,
            is_front_face: true,
        }
    }

    #[inline]
    /// Figure out wether we hit the front facing side of a body
    /// (normal towards ray) and flip the normal in case of back side
    ///
    pub fn set_ray_facing_normal(&mut self, ray: &Ray) {
        self.is_front_face = ray.direction.dot(&self.normal) < 0.0;
        if !self.is_front_face {
            self.normal = -self.normal;
        }
    }
}

/// trait for all bodies that can be potentially hit by ray and produce
/// the hit record
///
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ray_unit_direction() {
        let orig = Vector3D::new(0.0, 1.0, 2.0);
        let dir = Vector3D::new(1.0, 1.0, 1.0);
        let r = Ray::new(orig, dir);

        let expected_dir = Vector3D::new(
            1.0 / f32::sqrt(3.0),
            1.0 / f32::sqrt(3.0),
            1.0 / f32::sqrt(3.0),
        );

        assert_vec_eq(&r.direction, &expected_dir);
    }

    #[test]
    fn test_ray_at_value() {
        let orig = Vector3D::new(0.0, 1.0, 2.0);
        let dir = Vector3D::new(3.0, 4.0, 0.0);
        let r = Ray::new(orig, dir);

        let dest = r.at(0.0);
        assert_vec_eq(&dest, &orig);

        let dest = r.at(2.0);
        assert_vec_eq(&dest, &Vector3D::new(1.2, 2.6, 2.0));

        let dest = r.at(-3.5);
        assert_vec_eq(&dest, &Vector3D::new(-2.1, -1.8, 2.0));
    }
}
