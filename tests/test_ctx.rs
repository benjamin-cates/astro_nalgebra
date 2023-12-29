use astro_nalgebra::{make_dyn_ctx, BigFloat, BigFloatCtx, ConstCtx, RoundingMode};

make_dyn_ctx!(DynCtx1, DYN_CTX_1);
#[test]
fn test_dynamic_precision() {
    let prec = 1024;
    let rm = RoundingMode::Up;
    DynCtx1::set(prec, rm);
    assert_eq!(DynCtx1::get_prec(), prec);
    assert_eq!(DynCtx1::get_rm(), rm);
    let x: BigFloat<DynCtx1> = "2".parse().unwrap();
    assert_eq!(x.as_f64(), 2.0);
}

make_dyn_ctx!(DynCtxNone, DYN_CTX_NONE);
make_dyn_ctx!(DynCtxUp, DYN_CTX_UP);
make_dyn_ctx!(DynCtxDown, DYN_CTX_DOWN);
make_dyn_ctx!(DynCtxToZero, DYN_CTX_TO_ZERO);
make_dyn_ctx!(DynCtxFromZero, DYN_CTX_FROM_ZERO);
make_dyn_ctx!(DynCtxToEven, DYN_CTX_TO_EVEN);
make_dyn_ctx!(DynCtxToOdd, DYN_CTX_TO_ODD);
macro_rules! test_dyn_ctx_rm {
    ($ctx_name:ident, $variant:ident) => {
        $ctx_name::set(1024, RoundingMode::$variant);
        assert_eq!($ctx_name::get_rm(), RoundingMode::$variant);
        assert_eq!($ctx_name::get_prec(), 1024);
    };
}
#[test]
fn test_dynamic_rounding_modes() {
    test_dyn_ctx_rm!(DynCtxNone, None);
    test_dyn_ctx_rm!(DynCtxUp, Up);
    test_dyn_ctx_rm!(DynCtxDown, Down);
    test_dyn_ctx_rm!(DynCtxToZero, ToZero);
    test_dyn_ctx_rm!(DynCtxFromZero, FromZero);
    test_dyn_ctx_rm!(DynCtxToEven, ToEven);
    test_dyn_ctx_rm!(DynCtxToOdd, ToOdd);
}

#[test]
fn test_const_precision() {
    assert_eq!(ConstCtx::<256>::get_prec(), 256);
    assert_eq!(ConstCtx::<1024>::get_prec(), 1024);
    assert_eq!(ConstCtx::<1>::get_prec(), 1);
}

macro_rules! test_const_rm {
    ($rm:expr) => {
        assert_eq!(
            astro_nalgebra::ConstCtx::<1024, { $rm as u8 }>::get_rm(),
            $rm
        );
    };
}

#[test]
fn test_const_rounding_modes() {
    test_const_rm!(RoundingMode::None);
    test_const_rm!(RoundingMode::Up);
    test_const_rm!(RoundingMode::Down);
    test_const_rm!(RoundingMode::ToZero);
    test_const_rm!(RoundingMode::FromZero);
    test_const_rm!(RoundingMode::ToEven);
    test_const_rm!(RoundingMode::ToOdd);
}
