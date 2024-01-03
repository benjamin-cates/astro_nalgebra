use crate::{BigFloat, BigFloatCtx};
use nalgebra::ComplexField;
use nalgebra::Field;
use nalgebra::RealField;
use num_traits::FromPrimitive;
use num_traits::One;
use num_traits::Zero;

impl<CTX: BigFloatCtx> Field for BigFloat<CTX> {}

macro_rules! unary {
    ($name:ident) => {
        fn $name(self) -> Self {
            CTX::run(|ctx| {
                Self::from(
                    self.num
                        .$name(ctx.precision(), ctx.rounding_mode(), ctx.consts()),
                )
            })
        }
    };
    ($name:ident, no consts) => {
        #[inline]
        fn $name(self) -> Self {
            Self::from(self.num.$name(CTX::get_prec(), CTX::get_rm()))
        }
    };
    ($name:ident, no prec) => {
        #[inline]
        fn $name(self) -> Self {
            Self::from(self.num.$name())
        }
    };
    ($name:ident, $selfname:ident -> $val:expr) => {
        #[inline]
        fn $name($selfname) -> Self {
            $val
        }
    };
}

macro_rules! get_const {
    ($name:ident) => {
        fn $name() -> Self {
            Self::from(CTX::run(|ctx| {
                let p = ctx.precision();
                let rm = ctx.rounding_mode();
                ctx.consts().$name(p, rm)
            }))
        }
    };
    ($name:ident, pi frac $val:tt) => {
        fn $name() -> Self {
            CTX::run(|ctx| {
                let p = ctx.precision();
                let rm = ctx.rounding_mode();
                Self::from(ctx.consts().pi(p, rm)) / BigFloat::from_f64($val).unwrap()
            })
        }
    };
    ($name:ident, $val:expr) => {
        #[inline]
        fn $name() -> Self {
            $val
        }
    };
}

impl<CTX: BigFloatCtx + 'static> RealField for BigFloat<CTX> {
    #[inline(always)]
    fn is_sign_positive(&self) -> bool {
        self.num.is_positive()
    }
    #[inline(always)]
    fn is_sign_negative(&self) -> bool {
        self.num.is_negative()
    }
    fn max(self, other: Self) -> Self {
        // IEEE float says that NaN's should be ignored in the max operation
        if self.num.is_nan() {
            other
        } else if other.num.is_nan() {
            self
        } else {
            Self::from(self.num.max(&other.num))
        }
    }
    fn min(self, other: Self) -> Self {
        // IEEE float says that NaN's should be ignored in the min operation
        if self.num.is_nan() {
            other
        } else if other.num.is_nan() {
            self
        } else {
            Self::from(self.num.min(&other.num))
        }
    }
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::from(self.num.clamp(&min.num, &max.num))
    }
    #[inline(always)]
    fn copysign(mut self, sign: Self) -> Self {
        self.num.set_sign(sign.num.sign().unwrap());
        self
    }
    get_const!(frac_pi_2, pi frac 2.);
    get_const!(frac_pi_3, pi frac 3.);
    get_const!(frac_pi_4, pi frac 4.);
    get_const!(frac_pi_6, pi frac 6.);
    get_const!(frac_pi_8, pi frac 8.);
    get_const!(two_pi, pi frac 0.5);
    get_const!(pi);
    get_const!(e);
    get_const!(ln_2);
    get_const!(ln_10);
    get_const!(log2_e, Self::log2(Self::e()));
    get_const!(log10_e, Self::log10(Self::e()));
    #[inline]
    fn min_value() -> Option<Self> {
        let p = CTX::get_prec();
        Some(BigFloat::from(astro_float::BigFloat::min_value(p)))
    }
    #[inline]
    fn max_value() -> Option<Self> {
        let p = CTX::get_prec();
        Some(BigFloat::from(astro_float::BigFloat::max_value(p)))
    }
    get_const!(frac_1_pi, Self::recip(Self::pi()));
    get_const!(frac_2_pi, Self::from_f64(2.).unwrap() / Self::pi());
    get_const!(
        frac_2_sqrt_pi,
        Self::from_f64(2.).unwrap() / Self::pi().sqrt()
    );

    fn atan2(self, other: Self) -> Self {
        if other.num.is_zero() {
            if self.num.is_zero() {
                // Okay technically this should be NaN, but nalgebra
                // returns zero so it should copy that I guess
                Self::zero()
            } else if self.num.is_positive() {
                BigFloat::frac_pi_2()
            } else {
                -BigFloat::frac_pi_2()
            }
        } else if other.num.is_positive() {
            (self / other).atan()
        } else if self.num.is_negative() {
            (self / other).atan() - BigFloat::pi()
        } else {
            (self / other).atan() + BigFloat::pi()
        }
    }
}

impl<CTX: BigFloatCtx + 'static> ComplexField for BigFloat<CTX> {
    type RealField = Self;

    #[inline(always)]
    fn is_finite(&self) -> bool {
        !self.num.is_inf()
    }

    // Basic operations
    #[inline(always)]
    fn scale(self, factor: Self::RealField) -> Self {
        self * factor
    }
    #[inline(always)]
    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }
    fn hypot(self, other: Self) -> Self::RealField {
        let p = CTX::get_prec();
        let rm = CTX::get_rm();
        BigFloat::from(
            self.num
                .mul(&self.num, p, rm)
                .add(&other.num.mul(&other.num, p, rm), p, rm),
        )
        .sqrt()
    }
    unary!(recip, self -> Self::one() / self);

    // "Imaginary" number functions
    unary!(real, self -> self);
    #[inline(always)]
    fn from_real(re: Self::RealField) -> Self {
        re
    }
    unary!(imaginary, self -> Self::zero());
    fn modulus_squared(self) -> Self::RealField {
        Self::from(self.num.mul(&self.num, CTX::get_prec(), CTX::get_rm()))
    }
    unary!(modulus, self -> self.abs());
    #[inline(always)]
    fn argument(self) -> Self::RealField {
        if self.is_sign_negative() {
            Self::pi()
        } else {
            Self::zero()
        }
    }
    unary!(norm1, self -> self.abs());
    unary!(conjugate, self -> self);

    // Logarithmic
    unary!(ln);
    unary!(log2);
    unary!(log10);
    #[inline(always)]
    fn log(self, base: Self::RealField) -> Self {
        self.ln() / base.ln()
    }
    unary!(ln_1p, self -> (self + Self::one()).ln());

    // Exponential
    unary!(exp);
    unary!(exp2, self -> Self::from_f64(2.).unwrap().powf(self));
    #[inline(always)]
    fn exp_m1(self) -> Self {
        self.exp() - Self::one()
    }
    fn powi(self, n: i32) -> Self {
        if n >= 0 {
            BigFloat::from(self.num.powi(n as usize, CTX::get_prec(), CTX::get_rm()))
        } else {
            self.powf(BigFloat::<CTX>::from_i32(n).unwrap())
        }
    }
    fn powf(self, n: Self::RealField) -> Self {
        CTX::run(|ctx| {
            let p = ctx.precision();
            let rm = ctx.rounding_mode();
            BigFloat::from(self.num.pow(&n.num, p, rm, ctx.consts()))
        })
    }
    unary!(sqrt, no consts);
    unary!(cbrt, no consts);
    #[inline(always)]
    fn try_sqrt(self) -> Option<Self> {
        if self.num.is_negative() {
            None
        } else {
            Some(BigFloat::from(
                self.num.sqrt(CTX::get_prec(), CTX::get_rm()),
            ))
        }
    }
    #[inline(always)]
    fn powc(self, n: Self) -> Self {
        self.powf(n)
    }

    // Rounding and signage
    fn round(self) -> Self {
        // The astro_float round function actually rounds off to a specific precision
        // not to the integer level, so this is a custom implementation
        if self.num.is_inf() {
            if self.num.is_inf_pos() {
                Self::from(astro_float::INF_POS)
            } else {
                Self::from(astro_float::INF_NEG)
            }
        } else {
            Self::from((self + Self::from_f64(0.5).unwrap()).num.int())
        }
    }
    unary!(floor, no prec);
    unary!(ceil, no prec);
    unary!(abs, no prec);
    unary!(fract, no prec);
    fn trunc(self) -> Self {
        if self.num.is_inf() {
            if self.num.is_inf_pos() {
                Self::from(astro_float::INF_POS)
            } else {
                Self::from(astro_float::INF_NEG)
            }
        } else {
            Self::from(self.num.int())
        }
    }
    fn signum(self) -> Self {
        if self.num.is_nan() {
            self
        } else if self.num.is_positive() {
            Self::one()
        } else {
            -Self::one()
        }
        // Self::from(
        //     self.num
        //         .div(&self.num.abs(), CTX::get_prec(), CTX::get_rm()),
        // )
    }

    // Trigonometric
    unary!(sin);
    unary!(cos);
    unary!(tan);
    unary!(asin);
    unary!(acos);
    unary!(atan);
    unary!(sinh);
    unary!(cosh);
    unary!(tanh);
    unary!(asinh);
    unary!(acosh);
    unary!(atanh);
    fn sin_cos(self) -> (Self, Self) {
        CTX::run(|ctx| {
            let p = ctx.precision();
            let rm = ctx.rounding_mode();
            (
                BigFloat::from(self.num.sin(p, rm, ctx.consts())),
                BigFloat::from(self.num.cos(p, rm, ctx.consts())),
            )
        })
    }
}
