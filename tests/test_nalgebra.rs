use num_traits::{One, Zero};
use std::ops::Mul;

use astro_nalgebra::{BigFloat, ConstCtx};
use nalgebra::{Matrix2, Vector3};

type BF128 = BigFloat<ConstCtx<128>>;

#[test]
fn test_vec3() {
    let strs = vec!["1.2345678901234567890123456789e-1", "1.e+1", "Inf"];
    let outs = vec!["1.2345678901234567890123456789e+0", "1.e+2", "Inf"];
    let mut vec: Vector3<BF128> = Vector3::new(
        strs[0].parse().unwrap(),
        strs[1].parse().unwrap(),
        strs[2].parse().unwrap(),
    );
    vec = vec.mul("10".parse::<BF128>().unwrap());
    assert_eq!(vec.x, outs[0].parse().unwrap());
    assert_eq!(vec.y, outs[1].parse().unwrap());
    assert_eq!(vec.z, outs[2].parse().unwrap());
    assert_eq!(vec.x.to_string(), outs[0]);
    assert_eq!(vec.y.to_string(), outs[1]);
    assert_eq!(vec.z.to_string(), outs[2]);
}

#[test]
fn test_matrix_inverse() {
    let mat: Matrix2<BF128> = Matrix2::new(
        "3".parse().unwrap(),
        "4".parse().unwrap(),
        "4".parse().unwrap(),
        "3".parse().unwrap(),
    );
    let inv = mat.clone().try_inverse().unwrap();
    let ident = inv * mat;
    assert!(ident.get((0, 0)).unwrap().is_one());
    assert!(ident.get((1, 0)).unwrap().is_zero());
    assert!(ident.get((0, 1)).unwrap().is_zero());
    assert!(ident.get((0, 0)).unwrap().is_one());
}
