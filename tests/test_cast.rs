use astro_nalgebra::{self, BigFloat, ConstCtx, Sign};
use nalgebra::RealField;
use num_traits::FromPrimitive;

type BF64 = BigFloat<ConstCtx<64>>;
type BF128 = BigFloat<ConstCtx<128>>;
type BF256 = BigFloat<ConstCtx<256>>;

#[test]
fn test_f64_casts() {
    let floats_list: Vec<f64> = [
        vec![0., 1., 2.],
        vec![f64::MAX, f64::MIN],
        vec![-1., -2.],
        vec![2.1, 8.5, 100.5, 1.0000000000000001],
        vec![
            f64::INFINITY,
            f64::NEG_INFINITY,
            -f64::INFINITY,
            -f64::NEG_INFINITY,
        ],
    ]
    .into_iter()
    .flatten()
    .collect();
    for float in floats_list {
        assert_eq!(float, BF64::from_f64(float).unwrap().as_f64());
    }
    let super_exact_floats: Vec<(&str, f64)> = vec![
        ("1.0000000000000000000000000001", 1.0),
        ("12345.12345123450000001", 12345.1234512345),
    ];
    for (string, float) in super_exact_floats {
        assert_eq!(float, string.parse::<BF256>().unwrap().as_f64());
    }
    assert_eq!(BF128::pi().as_f64(), f64::pi());
}

#[test]
fn test_nan() {
    // Nan is Nan
    assert!(BF64::from_f64(f64::NAN).unwrap().as_f64().is_nan());
}

#[test]
fn test_from_u128() {
    for x in (0..500).chain((u128::MAX - 500)..u128::MAX) {
        let out = BF128::from_u128(x).unwrap().as_int().unwrap();
        assert_eq!(out.0, Sign::Pos);
        assert_eq!(x, out.1);
    }
    for x in -500..500 {
        let out = BF128::from_i32(x).unwrap().as_int().unwrap();
        assert_eq!(
            out.0,
            match x < 0 {
                true => Sign::Neg,
                false => Sign::Pos,
            }
        );
        assert_eq!(out.1, x.abs() as u128);
    }
}

macro_rules! cast_unsigned {
    ($type:ident, $func:ident) => {
        for x in (0..100).chain(($type::MAX - 10)..$type::MAX) {
            assert_eq!(
                <BF128 as TryInto<$type>>::try_into(BigFloat::$func(x).unwrap()).unwrap(),
                x
            );
        }
        assert_eq!(
            <BF128 as TryInto<$type>>::try_into(BigFloat::from_i32(-1).unwrap()),
            Err(())
        );
    };
}
#[test]
fn test_unsigned_casts() {
    cast_unsigned!(u8, from_u8);
    cast_unsigned!(u16, from_u16);
    cast_unsigned!(u32, from_u32);
    cast_unsigned!(u64, from_u64);
}

macro_rules! cast_signed {
    ($type:ident, $func:ident) => {
        for x in ($type::MIN..($type::MIN + 10))
            .chain(-10..10)
            .chain(($type::MAX - 10)..$type::MAX)
        {
            assert_eq!(
                <BF128 as TryInto<$type>>::try_into(BigFloat::$func(x).unwrap()).unwrap(),
                x
            );
        }
    };
}

#[test]
fn test_signed_casts() {
    cast_signed!(i8, from_i8);
    cast_signed!(i16, from_i16);
    cast_signed!(i32, from_i32);
    cast_signed!(i64, from_i64);
}
