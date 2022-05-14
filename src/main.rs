use std::io::Error;
use yarrr::prelude::*;

fn main() -> Result<(), Error> {
    let aspect_ratio = 16.0 / 9.0;
    let height = 400;
    let width = ((height as f32) * aspect_ratio) as u32;
    let im = Image::new(width, height);
    let vp = Viewport::new(2.0, aspect_ratio);
    let cam = Camera::new(vp, 1.0);
    print_ppm_ray(im, cam);
    //print_ppm(im);
    Ok(())
}
