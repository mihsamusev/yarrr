use yarrr::prelude::*;

fn create_scene() -> SphereScene {
    let m_right = Material::Metal(ColorRGB::new(0.8, 0.8, 0.8), 0.2);
    let m_left = Material::Metal(ColorRGB::new(0.8, 0.0, 0.4), 0.0);
    let m_center = Material::Lambertan(ColorRGB::new(0.2, 0.1, 0.9));
    let m_ground = Material::Lambertan(ColorRGB::new(0.2, 0.9, 0.4));

    let mut scene = SphereScene::new();
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
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 1600;
    let height = (width as f32 / aspect_ratio) as u32;
    let mut im = Image::new(width, height);

    // camera
    let vp = Viewport::new(2.0, aspect_ratio);
    let focal_length = 1.0;
    let cam_origin = Vector3D::zero();
    let cam = SimpleCamera::new(vp, focal_length, cam_origin);

    // scene
    let scene = create_scene();

    // render
    let settings = RenderSettings {
        samples_per_px: 100,
        bounce_depth: 5,
    };
    color_image(&mut im, cam, scene, settings);

    // save results
    image::save_buffer(
        "doc/materials_1600.jpeg",
        &im.as_bytes(),
        im.width,
        im.height,
        image::ColorType::Rgb8,
    )
    .expect("Unable to save image");
}
