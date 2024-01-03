// This file contains implementations for num_traits methods such as the FromPrimitive trait, One
// trait, Zero trait, and Num trait (from_str_radix).
use core::str::FromStr;

use crate::{BigFloat, BigFloatCtx};
use astro_float::Radix;
use astro_float::{self};
use nalgebra::RealField;
use num_traits::FromPrimitive;
use num_traits::Num;
use num_traits::{One, Zero};

macro_rules! from_prim {
    ($name: ident, $type:ty) => {
        fn $name(prim: $type) -> Option<Self> {
            Some(Self::from(astro_float::BigFloat::$name(
                prim,
                CTX::get_prec(),
            )))
        }
    };
}

impl<CTX: BigFloatCtx> FromPrimitive for BigFloat<CTX> {
    from_prim!(from_f64, f64);
    from_prim!(from_f32, f32);

    from_prim!(from_u8, u8);
    from_prim!(from_i8, i8);
    from_prim!(from_u16, u16);
    from_prim!(from_i16, i16);
    from_prim!(from_u32, u32);
    from_prim!(from_i32, i32);
    from_prim!(from_i64, i64);
    from_prim!(from_u64, u64);
    from_prim!(from_i128, i128);
    from_prim!(from_u128, u128);
}

impl<CTX: BigFloatCtx> Zero for BigFloat<CTX> {
    #[inline]
    fn zero() -> Self {
        Self::from(astro_float::BigFloat::from_word(0, CTX::get_prec()))
    }
    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<CTX: BigFloatCtx> One for BigFloat<CTX> {
    #[inline]
    fn one() -> Self {
        Self::from(astro_float::BigFloat::from_word(1, CTX::get_prec()))
    }
    fn is_one(&self) -> bool {
        match self.num.precision() {
            Some(prec) => self.num == astro_float::BigFloat::from_word(1, prec),
            None => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseBigFloatError {
    InvalidRadix,
    InvalidNumber,
}

impl<CTX: BigFloatCtx> Num for BigFloat<CTX> {
    type FromStrRadixErr = ParseBigFloatError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let radix_enum = match radix {
            2 => Radix::Bin,
            8 => Radix::Oct,
            10 => Radix::Dec,
            16 => Radix::Hex,
            _ => return Err(ParseBigFloatError::InvalidRadix),
        };

        CTX::run(|ctx| {
            let value = astro_float::BigFloat::parse(
                str,
                radix_enum,
                ctx.precision(),
                ctx.rounding_mode(),
                ctx.consts(),
            );
            if value.is_nan() {
                Err(ParseBigFloatError::InvalidNumber)
            } else {
                Ok(BigFloat::from(value))
            }
        })
    }
}

impl<CTX: BigFloatCtx> FromStr for BigFloat<CTX> {
    type Err = ParseBigFloatError;
    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10)
    }
}

impl<CTX: BigFloatCtx + 'static> num_traits::Signed for BigFloat<CTX> {
    fn signum(&self) -> Self {
        if self.num.is_positive() {
            Self::one()
        } else if self.num.is_negative() {
            -Self::one()
        } else {
            Self::zero()
        }
    }
    #[inline]
    fn abs(&self) -> Self {
        BigFloat::from(self.num.abs())
    }
    #[inline]
    fn is_positive(&self) -> bool {
        self.num.is_positive()
    }
    #[inline]
    fn is_negative(&self) -> bool {
        self.num.is_negative()
    }
    fn abs_sub(&self, other: &Self) -> Self {
        (self.clone() - other.clone()).max(Self::zero())
    }
}
