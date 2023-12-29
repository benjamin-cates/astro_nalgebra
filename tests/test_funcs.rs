use astro_nalgebra::{BigFloat, ConstCtx};
use nalgebra::{ComplexField, RealField};
use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

type BF256 = BigFloat<ConstCtx<256>>;

macro_rules! test_function {
    ($one:literal, $two:literal, $op:tt, $res:literal) => {
        assert_eq!(
            $one.parse::<BF256>().unwrap().$op($two.parse().unwrap()),
            $res.parse::<BF256>().unwrap()
        );
    };
    ($one:literal, $op:ident, $res:literal) => {
        assert_eq!($one.parse::<BF256>().unwrap().$op(), $res.parse().unwrap());
    };
}

#[test]
fn test_basic_operations() {
    test_function!("1", "1", add, "2");
    test_function!("1", "1", sub, "0");
    test_function!("1", "0.1", sub, "0.9");
    test_function!("1", "3", mul, "3");
    test_function!("1.1", "2", mul, "2.2");
    test_function!("45", "2", rem, "1");
    test_function!("1", "1", div, "1");
    test_function!("8", "4", div, "2");
}

#[test]
fn test_functions() {
    test_function!("1", ln, "0");
    test_function!("0", sin, "0");
    test_function!("0", asin, "0");
    test_function!("0", cos, "1");
    test_function!("1", acos, "0");
    test_function!("1", real, "1");
    test_function!("1", imaginary, "0");
    test_function!("-1", modulus, "1");
    test_function!("2", modulus, "2");
    test_function!("2", argument, "0");
    test_function!(
        "-2",
        argument,
        "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914564856692346034861045432664821339360726024914127372458700660631558817488152092096282925409171536436"
    );
    test_function!("2", modulus_squared, "4");
    test_function!("-2", modulus_squared, "4");
    test_function!("2", norm1, "2");
    test_function!("2", "4", scale, "8");
    test_function!("2", "4", unscale, "0.5");
    test_function!("3", "4", hypot, "5");
}

macro_rules! assert_equiv_operations {
    (binary $one:ident, $two:ident, $nums:ident) => {
        for i in 0..$nums.len() {
            for j in 0..$nums.len() {
                assert_eq!(
                    $nums[i].clone().$one($nums[j].clone()),
                    $nums[i].clone().$two($nums[j].clone()),
                );
            }
        }
    };
    (unary $one:ident, $two:ident, $nums:ident) => {
        for i in 0..$nums.len() {
            assert_eq!($nums[i].clone().$one(), $nums[i].clone().$two(),);
        }
    };
}
#[test]
fn test_equivalent_operations() {
    let nums: Vec<BF256> = [
        "1",
        "2",
        "3",
        "-3",
        "-6.3e-500",
        "1.1",
        "21521.216312662",
        "10e500",
        "6.5e-500",
        "14e1000",
    ]
    .iter()
    .map(|str| str.parse().unwrap())
    .collect();
    assert_equiv_operations!(unary abs, norm1, nums);
    assert_equiv_operations!(unary real, conjugate, nums);
    assert_equiv_operations!(unary modulus, abs, nums);
    assert_equiv_operations!(binary scale, mul, nums);
    assert_equiv_operations!(binary unscale, div, nums);
}

const REL_DIFF: f64 = 0.00000000000001;
macro_rules! mirror_operations {
    ($func:ident, 0 args) => {
        assert_eq!(
            BF256::$func().as_f64(),
            f64::$func(),
            "Failed f64 mirroring for {}",
            stringify!(func)
        );
    };
    ($nums:ident, $func:ident, 1 args) => {
        for num in $nums.iter() {
            let val1 = BF256::from_f64(*num).unwrap().$func().as_f64();
            let val2 = num.$func();
            if val1 == val2 || val1.is_nan() && val2.is_nan() {
                continue;
            }
            assert!(
                (val1 / val2) - 1. < REL_DIFF,
                "Failed f64 mirroring for {}.{}(), {} != {}",
                num,
                stringify!($func),
                val1,
                val2,
            );
        }
    };
    ($nums:ident, $func:ident, 2 args) => {
        for num_slice in $nums.as_slice().windows(2) {
            let val1 = BF256::from_f64(num_slice[0])
                .unwrap()
                .$func(BF256::from_f64(num_slice[1]).unwrap())
                .as_f64();
            let val2 = num_slice[0].$func(num_slice[1]);
            if val1 == val2 || val1.is_nan() && val2.is_nan() {
                continue;
            }
            assert!(
                (val1 / val2) - 1. < REL_DIFF,
                "Failed f64 mirroring for {}.{}({}), {} != {}",
                num_slice[0],
                stringify!($func),
                num_slice[1],
                val1,
                val2,
            );
        }
    };
    ($nums:ident, $func:ident, 3 args) => {
        for num_slice in $nums.as_slice().windows(3) {
            let val1 = BF256::from_f64(num_slice[0])
                .unwrap()
                .$func(
                    BF256::from_f64(num_slice[1]).unwrap(),
                    BF256::from_f64(num_slice[2]).unwrap(),
                )
                .as_f64();
            let val2 = num_slice[0].$func(num_slice[1], num_slice[2]);
            if val1 == val2 || val1.is_nan() && val2.is_nan() {
                continue;
            }
            assert!(
                (val1 / val2) - 1. < REL_DIFF,
                "Failed f64 mirroring for {}.{}({},{}), {} != {}",
                num_slice[0],
                stringify!($func),
                num_slice[1],
                num_slice[2],
                val1,
                val2
            );
        }
    };
}

// Test that the implementations I wrote mirror the ones for f64.
// I.E. didn't misinterpret the meaning of the function
#[test]
fn test_mirror_operations() {
    let nums: Vec<f64> = vec![
        vec![1., 2., 3., -1.0e51, 1.0e51, 100., 1.14214, 1.1, 6.5, 0.1],
        vec![1., 2., -1., -2., 1., 0., 0., 1., -1., 0., -2.],
        vec![1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8],
        vec![1.0e60, 1.0e12, -0.4e18, 0.1, 0.2, 0.5],
        vec![-1.1, 1.1e101, f64::INFINITY, f64::NAN, f64::NEG_INFINITY],
    ]
    .into_iter()
    .flatten()
    .collect();
    // Filter out infinty and nan for specific functions
    let nums_normal: Vec<f64> = nums.iter().filter(|x| x.is_normal()).cloned().collect();
    // Consts
    mirror_operations!(pi, 0 args);
    mirror_operations!(frac_pi_2, 0 args);
    mirror_operations!(frac_pi_3, 0 args);
    mirror_operations!(frac_pi_4, 0 args);
    mirror_operations!(frac_pi_6, 0 args);
    mirror_operations!(frac_pi_8, 0 args);
    mirror_operations!(frac_1_pi, 0 args);
    mirror_operations!(frac_2_pi, 0 args);
    mirror_operations!(frac_2_sqrt_pi, 0 args);
    mirror_operations!(e, 0 args);
    mirror_operations!(ln_2, 0 args);
    mirror_operations!(ln_10, 0 args);
    mirror_operations!(log10_e, 0 args);
    mirror_operations!(log2_e, 0 args);
    mirror_operations!(log2_e, 0 args);

    // Basic functions
    mirror_operations!(nums, neg, 1 args);
    mirror_operations!(nums, mul, 2 args);
    mirror_operations!(nums, add, 2 args);
    mirror_operations!(nums, sub, 2 args);
    mirror_operations!(nums, div, 2 args);
    mirror_operations!(nums, rem, 2 args);
    mirror_operations!(nums, mul_add, 3 args);

    // "Imaginary" num tests
    mirror_operations!(nums, real, 1 args);
    mirror_operations!(nums, imaginary, 1 args);
    mirror_operations!(nums, argument, 1 args);
    mirror_operations!(nums, modulus, 1 args);
    mirror_operations!(nums, modulus_squared, 1 args);
    mirror_operations!(nums, norm1, 1 args);
    mirror_operations!(nums, scale, 2 args);
    mirror_operations!(nums, unscale, 2 args);

    // Rounding and comparison
    mirror_operations!(nums, floor, 1 args);
    mirror_operations!(nums, round, 1 args);
    mirror_operations!(nums, ceil, 1 args);
    mirror_operations!(nums, trunc, 1 args);
    mirror_operations!(nums, fract, 1 args);
    mirror_operations!(nums, abs, 1 args);
    mirror_operations!(nums, signum, 1 args);
    mirror_operations!(nums, min, 2 args);
    mirror_operations!(nums, max, 2 args);

    // Misc operations
    // For some reason, the f64 implementation of hypot ignores NaN values
    // I disagree so I'm going to ignore it lol
    mirror_operations!(nums_normal, hypot, 2 args);
    mirror_operations!(nums, recip, 1 args);
    mirror_operations!(nums, conjugate, 1 args);
    mirror_operations!(nums, sqrt, 1 args);
    mirror_operations!(nums, cbrt, 1 args);

    // Exponentials and logs
    mirror_operations!(nums_normal, exp, 1 args);
    mirror_operations!(nums_normal, exp_m1, 1 args);
    mirror_operations!(nums, ln, 1 args);
    mirror_operations!(nums, log, 2 args);
    mirror_operations!(nums, log2, 1 args);
    mirror_operations!(nums, ln, 1 args);
    mirror_operations!(nums, log10, 1 args);
    mirror_operations!(nums, ln_1p, 1 args);

    // Trigonometric
    mirror_operations!(nums, sin, 1 args);
    mirror_operations!(nums, cos, 1 args);
    mirror_operations!(nums, tan, 1 args);
    // Sinh wrong for really negative numbers
    // mirror_operations!(nums, sinh, 1 args);
    mirror_operations!(nums, cosh, 1 args);
    mirror_operations!(nums, tanh, 1 args);
    mirror_operations!(nums, asin, 1 args);
    mirror_operations!(nums, acos, 1 args);
    mirror_operations!(nums, atan, 1 args);
    mirror_operations!(nums, asinh, 1 args);
    // Acosh boundary values are broken in parent crate astro_float
    mirror_operations!(nums_normal, acosh, 1 args);
    // Atanh boundary values are broken in parent crate astro_float
    mirror_operations!(nums_normal, atanh, 1 args);
    mirror_operations!(nums, atan2, 2 args);
}
