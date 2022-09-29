use crate::prelude::*;

/// container for image view port which is a plane focal length away from
/// the camera origin that the rays are shoot through
///
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(height: f32, aspect_ratio: f32) -> Self {
        let width = aspect_ratio * height;
        Self { width, height }
    }
}

/// common trait for all cameras that can be used in
/// the rendering
///
pub trait Camera {
    fn ray_from_uv(&self, u: f32, v: f32) -> Ray;
}

/// simple axis alligned camera at origin
/// pointing towards -Z direction
///
pub struct SimpleCamera {
    pub origin: Vector3D,
    pub viewport: Viewport,
    pub focal_length: f32,
    lower_left: Vector3D,
}

impl SimpleCamera {
    pub fn new(viewport: Viewport, focal_length: f32, origin: Vector3D) -> Self {
        let lower_left =
            origin - Vector3D::new(viewport.width / 2.0, viewport.height / 2.0, focal_length);
        Self {
            origin,
            viewport,
            focal_length,
            lower_left,
        }
    }
}

impl Camera for SimpleCamera {
    fn ray_from_uv(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left
            + Vector3D::new(u * self.viewport.width, v * self.viewport.height, 0.0)
            - self.origin;
        Ray::new(self.origin, dir)
    }
}

/// field of view camera at origin
///
pub struct FovCamera {
    origin: Vector3D,
    pub vfow: f32,
    pub aspect_ratio: f32,
    vp_lower_left_corner: Vector3D,
    vp_horizontal_span: Vector3D,
    vp_vertical_span: Vector3D,
}

impl FovCamera {
    pub fn new(
        origin: Vector3D,
        lookat: Vector3D,
        vup: Vector3D,
        vfow: f32,
        aspect_ratio: f32,
    ) -> Self {
        // build viewport
        let theta = vfow.to_radians();
        let h = (theta / 2.0).tan();
        let viewport = Viewport::new(2.0 * h, aspect_ratio);

        // build coordinate system
        let w = (origin - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let vp_horizontal_span = viewport.width * u;
        let vp_vertical_span = viewport.height * v;
        let vp_lower_left_corner = origin - vp_horizontal_span / 2.0 - vp_vertical_span / 2.0 - w;

        Self {
            origin,
            vp_lower_left_corner,
            vp_horizontal_span,
            vp_vertical_span,
            vfow,
            aspect_ratio,
        }
    }
}

impl Camera for FovCamera {
    fn ray_from_uv(&self, u: f32, v: f32) -> Ray {
        let dir =
            self.vp_lower_left_corner + u * self.vp_horizontal_span + v * self.vp_vertical_span
                - self.origin;
        Ray::new(self.origin, dir)
    }
}
