use std::{io::Error, rc::Rc};
use yarrr::prelude::*;

fn main() -> Result<(), Error> {
    let aspect_ratio = 16.0 / 9.0;
    //let height = 400;
    //let width = ((height as f32) * aspect_ratio) as u32;
    let width = 800;
    let height = (width as f32 / aspect_ratio) as u32;
    let mut im = Image::new(width, height);

    let vp = Viewport::new(2.0, aspect_ratio);
    let focal_length = 1.0;
    let cam_origin = Vector3D::default();
    let cam = Camera::new(vp, focal_length, cam_origin);

    let m_right = Rc::new(Material::Metal(ColorRGB::new(0.8, 0.8, 0.8), 0.2));
    let m_left = Rc::new(Material::Dielectric(1.5));
    let m_center = Rc::new(Material::Lambertan(ColorRGB::new(0.2, 0.1, 0.9)));
    let m_ground = Rc::new(Material::Lambertan(ColorRGB::new(0.2, 0.9, 0.4)));

    let mut scene = HittableScene::new();
    scene.add(Rc::new(Sphere::new(
        Vector3D::new(-1.0, -0.0, -1.0),
        0.5,
        m_left,
    )));

    scene.add(Rc::new(Sphere::new(
        Vector3D::new(1.0, 0.0, -1.0),
        0.5,
        m_right,
    )));

    scene.add(Rc::new(Sphere::new(
        Vector3D::new(0.0, 0.0, -1.0),
        0.5,
        m_center,
    )));

    scene.add(Rc::new(Sphere::new(
        Vector3D::new(0.0, -100.5, -1.0),
        100.0,
        m_ground,
    )));

    // render
    color_image(&mut im, cam, &scene);
    print_ppm(&im);
    Ok(())
}
