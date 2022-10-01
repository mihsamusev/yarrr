use rand::Rng;
use yarrr::prelude::*;

fn create_scene() -> SphereScene {
    let mut scene = SphereScene::new();

    // ground
    let m_ground = Material::Lambertan(ColorRGB::new(0.5, 0.5, 0.5));
    scene.add(Sphere::new(
        Vector3D::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    ));

    // 3 big beautiful spheres
    let material1 = Material::Dielectric(1.5);
    scene.add(Sphere::new(Vector3D::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertan(ColorRGB::new(0.4, 0.2, 0.1));
    scene.add(Sphere::new(Vector3D::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(ColorRGB::new(0.7, 0.6, 0.5), 0.0);
    scene.add(Sphere::new(Vector3D::new(4.0, 1.0, 0.0), 1.0, material3));

    // random small spheres on a grid with random materials
    let mut rng = rand::thread_rng();
    let scene_bound = Vector3D::new(4.0, 0.2, 0.0);
    let n = 11;
    for i in -n..=n {
        for j in -n..=n {
            let material_rand: u32 = rng.gen_range(0..100);
            let x = i as f32 + rng.gen_range(0.0..0.9);
            let y = j as f32 + rng.gen_range(0.0..0.9);
            let center = Vector3D::new(x, 0.2, y);

            if (center - scene_bound).norm() > 0.9 {
                let material = match material_rand {
                    // diffuse
                    0..=79 => {
                        let albedo = ColorRGB::random(0.0, 1.0) * ColorRGB::random(0.0, 1.0);
                        Material::Lambertan(albedo)
                    }
                    // metal
                    80..=94 => {
                        let albedo = ColorRGB::random(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Material::Metal(albedo, fuzz)
                    }
                    // glass
                    _ => Material::Dielectric(1.5),
                };
                scene.add(Sphere::new(center, 0.2, material));
            }
        }
    }

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

    let width = 1600;
    let height = (width as f32 / aspect_ratio) as u32;
    let mut im = Image::new(width, height);

    // scene
    let scene = create_scene();

    // render
    let settings = RenderSettings {
        samples_per_px: 100,
        bounce_depth: 5,
    };
    color_image(&mut im, cam, scene, settings);
    // print_ppm(&im);

    image::save_buffer(
        "doc/final_render_1600_2.jpeg",
        &im.as_bytes(),
        im.width,
        im.height,
        image::ColorType::Rgb8,
    )
    .expect("Ünable to save image");
}
