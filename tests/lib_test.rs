use showballz::Vector3D;

const MAX_TOL: f32 = 1e-8;

fn assert_vec_eq(left: Vector3D, right: Vector3D, tol: f32) {
    let v_diff = left - right;
    assert!(v_diff.x < tol);
    assert!(v_diff.y < tol);
    assert!(v_diff.z < tol);
}

#[test]
fn test_vector_create() {
    let v = Vector3D::default();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_vector_index() {
    let v = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(v[0], 0.1);
    assert_eq!(v[1], 0.2);
    assert_eq!(v[2], 0.3);
}

#[test]
fn test_vector_add_and_eq() {
    let v1 = Vector3D {
        x: 0.5,
        y: -0.1,
        z: 0.0,
    };
    let v2 = Vector3D::new(1.5, 2.1, 3.0);
    let v3 = v1 + v2;
    assert_vec_eq(v3, Vector3D::new(2.0, 2.0, 3.0), MAX_TOL);
}

#[test]
fn test_vector_times_scalar() {
    let v = Vector3D::new(1.0, 2.0, 3.0);
    let v_scaled = v * 10.0;
    assert_vec_eq(v_scaled, Vector3D::new(10.0, 20.0, 30.0), MAX_TOL);
}

#[test]
fn test_vector_get_unit() {
    let v = Vector3D::new(4.0, -3.0, 0.0);
    let v_unit = v.unit();
    assert_vec_eq(v_unit, Vector3D::new(0.8, 0.6, 0.0), MAX_TOL);
}

#[test]
fn test_vector_norm() {
    let v1 = Vector3D::new(3.0, -4.0, 5.0);
    assert!((v1.norm_squared() - 50.0).abs() < MAX_TOL);
    assert!((v1.norm() - 50.0_f32.sqrt()).abs() < MAX_TOL);
}

#[test]
fn test_vector_dot_product() {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(-1.0, -2.0, 3.0);
    assert!((v1.dot(&v2) - 4.0).abs() < MAX_TOL);
}

#[test]
fn test_vector_cross_product() {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(3.0, 2.0, 1.0);
    assert_vec_eq(v1.cross(&v2), Vector3D::new(-4.0, 8.0, -4.0), MAX_TOL);
    //assert_vec_eq(v2.cross(&v1), Vector3D::new(4.0, -8.0, 4.0), MAX_TOL);
}
