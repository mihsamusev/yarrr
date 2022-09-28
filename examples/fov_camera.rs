use std::rc::Rc;
use yarrr::prelude::*;

fn create_scene() -> HittableScene {
    let m_left = Rc::new(Material::Lambertan(ColorRGB::new(0.0, 0.0, 1.0)));
    let m_right = Rc::new(Material::Lambertan(ColorRGB::new(1.0, 0.0, 0.0)));

    let mut scene = HittableScene::new();
    let radius = std::f32::consts::FRAC_PI_4.cos();

    scene.add(Rc::new(Sphere::new(
        Vector3D::new(-radius, 0.0, -1.0),
        radius,
        m_left,
    )));

    scene.add(Rc::new(Sphere::new(
        Vector3D::new(radius, 0.0, -1.0),
        radius,
        m_right,
    )));

    scene
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 90.0;
    let cam = FovCamera::new(
        Vector3D::zero(),
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
