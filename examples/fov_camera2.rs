use std::rc::Rc;
use yarrr::prelude::*;

fn create_scene() -> HittableScene {
    let m_left = Rc::new(Material::Metal(ColorRGB::new(0.8, 0.6, 0.2), 0.3));
    let m_right = Rc::new(Material::Metal(ColorRGB::new(0.0, 0.6, 0.5), 0.0));
    let m_center = Rc::new(Material::Lambertan(ColorRGB::new(1.0, 1.0, 0.0)));
    let m_ground = Rc::new(Material::Lambertan(ColorRGB::new(0.2, 0.9, 0.4)));

    // create scene
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

    scene
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 90.0;
    let cam = FovCamera::new(
        Vector3D::new(-2.0, 2.0, 1.0),
        -Vector3D::unit_z(),
        Vector3D::unit_y(),
        vfov,
        aspect_ratio,
    );

    let width = 800;
    let height = (width as f32 / aspect_ratio) as u32;
    let mut im = Image::new(width, height);

    // scene
    let scene = create_scene();

    // render
    let settings = RenderSettings {
        samples_per_px: 100,
        bounce_depth: 5,
    };
    color_image(&mut im, cam, &scene, settings);
    print_ppm(&im);
}
