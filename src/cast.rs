use crate::{BigFloat, BigFloatCtx};
use astro_float::Sign;

impl<CTX: BigFloatCtx> BigFloat<CTX> {
    /// Returns the closest [`f64`] to this BigFloat.
    ///
    /// If self is `NaN`, returns [`f64::NAN`]
    ///
    /// If self is `Inf`, returns [`f64::INFINITY`] or [`f64::NEG_INFINITY`]
    ///
    /// Otherwise, returns the closest `f64` value to this BigFloat.
    ///
    /// **NOTE:** If the value is above [`f64::MAX`], this function will overflow into `f64::INFINITY`.
    ///
    /// **NOTE:** This is function is defined within this package because `astro_float`'s
    /// implementation is private, so it is not an officially supported function.'
    pub fn as_f64(&self) -> f64 {
        if self.num.is_nan() {
            return f64::NAN;
        }
        if self.num.is_inf() {
            return if self.num.is_inf_pos() {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            };
        }
        // We can safely unwrap here because it always succeeds when it is not nan or inf
        let raw_parts = self.num.as_raw_parts().unwrap();
        let exp = raw_parts.3;
        let mantissa = raw_parts.0;
        match mantissa.last() {
            Some(val) => {
                let mut val = (*val as f64) / (u64::MAX as f64 + 1.) * 2.0;
                val *= 2.0_f64.powi(exp - 1);
                if self.num.is_negative() {
                    val *= -1.;
                }
                val
            }
            None => 0.0,
        }
    }
    /// Returns sign and integer as u128.
    /// If the absolute value is greater than u128::MAX, returns None.
    ///
    /// **NOTE:** it is much more idiomatic to use the [`BigFloat::try_into`] method.
    ///
    /// **NOTE:** This will truncate anything below 1 **without** giving a warning or error
    /// You can check if a number is an integer by ensuring that it is equivalent to its truncated
    /// form by using this snippet: `float.clone().trunc() - float`
    ///
    ///
    /// ## Example
    /// ```rust
    /// use astro_nalgebra::{BigFloat,ConstCtx,Sign};
    /// let int: BigFloat<ConstCtx<128>> = "1234512345".parse().unwrap();
    /// if let Some((sign, integer)) = int.as_int() {
    ///     assert_eq!(integer,1234512345u128);
    ///     assert_eq!(sign, Sign::Pos);
    /// }
    /// else {
    ///     // Value greater than u128::MAX
    /// }
    /// ```
    ///
    ///
    pub fn as_int(&self) -> Option<(Sign, u128)> {
        let raw_parts = self.num.as_raw_parts()?;
        let sign = raw_parts.2;
        let exp = raw_parts.3;
        let mantissa = raw_parts.0;
        if exp > 128 {
            return None;
        }
        let mut val = 0u128;
        if mantissa.is_empty() {
            return Some((Sign::Pos, 0));
        }
        if exp < 64 {
            val += (*mantissa.last().unwrap() as u128) >> (64 - exp);
        } else {
            val += (*mantissa.last().unwrap() as u128) << (exp - 64);
        }
        if mantissa.len() >= 2 && exp > 64 {
            let second_to_last = mantissa[mantissa.len() - 2];
            val += (second_to_last as u128) >> (exp - 128);
        }
        Some((sign, val))
    }
}

macro_rules! cast_float {
    ($type:ty) => {
        impl<CTX: BigFloatCtx> From<BigFloat<CTX>> for $type {
            fn from(x: BigFloat<CTX>) -> Self {
                x.as_f64() as Self
            }
        }
    };
}

cast_float!(f64);
cast_float!(f32);

macro_rules! cast_unsigned {
    ($type:ty) => {
        impl<CTX: BigFloatCtx> TryFrom<BigFloat<CTX>> for $type {
            type Error = ();
            fn try_from(x: BigFloat<CTX>) -> Result<Self, Self::Error> {
                let out = match x.as_int() {
                    Some(val) => val,
                    None => return Err(()),
                };
                if out.0 == Sign::Neg {
                    return Err(());
                }
                <$type as TryFrom<u128>>::try_from(out.1).ok().ok_or(())
            }
        }
    };
}

cast_unsigned!(u128);
cast_unsigned!(u64);
cast_unsigned!(u32);
cast_unsigned!(u16);
cast_unsigned!(u8);

macro_rules! cast_signed {
    ($type:ty) => {
        impl<CTX: BigFloatCtx> TryInto<$type> for BigFloat<CTX> {
            type Error = ();
            fn try_into(self) -> Result<$type, Self::Error> {
                let out = self.as_int().ok_or(())?;
                let sign = match out.0 {
                    Sign::Neg => {
                        if out.1 == <$type>::MAX as u128 + 1 {
                            return Ok(<$type>::MIN);
                        }
                        -1
                    }
                    Sign::Pos => 1,
                };
                Ok(sign * <u128 as TryInto<$type>>::try_into(out.1).ok().ok_or(())?)
            }
        }
    };
}

cast_signed!(i128);
cast_signed!(i64);
cast_signed!(i32);
cast_signed!(i16);
cast_signed!(i8);
