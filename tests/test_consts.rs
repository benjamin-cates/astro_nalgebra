use astro_nalgebra::{BigFloat, ConstCtx};
use nalgebra::{ComplexField, RealField};
use num_traits::FromPrimitive;

type BF1024 = BigFloat<ConstCtx<1024>>;

macro_rules! test_pi_fraction {
    ($func:ident, $frac:literal) => {
        assert_eq!(
            BF1024::$func(),
            BF1024::pi() / BF1024::from_f64($frac).unwrap()
        );
    };
}
#[test]
fn test_pis() {
    assert_eq!("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679",&format!("{}",BigFloat::<ConstCtx<2048>>::pi().to_string())[0..102]);
    test_pi_fraction!(two_pi, 0.5);
    test_pi_fraction!(frac_pi_2, 2.0);
    test_pi_fraction!(frac_pi_3, 3.0);
    test_pi_fraction!(frac_pi_4, 4.0);
    test_pi_fraction!(frac_pi_6, 6.0);
    test_pi_fraction!(frac_pi_8, 8.0);
    assert_eq!(
        BF1024::frac_1_pi(),
        BF1024::from_f64(1.).unwrap() / BF1024::pi()
    );
    assert_eq!(
        BF1024::frac_2_pi(),
        BF1024::from_f64(2.).unwrap() / BF1024::pi()
    );
    assert_eq!(
        BF1024::frac_2_sqrt_pi(),
        BF1024::from_f64(2.).unwrap() / BF1024::pi().sqrt()
    );
}

#[test]
fn test_e() {
    let e_str = "2.71828182845904523536028747135266249775724709369995957496696762772407663035354759457138217852516642742746639193200305992181741359662904357290033429526059563073813232862794349076323382988075319525101901157383418793070215408914993488416750924476146066808226480016847741185374234544243710753907774499206955170276183860626133138458300075204493382656029760673711320070932870912744374704723069697720931014169283681902551510865746377211125238978442505695369677078544996996794686445490598793163688923009879312";
    assert_eq!(
        e_str,
        &BigFloat::<ConstCtx<100000>>::e().to_string()[0..e_str.len()]
    )
}
