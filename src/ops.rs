// This file contains implementations for Neg, Add, Sub, Mul, Div, Rem, and their Assign variants
use crate::{BigFloat, BigFloatCtx};
use core::cmp;
use core::ops::Neg;
use core::ops::{Add, AddAssign};
use core::ops::{Div, DivAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Rem, RemAssign};
use core::ops::{Sub, SubAssign};

impl<CTX: BigFloatCtx> Neg for BigFloat<CTX> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::from(-self.num)
    }
}

macro_rules! binary_op {
    ($name:ident, $func:ident) => {
        impl<CTX: BigFloatCtx> $name<Self> for BigFloat<CTX> {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Self::from(self.num.$func(&rhs.num, CTX::get_prec(), CTX::get_rm()))
            }
        }
    };
}
macro_rules! binary_op_assign {
    ($name:ident, $func:ident, $calls:ident) => {
        impl<CTX: BigFloatCtx> $name<Self> for BigFloat<CTX> {
            #[inline]
            fn $func(&mut self, rhs: Self) {
                let out = self.num.$calls(&rhs.num, CTX::get_prec(), CTX::get_rm());
                self.num = out;
            }
        }
    };
}

binary_op!(Add, add);
binary_op!(Sub, sub);
binary_op!(Mul, mul);
binary_op!(Div, div);
binary_op_assign!(AddAssign, add_assign, add);
binary_op_assign!(SubAssign, sub_assign, sub);
binary_op_assign!(MulAssign, mul_assign, mul);
binary_op_assign!(DivAssign, div_assign, div);

impl<CTX: BigFloatCtx> Rem<Self> for BigFloat<CTX> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::from(self.num.rem(&rhs.num))
    }
}

impl<CTX: BigFloatCtx> RemAssign for BigFloat<CTX> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.num = self.num.rem(&rhs.num);
    }
}

impl<CTX: BigFloatCtx> PartialOrd<Self> for BigFloat<CTX> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}
