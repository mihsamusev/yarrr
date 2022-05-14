mod utils;
use utils::assert_vec_eq;
use yarrr::prelude::*;

#[test]
fn test_create_ray_unit_direction() {
    let orig = Vector3D::new(0.0, 1.0, 2.0);
    let dir = Vector3D::new(1.0, 1.0, 1.0);
    let r = Ray::new(orig, dir);

    let expected_dir = Vector3D::new(
        1.0 / f32::sqrt(3.0),
        1.0 / f32::sqrt(3.0),
        1.0 / f32::sqrt(3.0),
    );

    assert_vec_eq(&r.direction, &expected_dir);
}

#[test]
fn test_ray_at_value() {
    let orig = Vector3D::new(0.0, 1.0, 2.0);
    let dir = Vector3D::new(3.0, 4.0, 0.0);
    let r = Ray::new(orig, dir);

    let dest = r.at(0.0);
    assert_vec_eq(&dest, &orig);

    let dest = r.at(2.0);
    assert_vec_eq(&dest, &Vector3D::new(1.2, 2.6, 2.0));

    let dest = r.at(-3.5);
    assert_vec_eq(&dest, &Vector3D::new(-2.1, -1.8, 2.0));
}
