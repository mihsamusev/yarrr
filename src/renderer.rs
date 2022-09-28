use crate::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

fn to_string(vec: &ColorRGB) -> String {
    let r = (255.999 * vec.x) as u32;
    let g = (255.999 * vec.y) as u32;
    let b = (255.999 * vec.z) as u32;
    format!("{} {} {} ", r, g, b)
}

pub fn print_ppm(image: &Image) {
    println!("P3\n{} {}\n255", image.width, image.height);

    // print row by row
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            print!("{}", to_string(&image.at(i, j)));
        }
        println!();
    }
}

pub struct RenderSettings {
    pub samples_per_px: u32,
    pub bounce_depth: u32,
}

pub fn collect_color<T>(ray: &Ray, world: &T, depth: u32) -> ColorRGB
where
    T: Hittable + 'static,
{
    if depth == 0 {
        return ColorRGB::new(0.0, 0.0, 0.0);
    }

    if let Some(hitdata) = world.hit(ray, 0.0001, f32::INFINITY) {
        if let Some(bounce) = Material::scatter(ray, &hitdata) {
            bounce.attenuation * collect_color(&bounce.ray, world, depth - 1)
        } else {
            ColorRGB::new(0.0, 0.0, 0.0)
        }
    } else {
        let k = 0.5 * (ray.direction.y + 1.0);
        ColorRGB::new(1.0, 1.0, 1.0) * (1.0 - k) + ColorRGB::new(0.5, 0.7, 1.0) * k
    }
}

pub fn color_image<T>(image: &mut Image, camera: impl Camera, world: &T, settings: RenderSettings)
where
    T: Hittable + 'static,
{
    let bar = ProgressBar::new((image.height).into()).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    for j in 0..image.height {
        for i in 0..image.width {
            let mut color = ColorRGB::default();
            for _ in 0..settings.samples_per_px {
                // find normalzed coordsinates + random deviation and ray through them
                let (u, v) = image.pixel_to_uv_noisy(i, j);
                let ray = camera.ray_from_uv(u, v);

                // decide on color depending on the world properties
                color += collect_color(&ray, world, settings.bounce_depth);
            }
            color = clamp_color(color, settings.samples_per_px);
            image.set_at(i, j, color);
        }

        // row finished
        bar.inc(1);
    }
    bar.finish();
}

fn clamp_color(color: ColorRGB, samples_per_px: u32) -> ColorRGB {
    let scale = 1.0 / samples_per_px as f32;
    ColorRGB {
        x: (scale * color.x).sqrt().clamp(0.0, 0.999),
        y: (scale * color.y).sqrt().clamp(0.0, 0.999),
        z: (scale * color.z).sqrt().clamp(0.0, 0.999),
    }
}
