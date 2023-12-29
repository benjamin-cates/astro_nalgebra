/// This file contains boilerplate for traits in the simba library
use crate::{BigFloat, BigFloatCtx};
use num_traits::FromPrimitive;
use simba::scalar::{SubsetOf, SupersetOf};
use simba::simd::SimdValue;

// Hidden because not important
#[doc(hidden)]
impl<CTX: BigFloatCtx> SubsetOf<Self> for BigFloat<CTX> {
    fn to_superset(&self) -> Self {
        self.clone()
    }
    fn from_superset_unchecked(element: &Self) -> Self {
        element.clone()
    }
    fn is_in_subset(_element: &Self) -> bool {
        true
    }
}
// Hidden because not important
#[doc(hidden)]
impl<CTX: BigFloatCtx> SupersetOf<f64> for BigFloat<CTX> {
    fn is_in_subset(&self) -> bool {
        true
    }
    fn to_subset_unchecked(&self) -> f64 {
        self.as_f64()
    }
    fn from_subset(element: &f64) -> Self {
        BigFloat::<CTX>::from_f64(*element).unwrap()
    }
}

// Hidden because not relevant to end users
#[doc(hidden)]
impl<CTX: BigFloatCtx> SimdValue for BigFloat<CTX> {
    type SimdBool = bool;
    type Element = Self;
    fn lanes() -> usize {
        1
    }
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
    fn splat(val: Self::Element) -> Self {
        val
    }
    fn extract(&self, i: usize) -> Self::Element {
        if i != 0 {
            panic!("Invalid lane");
        }
        self.clone()
    }
    fn replace(&mut self, i: usize, val: Self::Element) {
        if i != 0 {
            panic!("Invalid lane");
        }
        *self = val;
    }
    unsafe fn replace_unchecked(&mut self, _i: usize, val: Self::Element) {
        *self = val;
    }
    unsafe fn extract_unchecked(&self, _i: usize) -> Self::Element {
        self.clone()
    }
}
