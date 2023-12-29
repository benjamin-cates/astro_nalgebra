use astro_nalgebra::{self, BigFloat, ConstCtx};
use num_traits::Num;

type BF128 = BigFloat<ConstCtx<128>>;

macro_rules! base_parse_tests {
    ($tests:ident, $base:literal, $formatter:literal) => {
        for (string, output, num) in $tests.into_iter() {
            let big_float = BF128::from_str_radix(string, $base).unwrap();
            let to_string = format!($formatter, big_float);
            assert_eq!(to_string.as_str(), output);
            assert_eq!(big_float.as_f64(), num);
        }
    };
}

#[test]
fn test_string_parse_base10() {
    let tests: Vec<(&str, &str, f64)> = vec![
        ("1", "1.e+0", 1.),
        ("0", "0.0", 0.),
        ("10000", "1.e+4", 10000.),
        ("1.1", "1.1e+0", 1.1),
        ("1.5", "1.5e+0", 1.5),
        ("0.5", "5.e-1", 0.5),
        ("Inf", "Inf", f64::INFINITY),
        ("-Inf", "-Inf", f64::NEG_INFINITY),
    ];
    base_parse_tests!(tests, 10, "{}");
    let float: BF128 = "1.00000000000000000001".parse().unwrap();
    assert_eq!("1.00000000000000000001e+0", float.to_string());
}

#[test]
fn test_string_parse_base2() {
    let tests: Vec<(&str, &str, f64)> = vec![
        ("1", "1.e+0", 1.),
        ("0", "0.0", 0.),
        ("10", "1.e+1", 2.),
        ("100", "1.e+10", 4.),
        ("1000", "1.e+11", 8.),
        ("10000", "1.e+100", 16.),
        ("1.1", "1.1e+0", 1.5),
        ("1.110", "1.11e+0", 1.75),
        ("0.1", "1.e-1", 0.5),
        ("Inf", "Inf", f64::INFINITY),
        ("-Inf", "-Inf", f64::NEG_INFINITY),
    ];
    base_parse_tests!(tests, 2, "{:b}");
}

#[test]
fn test_string_parse_base8() {
    let tests: Vec<(&str, &str, f64)> = vec![
        ("1", "1.e+0", 1.),
        ("0", "0.0", 0.),
        ("10000", "1.e+4", 4096.),
        ("1.1", "1.1e+0", 1.125),
        ("1.11000", "1.11e+0", 1.140625),
        ("0.1", "1.e-1", 0.125),
        ("Inf", "Inf", f64::INFINITY),
        ("-Inf", "-Inf", f64::NEG_INFINITY),
    ];
    base_parse_tests!(tests, 8, "{:o}");
}

#[test]
fn test_string_parse_base16() {
    let tests: Vec<(&str, &str, f64)> = vec![
        ("1", "1._e+0", 1.),
        ("0", "0.0", 0.),
        ("10000", "1._e+4", 65536.),
        ("1.1", "1.1_e+0", 1.0625),
        ("1.11000", "1.11_e+0", 1.06640625),
        ("0.1", "1._e-1", 0.0625),
        ("A", "A._e+0", 10.),
        ("FF", "F.F_e+1", 255.),
        ("Inf", "Inf", f64::INFINITY),
        ("-Inf", "-Inf", f64::NEG_INFINITY),
    ];
    base_parse_tests!(tests, 16, "{:X}");
}

#[test]
fn test_string_parse_errors() {
    for x in 0..=26 {
        if x == 2 || x == 8 || x == 10 || x == 16 {
            continue;
        }
        assert_eq!(
            BF128::from_str_radix("123", x),
            Err(astro_nalgebra::ParseBigFloatError::InvalidRadix)
        );
    }
    assert_eq!(
        BF128::from_str_radix("()", 10),
        Err(astro_nalgebra::ParseBigFloatError::InvalidNumber)
    );
}
