use crate::linalg::Vector3D;

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

pub fn ray_color_blue_grad(ray: &Ray) -> Vector3D {
    let t = 0.5 * ray.direction.y + 1.0;
    Vector3D::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3D::new(0.5, 0.7, 1.0) * t
}

pub fn ray_color_3d_grad(ray: &Ray) -> Vector3D {
    let tx = ray.direction.x;
    let ty = ray.direction.y;
    Vector3D::new(tx, ty, 0.25).unit()
}
