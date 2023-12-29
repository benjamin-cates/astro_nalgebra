use crate::BigFloat;
use crate::BigFloatCtx;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use nalgebra::ComplexField;
use num_traits::FromPrimitive;

impl<CTX: BigFloatCtx + 'static> RelativeEq<Self> for BigFloat<CTX> {
    fn default_max_relative() -> Self::Epsilon {
        Self::from_f64(2.0f64.powi(-(CTX::get_prec() as i32 - 4))).unwrap()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        _max_relative: Self::Epsilon,
    ) -> bool {
        if self == other {
            return true;
        }
        if self.num.is_inf() || other.num.is_inf() {
            return false;
        }
        let abs_diff = (self.clone() - other.clone()).abs();
        if abs_diff <= epsilon {
            return true;
        }
        abs_diff <= Self::from(self.num.abs().max(&other.num.abs())) * Self::default_max_relative()
    }
}

impl<CTX: BigFloatCtx> AbsDiffEq<Self> for BigFloat<CTX> {
    type Epsilon = Self;
    fn default_epsilon() -> Self::Epsilon {
        Self::from_f64(2.0f64.powi(-(CTX::get_prec() as i32 - 4))).unwrap()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.num
            .sub(&other.num, CTX::get_prec(), CTX::get_rm())
            .abs()
            .lt(&epsilon.num)
    }
}

impl<CTX: BigFloatCtx> UlpsEq for BigFloat<CTX> {
    fn default_max_ulps() -> u32 {
        4
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, _max_ulps: u32) -> bool {
        self.abs_diff_eq(other, epsilon)
    }
}
