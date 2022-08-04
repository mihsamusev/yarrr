mod utils;
use utils::{assert_almost_eq, assert_vec_eq};
use yarrr::prelude::*;

#[test]
fn test_vector_create() {
    let v = Vector3D::default();
    assert_almost_eq(v.x, 0.0);
    assert_almost_eq(v.y, 0.0);
    assert_almost_eq(v.z, 0.0);
}

#[test]
fn test_vector_index() {
    let v = Vector3D::new(0.1, 0.2, 0.3);
    assert_almost_eq(v[0], 0.1);
    assert_almost_eq(v[1], 0.2);
    assert_almost_eq(v[2], 0.3);
}

#[test]
fn test_vector_add_and_eq() {
    let v1 = Vector3D::new(0.5, -0.1, 0.0);
    let v2 = Vector3D::new(1.5, 2.1, 3.0);
    let v3 = v1 + v2;
    assert_vec_eq(&v3, &Vector3D::new(2.0, 2.0, 3.0));
}

#[test]
fn test_vector_negate() {
    let v1 = Vector3D::new(0.5, -0.1, 0.0);
    let v2 = -v1;
    assert_vec_eq(&v2, &Vector3D::new(-0.5, 0.1, 0.0));
}

#[test]
fn test_vector_times_scalar() {
    let v = Vector3D::new(1.0, 2.0, 3.0);
    let v_scaled = v * 10.0;
    assert_vec_eq(&v_scaled, &Vector3D::new(10.0, 20.0, 30.0));
}

#[test]
fn test_vector_times_vector() {
    let v1 = Vector3D::new(-1.0, 2.0, 3.0);
    let v2 = Vector3D::new(1.0, -2.0, 3.0);
    let v_mul = v1 * v2;
    assert_vec_eq(&v_mul, &Vector3D::new(-1.0, -4.0, 9.0));
}

#[test]
fn test_vector_get_unit() {
    let v = Vector3D::new(4.0, -3.0, 0.0);
    let v_unit = v.unit();
    assert_vec_eq(&v_unit, &Vector3D::new(0.8, -0.6, 0.0));
}

#[test]
fn test_vector_norm() {
    let v1 = Vector3D::new(3.0, -4.0, 5.0);
    assert_almost_eq(v1.norm_squared(), 50.0);
    assert_almost_eq(v1.norm(), 50.0_f32.sqrt());
}

#[test]
fn test_vector_dot_product() {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(-1.0, -2.0, 3.0);
    assert_almost_eq(v1.dot(&v2), 4.0);
}

#[test]
fn test_vector_cross_product() {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(3.0, 2.0, 1.0);
    assert_vec_eq(&v1.cross(&v2), &Vector3D::new(-4.0, 8.0, -4.0));
    assert_vec_eq(&v2.cross(&v1), &Vector3D::new(4.0, -8.0, 4.0));
}

#[test]
fn test_reflect() {
    let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
    let vec = Vector3D::new(1.0, -1.0, 0.0);
    let reflected = reflect(&vec, &normal);
    assert_vec_eq(&reflected, &Vector3D::new(1.0, 1.0, 0.0));
}

#[test]
fn test_refract_same_medium() {
    let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
    let vec = Vector3D::new(1.0, -1.0, 0.0).unit();
    let refracted = refract(&vec, &normal, 1.0);
    assert_vec_eq(&refracted, &Vector3D::new(1.0, -1.0, 0.0).unit());
}

#[test]
fn test_refract_different_medium() {
    // based on this https://www.omnicalculator.com/physics/snells-law
    let refraction_index = 0.8 / 0.6;
    let angle_in = 30.0 * std::f32::consts::PI / 180.0;
    let angle_out = 41.8103 * std::f32::consts::PI / 180.0;

    let normal = Vector3D::new(0.0, 2.0, 0.0).unit();
    let vec = Vector3D::new(angle_in.sin(), -angle_in.cos(), 0.0).unit();
    let refracted = refract(&vec, &normal, refraction_index);
    dbg!(refracted);

    assert_vec_eq(
        &refracted,
        &Vector3D::new(angle_out.sin(), -angle_out.cos(), 0.0).unit(),
    );
}
