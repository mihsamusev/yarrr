use yarrr::prelude::*;

const MAX_TOL_F32: f32 = 1e-6;

pub fn assert_almost_eq(left: f32, right: f32) {
    assert!((left - right).abs() < MAX_TOL_F32)
}

pub fn assert_vec_eq(left: &Vector3D, right: &Vector3D) {
    for i in 0..3 {
        assert_almost_eq(left[i], right[i]);
    }
}
