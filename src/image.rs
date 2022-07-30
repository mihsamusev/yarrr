use crate::prelude::*;
use indicatif::ProgressBar;
use rand::Rng;

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
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![RgbColor::default(); (width * height) as usize];
        Self {
            width,
            height,
            buffer,
        }
    }
    pub fn at(&self, i: u32, j: u32) -> RgbColor {
        let idx = (j * self.width + i) as usize;
        self.buffer[idx]
    }

    pub fn set_at(&mut self, i: u32, j: u32, color: RgbColor) {
        let idx = (j * self.width + i) as usize;
        self.buffer[idx] = color;
    }

    pub fn pixel_to_uv(&self, i: u32, j: u32) -> (f32, f32) {
        let u = i as f32 / (self.width - 1) as f32;
        let v = j as f32 / (self.height - 1) as f32;
        (u, v)
    }

    pub fn pixel_to_uv_noisy(&self, i: u32, j: u32) -> (f32, f32) {
        let mut rng = rand::thread_rng();
        let u = (i as f32 + rng.gen_range(0.0..1.0)) / (self.width - 1) as f32;
        let v = (j as f32 + rng.gen_range(0.0..1.0)) / (self.height - 1) as f32;
        (u, v)
    }
}

pub fn print_ppm(image: &Image) {
    let bar = ProgressBar::new((image.height).into());
    // print header
    println!("P3\n{} {}\n255", image.width, image.height);

    // print row by row
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            print!("{}", to_string(&image.at(i, j)));
        }
        println!();
        bar.inc(1);
    }
    bar.finish();
}

pub fn collect_color<T: Hittable + 'static>(ray: &Ray, world: &T) -> RgbColor {
    if let Some(hitdata) = world.hit(ray, 0.0, f32::MAX) {
        (hitdata.normal + Vector3D::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let k = 0.5 * (ray.direction.y + 1.0);
        RgbColor::new(1.0, 1.0, 1.0) * (1.0 - k) + RgbColor::new(0.5, 0.7, 1.0) * k
    }
}

pub fn color_image<T: Hittable + 'static>(image: &mut Image, camera: Camera, world: &T) {
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            // find normalzed coordsinates and ray through them
            let (u, v) = image.pixel_to_uv(i, j);
            let ray = camera.ray_from_uv(u, v);

            // decide on color depending on the world properties
            let color = collect_color(&ray, world);
            image.set_at(i, j, color);
        }
    }
}

pub fn color_image_noisy<T: Hittable + 'static>(image: &mut Image, camera: Camera, world: &T) {
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let mut color = RgbColor::default();
            for _ in 0..100 {
                // find normalzed coordsinates + random deviation and ray through them
                let (u, v) = image.pixel_to_uv_noisy(i, j);
                let ray = camera.ray_from_uv(u, v);

                // decide on color depending on the world properties
                color += collect_color(&ray, world);
            }
            color = clamp_color(color, 100);
            image.set_at(i, j, color);
        }
    }
}

fn clamp_color(color: RgbColor, samples_per_px: u32) -> RgbColor {
    let scale = 1.0 / samples_per_px as f32;
    RgbColor {
        x: (scale * color.x).clamp(0.0, 0.999),
        y: (scale * color.y).clamp(0.0, 0.999),
        z: (scale * color.z).clamp(0.0, 0.999),
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
    lower_left: Vector3D,
}

impl Camera {
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

    pub fn ray_from_uv(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left
            + Vector3D::new(u * self.viewport.width, v * self.viewport.height, 0.0)
            - self.origin;
        Ray::new(self.origin, dir)
    }
}

#[cfg(test)]
mod tests {
    use crate::image::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_pixel_to_uv() {
        let img = Image::new(800, 600);
        let (u, v) = img.pixel_to_uv(0, 0);
        assert!(approx_eq!(f32, u, 0.0, epsilon = 10e-6));
        assert!(approx_eq!(f32, v, 0.0, epsilon = 10e-6));

        let (u, v) = img.pixel_to_uv(799, 0);
        assert!(approx_eq!(f32, u, 1.0, epsilon = 10e-6));
        assert!(approx_eq!(f32, v, 0.0, epsilon = 10e-6));

        let (u, v) = img.pixel_to_uv(799, 599);
        assert!(approx_eq!(f32, u, 1.0, epsilon = 10e-6));
        assert!(approx_eq!(f32, v, 1.0, epsilon = 10e-6));
    }
}
