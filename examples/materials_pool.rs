use rand::Rng;
use std::rc::Rc;
use yarrr::prelude::*;

fn create_scene() -> HittableScene {
    let m_right = Material::Metal(ColorRGB::new(0.8, 0.8, 0.8), 0.2);
    let m_left = Material::Dielectric(1.5);
    let m_center = Material::Lambertan(ColorRGB::new(0.2, 0.1, 0.9));
    let m_ground = Material::Lambertan(ColorRGB::new(0.2, 0.9, 0.4));

    let mut scene = HittableScene::new();
    scene.add(Sphere::new(Vector3D::new(-1.0, -0.0, -1.0), 0.5, m_left));

    scene.add(Sphere::new(Vector3D::new(1.0, 0.0, -1.0), 0.5, m_right));

    scene.add(Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5, m_center));

    scene.add(Sphere::new(
        Vector3D::new(0.0, -100.5, -1.0),
        100.0,
        m_ground,
    ));
    scene
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let vfov = 20.0;
    let cam = FovCamera::new(
        Vector3D::new(13.0, 2.0, 3.0),
        Vector3D::zero(),
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
    par_color_image(&mut im, cam, scene, settings);
    // print_ppm(&im);

    image::save_buffer(
        "doc/render.jpeg",
        &im.as_bytes(),
        im.width,
        im.height,
        image::ColorType::Rgb8,
    )
    .expect("Ünable to save image");
}
