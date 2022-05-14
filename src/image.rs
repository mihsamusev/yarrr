use crate::prelude::*;
use indicatif::ProgressBar;
type RgbColor = Vector3D;

fn to_string(vec: &RgbColor) -> String {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    format!("{} {} {} ", r, g, b)
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    buffer: Vec<RgbColor>,
}

impl Image {
    //pub fn new(width: u32, height: u32) -> Self {}
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![RgbColor::default(); (width * height) as usize];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn with_gradient(width: u32, height: u32) -> Self {
        let mut buffer: Vec<RgbColor> = Vec::with_capacity((width * height) as usize);
        for row in 0..height {
            for col in 0..width {
                let r = (col as f32) / (width as f32 - 1.0);
                let g = (row as f32) / (height as f32 - 1.0);
                let color = RgbColor::new(r, g, 0.25);
                buffer.push(color);
            }
        }
        Self {
            width,
            height,
            buffer,
        }
    }
}

pub fn print_ppm(image: Image) {
    let bar = ProgressBar::new((image.height).into());
    println!("P3\n{} {}\n255", image.width, image.height);
    for row in 0..image.height {
        for col in 0..image.width {
            let idx = (row * image.width + col) as usize;
            print!("{}", to_string(&image.buffer[idx]));
        }
        println!();
        bar.inc(1);
    }
    bar.finish();
}

pub fn ray_color_blue_grad(ray: &Ray) -> Vector3D {
    let t = 0.5 * (ray.direction.y + 1.0);
    Vector3D::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3D::new(0.5, 0.7, 1.0) * t
}

pub fn ray_color_3d_grad(ray: &Ray) -> Vector3D {
    let tx = ray.direction.x;
    let ty = ray.direction.y;
    Vector3D::new(tx, ty, 0.25).unit()
}

pub fn ray_color_with_sphere(ray: &Ray, sphere: &Sphere) -> RgbColor {
    if let Some(t) = ray.intesect(sphere) {
        let normal = (ray.at(t) - sphere.center).unit();
        (normal + Vector3D::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let k = 0.5 * (ray.direction.y + 1.0);
        RgbColor::new(1.0, 1.0, 1.0) * (1.0 - k) + RgbColor::new(0.5, 0.7, 1.0) * k
    }
}

pub fn print_ppm_ray(image: Image, camera: Camera, sphere: Sphere) {
    // header
    println!("P3\n{} {}\n255", image.width, image.height);

    // scanlines
    for row in (0..image.height).rev() {
        for col in 0..image.width {
            let u = col as f32 / (image.width - 1) as f32;
            let v = row as f32 / (image.height - 1) as f32;
            let ray = camera.ray_from_uv(u, v);
            let col = ray_color_with_sphere(&ray, &sphere);
            print!("{}", to_string(&col));
        }
        println!();
    }
}

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

pub struct Camera {
    pub origin: Vector3D,
    pub viewport: Viewport,
    pub focal_length: f32,
}

impl Camera {
    pub fn new(viewport: Viewport, focal_length: f32) -> Self {
        let origin = Vector3D::default();
        Self {
            origin,
            viewport,
            focal_length,
        }
    }

    pub fn ray_from_uv(&self, u: f32, v: f32) -> Ray {
        let lower_left = self.origin
            - Vector3D::new(
                self.viewport.width / 2.0,
                self.viewport.height / 2.0,
                self.focal_length,
            );
        let dir = lower_left
            + Vector3D::new(u * self.viewport.width, v * self.viewport.height, 0.0)
            - self.origin;
        Ray::new(self.origin, dir)
    }
}
