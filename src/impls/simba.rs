/// This file contains boilerplate for traits in the simba library
use crate::{BigFloat, BigFloatCtx};
use num_traits::FromPrimitive;
use simba::scalar::{SubsetOf, SupersetOf};
use simba::simd::SimdValue;

// Hidden because not important
#[doc(hidden)]
impl<CTX: BigFloatCtx> SubsetOf<Self> for BigFloat<CTX> {
    #[inline(always)]
    fn to_superset(&self) -> Self {
        self.clone()
    }
    #[inline(always)]
    fn from_superset_unchecked(element: &Self) -> Self {
        element.clone()
    }
    #[inline(always)]
    fn is_in_subset(_element: &Self) -> bool {
        true
    }
}
// Hidden because not important
#[doc(hidden)]
impl<CTX: BigFloatCtx> SupersetOf<f64> for BigFloat<CTX> {
    #[inline(always)]
    fn is_in_subset(&self) -> bool {
        true
    }
    #[inline(always)]
    fn to_subset_unchecked(&self) -> f64 {
        self.as_f64()
    }
    #[inline(always)]
    fn from_subset(element: &f64) -> Self {
        BigFloat::<CTX>::from_f64(*element).unwrap()
    }
}

// Hidden because not relevant to end users
#[doc(hidden)]
impl<CTX: BigFloatCtx> SimdValue for BigFloat<CTX> {
    type SimdBool = bool;
    type Element = Self;
    #[inline(always)]
    fn lanes() -> usize {
        1
    }
    #[inline(always)]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    fn splat(val: Self::Element) -> Self {
        val
    }
    #[inline(always)]
    fn extract(&self, i: usize) -> Self::Element {
        if i != 0 {
            panic!("Invalid lane");
        }
        self.clone()
    }
    #[inline(always)]
    fn replace(&mut self, i: usize, val: Self::Element) {
        if i != 0 {
            panic!("Invalid lane");
        }
        *self = val;
    }
    #[inline(always)]
    unsafe fn replace_unchecked(&mut self, _i: usize, val: Self::Element) {
        *self = val;
    }
    #[inline(always)]
    unsafe fn extract_unchecked(&self, _i: usize) -> Self::Element {
        self.clone()
    }
}
