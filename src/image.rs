use crate::prelude::*;
use indicatif::ProgressBar;
use rand::Rng;

pub type ColorRGB = Vector3D;

fn to_string(vec: &ColorRGB) -> String {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    format!("{} {} {} ", r, g, b)
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    buffer: Vec<ColorRGB>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![ColorRGB::default(); (width * height) as usize];
        Self {
            width,
            height,
            buffer,
        }
    }
    pub fn at(&self, i: u32, j: u32) -> ColorRGB {
        let idx = (j * self.width + i) as usize;
        self.buffer[idx]
    }

    pub fn set_at(&mut self, i: u32, j: u32, color: ColorRGB) {
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

pub fn collect_color<T>(ray: &Ray, world: &T, depth: u32) -> ColorRGB
where
    T: Hittable + 'static,
{
    if depth == 0 {
        return ColorRGB::new(0.0, 0.0, 0.0);
    }

    if let Some(hitdata) = world.hit(ray, 0.0001, f32::INFINITY) {
        //let diffuse_direction = hitdata.normal + Vector3D::random(-1.0, 1.0).unit(); // Lambertan with cos(f)
        // let diffuse_direction = hitdata.normal + Vector3D::unit_sphere_sample(); hacky defuse material from sphere sample cos3(f)
        //let bounced_ray_end = hitdata.point + diffuse_direction;
        //let bounced_ray = Ray::new(hitdata.point, bounced_ray_end - hitdata.point);
        //collect_color_difuse(&bounced_ray, world, depth - 1) * 0.5
        if let Some(bounce) = Material::scatter(&ray, &hitdata) {
            return bounce.attenuation * collect_color(&bounce.ray, world, depth - 1);
        } else {
            return ColorRGB::new(0.0, 0.0, 0.0);
        }
    } else {
        let k = 0.5 * (ray.direction.y + 1.0);
        ColorRGB::new(1.0, 1.0, 1.0) * (1.0 - k) + ColorRGB::new(0.5, 0.7, 1.0) * k
    }
}

pub fn color_image<T>(image: &mut Image, camera: impl Camera, world: &T)
where
    T: Hittable + 'static,
{
    let samples_per_px = 100;
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let mut color = ColorRGB::default();
            for _ in 0..samples_per_px {
                // find normalzed coordsinates + random deviation and ray through them
                let (u, v) = image.pixel_to_uv_noisy(i, j);
                let ray = camera.ray_from_uv(u, v);

                // decide on color depending on the world properties
                color += collect_color(&ray, world, 5);
            }
            color = clamp_color(color, samples_per_px);
            image.set_at(i, j, color);
        }
    }
}

fn clamp_color(color: ColorRGB, samples_per_px: u32) -> ColorRGB {
    let scale = 1.0 / samples_per_px as f32;
    ColorRGB {
        x: (scale * color.x).sqrt().clamp(0.0, 0.999),
        y: (scale * color.y).sqrt().clamp(0.0, 0.999),
        z: (scale * color.z).sqrt().clamp(0.0, 0.999),
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
