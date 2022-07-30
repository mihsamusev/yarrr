use std::{io::Error, rc::Rc};
use yarrr::prelude::*;

fn main() -> Result<(), Error> {
    let aspect_ratio = 16.0 / 9.0;
    let height = 400;
    let width = ((height as f32) * aspect_ratio) as u32;
    let mut im = Image::new(width, height);

    let vp = Viewport::new(2.0, aspect_ratio);
    let focal_length = 1.0;
    let cam_origin = Vector3D::default();
    let cam = Camera::new(vp, focal_length, cam_origin);

    let mut scene = HittableScene::new();
    scene.add(Rc::new(Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Rc::new(Sphere::new(Vector3D::new(0.0, -100.5, -1.0), 100.)));

    // vec on a heap
    // let world = vec![
    //     Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5),
    //     Sphere::new(Vector3D::new(0.0, -100.5, -1.0), 100.0),
    // ];
    color_image_noisy(&mut im, cam, &scene);
    print_ppm(&im);
    Ok(())
}
