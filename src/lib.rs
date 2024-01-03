#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
use core::fmt::{Binary, Debug, Display, Octal, UpperHex};
use core::marker::PhantomData;

mod cast;
mod ctx;
mod impls;
mod ops;

// Re-exports
pub use crate::ctx::BigFloatCtx;
pub use crate::ctx::ConstCtx;
pub use crate::impls::num_traits::ParseBigFloatError;
pub use astro_float;
pub use astro_float::RoundingMode;
pub use astro_float::Sign;
pub use astro_float::EXPONENT_MAX;
pub use astro_float::EXPONENT_MIN;
pub use nalgebra;
pub use num_traits;

/// Arbitrary precision float type that is a wrapper around [`astro_float::BigFloat`]. Has trait
/// implementations to be used with [nalgebra] and [num_traits].
///
/// BigFloat has a type parameter (CTX) that can be either [`ConstCtx`] if the precision and
/// rounding mode is known at compile-time. Or one can be created with
/// [`make_dyn_ctx`] if the precision or rounding mode are not known at
/// compile time.
///
/// One limitation with this library is that only BigFloats with the same CTX parameter are allowed
/// to interop. This means that the precision of a computation system cannot be changed without
/// explicitly casting all variables.
///
/// **NOTE:** It is recommended to make a type alias such as the one below.
///
/// ## Example
/// ```rust
/// use astro_nalgebra::{BigFloat, ConstCtx};
///
/// type BF128 = BigFloat<ConstCtx<128>>;
///
/// let ten: BF128 = "10".parse().unwrap();
/// let seven: BF128 = "7".parse().unwrap();
/// println!("{}", ten / seven);
/// ```
///
/// [nalgebra]: https://docs.rs/nalgebra
/// [num_traits]: https://docs.rs/num_traits
#[derive(Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BigFloat<CTX: BigFloatCtx> {
    pub(crate) num: astro_float::BigFloat,
    pub(crate) _pd: PhantomData<fn() -> CTX>,
}

impl<CTX: BigFloatCtx> From<astro_float::BigFloat> for BigFloat<CTX> {
    #[inline]
    fn from(value: astro_float::BigFloat) -> Self {
        BigFloat {
            num: value,
            _pd: PhantomData,
        }
    }
}

impl<CTX: BigFloatCtx> Clone for BigFloat<CTX> {
    #[inline]
    fn clone(&self) -> Self {
        BigFloat {
            num: self.num.clone(),
            _pd: PhantomData,
        }
    }
}
impl<CTX: BigFloatCtx> PartialEq for BigFloat<CTX> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

macro_rules! impl_display_wrapper {
    ($trait_name:ident) => {
        #[doc(hidden)]
        impl<CTX: BigFloatCtx> core::fmt::$trait_name for BigFloat<CTX> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <astro_float::BigFloat as $trait_name>::fmt(&self.num, f)
            }
        }
    };
}
// Wrapper around Display method on BigFloat
impl_display_wrapper!(Debug);
impl_display_wrapper!(Display);
impl_display_wrapper!(Binary);
impl_display_wrapper!(Octal);
impl_display_wrapper!(UpperHex);
