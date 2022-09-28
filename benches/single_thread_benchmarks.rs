use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::rc::Rc;
use yarrr::body::{HittableScene, Sphere};
use yarrr::image::ColorRGB;
use yarrr::linalg::Vector3D;
use yarrr::material::Material;
use yarrr::prelude::Scatter;
use yarrr::ray::{HitRecord, Hittable, Ray};

fn make_random_vector() -> Vector3D {
    Vector3D::random(0.0, 1.0)
}

fn unit_sphere_sample() -> Vector3D {
    Vector3D::unit_sphere_sample()
}

fn hit_one_sphere() {
    let sphere = Sphere::new(Vector3D::new(2.0, 0.0, 0.0), 0.5, Material::None);
    let ray = Ray::new(Vector3D::zero(), Vector3D::unit_x());
    sphere.hit(&ray, 0.00001, f32::MAX);
}

fn hit_n_spheres(n: u32) {
    let mut scene = HittableScene::new();
    for i in 0..n {
        let x = 10000.0 - 2.0 * (i as f32);
        let sphere = Sphere::new(Vector3D::new(x, 0.0, 0.0), 0.5, Material::None);
        scene.add(Rc::new(sphere));
    }
    let ray = Ray::new(Vector3D::zero(), Vector3D::unit_x());
    scene.hit(&ray, 0.00001, f32::MAX);
}

fn lambertian_material_scatter() {
    let material = Material::Lambertan(ColorRGB::new(0.5, 0.5, 0.5));
    let incoming_ray = Ray::new(Vector3D::new(-1.0, -1.0, 0.0), Vector3D::new(1.0, 1.0, 0.0));
    let hit_record = HitRecord::new(Vector3D::zero(), 1.0, -Vector3D::unit_x(), &material);
    Material::scatter(&incoming_ray, &hit_record);
}

fn metal_material_scatter() {
    let material = Material::Metal(ColorRGB::new(0.5, 0.5, 0.5), 0.5);
    let incoming_ray = Ray::new(Vector3D::new(-1.0, -1.0, 0.0), Vector3D::new(1.0, 1.0, 0.0));
    let hit_record = HitRecord::new(Vector3D::zero(), 1.0, -Vector3D::unit_x(), &material);
    Material::scatter(&incoming_ray, &hit_record);
}

fn dielectric_material_scatter() {
    let material = Material::Dielectric(0.5);
    let incoming_ray = Ray::new(Vector3D::new(-1.0, -1.0, 0.0), Vector3D::new(1.0, 1.0, 0.0));
    let hit_record = HitRecord::new(Vector3D::zero(), 1.0, -Vector3D::unit_x(), &material);
    Material::scatter(&incoming_ray, &hit_record);
}

fn no_material_scatter() {
    let material = Material::None;
    let incoming_ray = Ray::new(Vector3D::new(-1.0, -1.0, 0.0), Vector3D::new(1.0, 1.0, 0.0));
    let hit_record = HitRecord::new(Vector3D::zero(), 1.0, -Vector3D::unit_x(), &material);
    Material::scatter(&incoming_ray, &hit_record);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("random vector", |b| b.iter(make_random_vector));
    c.bench_function("unit sphere sample", |b| b.iter(unit_sphere_sample));
    c.bench_function("hit 1 sphere", |b| b.iter(hit_one_sphere));
    c.bench_function("hit n spheres", |b| b.iter(|| hit_n_spheres(black_box(5))));
    c.bench_function("lambertan scatter", |b| b.iter(lambertian_material_scatter));
    c.bench_function("metal scatter", |b| b.iter(metal_material_scatter));
    c.bench_function("dielectric_scatter", |b| {
        b.iter(dielectric_material_scatter)
    });
    c.bench_function("no material scatter", |b| b.iter(no_material_scatter));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
