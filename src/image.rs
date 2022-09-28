use crate::prelude::*;
use rand::distributions::{Distribution, Uniform};

pub type ColorRGB = Vector3D;

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
        let range = Uniform::from(0.0..1.0);
        let u = (i as f32 + range.sample(&mut rng)) / (self.width - 1) as f32;
        let v = (j as f32 + range.sample(&mut rng)) / (self.height - 1) as f32;
        (u, v)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.buffer
            .iter()
            .rev()
            .flat_map(|color| {
                [
                    (255.999 * color.x) as u8,
                    (255.999 * color.y) as u8,
                    (255.999 * color.z) as u8,
                ]
                .into_iter()
            })
            .collect::<Vec<u8>>()
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
