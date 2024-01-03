extern crate alloc;
use alloc::collections::btree_map::{BTreeMap, Entry};
use astro_float::{ctx::Context, Consts, RoundingMode, EXPONENT_MAX, EXPONENT_MIN};
#[cfg(std)]
use core::cell::RefCell;
use core::fmt::Debug;

#[cfg(std)]
std::thread_local! {
/// Global constant that stores a constants cache for each context.
    pub(crate) static CONTEXTS: RefCell<BTreeMap<(usize, u8), Context>> = RefCell::new(BTreeMap::new());
}
#[cfg(not(std))]
pub(crate) static mut CONTEXTS: BTreeMap<(usize, u8), Context> = BTreeMap::new();

/// This trait specifies a type that has zero-argument methods that return a precision and a
/// rounding mode
///
/// **NOTE:** this trait should not be implemented by the user.
/// Please use [`ConstCtx`] or [`make_dyn_ctx`](crate::make_dyn_ctx).
///
pub trait BigFloatCtx {
    /// Returns the precision in bits of this specific context.
    /// This is either a const for [`ConstCtx`] or dynamic with
    /// [`make_dyn_ctx`](crate::make_dyn_ctx).
    fn get_prec() -> usize;
    /// Returns the [`RoundingMode`](astro_float::RoundingMode) of this specific context.
    /// This is either a const for [`ConstCtx`] or dynamic with
    /// [`make_dyn_ctx`](crate::make_dyn_ctx).
    fn get_rm() -> RoundingMode;

    /// Run the associated function, passing in an [`astro_float::ctx::Context`] as a mutable reference
    #[cfg(std)]
    fn run<F, R>(f: F) -> R
    where
        F: FnOnce(&mut astro_float::ctx::Context) -> R,
    {
        let p = Self::get_prec();
        let rm = Self::get_rm();
        // We can run borrow_mut without panicking because the variable is thread_local
        return CONTEXTS.with(|ctxs| match ctxs.borrow_mut().entry((p, rm as u8)) {
            Entry::Vacant(v) => {
                let context =
                    Context::new(p, rm, Consts::new().unwrap(), EXPONENT_MIN, EXPONENT_MAX);
                f(v.insert(context))
            }
            Entry::Occupied(mut o) => f(o.get_mut()),
        });
    }
    #[cfg(not(std))]
    fn run<F, R>(f: F) -> R
    where
        F: FnOnce(&mut astro_float::ctx::Context) -> R,
    {
        let p = Self::get_prec();
        let rm = Self::get_rm();
        // We need an unsafe block because it is a global static in the no_std environment
        // This is, however, okay because it is going to be single threaded
        unsafe {
            match CONTEXTS.entry((p, rm as u8)) {
                Entry::Vacant(v) => {
                    let context =
                        Context::new(p, rm, Consts::new().unwrap(), EXPONENT_MIN, EXPONENT_MAX);
                    f(v.insert(context))
                }
                Entry::Occupied(mut o) => f(o.get_mut()),
            }
        }
    }
}

/// Computation context for [`BigFloat`](crate::BigFloat) that has a compile-time constant precision and rounding
/// mode. This tag struct is required to specify what the precision result will be for methods in
/// [nalgebra] that do not take any arguments such as [`RealField::pi`](nalgebra::RealField::pi).
///
/// Example
/// ```rust
/// use astro_nalgebra::{BigFloat, ConstCtx, RoundingMode};
///
/// // This defines a type that has a precision upper bound of
/// // 1024 bits in the mantissa and no explicit rounding mode
/// type BF1024 = BigFloat<ConstCtx<1024>>;
///
/// // This defines a type that has a precision upper bound of 256 bits in the
/// // mantissa and rounds up for all imprecise operations.
/// // Note that casting cannot be inlined into the const generic, so it has to
/// // be declared as a constant outside and then referenced.
/// const UP: u8 = RoundingMode::Up as u8;
/// type BF256Up = BigFloat<ConstCtx<256,UP>>;
///
/// fn main() {
///     let two: BF1024 = "2".parse().unwrap();
/// }
/// ```
///
/// This struct is meant to be used as a tag, so it is never actually constructable.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ConstCtx<const P: usize, const RM: u8 = 1> {
    _private: (),
}

impl<const P: usize, const RM: u8> BigFloatCtx for ConstCtx<P, RM> {
    #[inline]
    fn get_prec() -> usize {
        P
    }
    #[inline]
    fn get_rm() -> RoundingMode {
        match RM {
            1 => astro_float::RoundingMode::None,
            2 => astro_float::RoundingMode::Up,
            4 => astro_float::RoundingMode::Down,
            8 => astro_float::RoundingMode::ToZero,
            16 => astro_float::RoundingMode::FromZero,
            32 => astro_float::RoundingMode::ToEven,
            64 => astro_float::RoundingMode::ToOdd,
            _ => panic!("Invalid rounding mode index: {}", RM),
        }
    }
}

/// Creates a dynamic context with a precision and rounding mode
/// that can be set once at run-time.
///
/// This macro takes in the name of a dynamic context (the new type to be made)
/// And the name of a global [`OnceLock`](std::sync::OnceLock) to store the precision and rounding mode.
/// The name of this global variable is not important, it just has to be unique
/// within the scope of the macro call.
///
/// The method `{ContextName}::set(precision, rounding_mode)` when called will
/// set the precision and rounding mode of that context. Calling set twice will cause a panic.
///
/// ## Example
/// ```rust
/// use astro_nalgebra::{BigFloat, make_dyn_ctx, RoundingMode};
///
/// make_dyn_ctx!(DynCtx, DYN_CTX_CELL);
///
/// type DynFloat = BigFloat<DynCtx>;
/// fn main() {
///     assert_eq!(DynCtx::is_set(), false);
///     let precision = 88;
///     let rounding_mode = RoundingMode::None;
///     // Sets the precision and rounding mode of the DynCtx context.
///     // This method will panic if it is called twice.
///     DynCtx::set(precision, rounding_mode);
///     // Context is now set, do not call DynCtx::set again
///     assert_eq!(DynCtx::is_set(), true);
///     let num: DynFloat = "120".parse().unwrap();
/// }
/// ```
#[cfg(std)]
#[macro_export]
macro_rules! make_dyn_ctx {
    ($type_name:ident, $singleton_name:ident) => {
        #[derive(Clone, PartialEq, Debug, Copy)]
        pub struct $type_name {
            _private: (),
        }
        static $singleton_name: core::sync::OnceLock<(usize, astro_nalgebra::RoundingMode)> =
            core::sync::OnceLock::new();
        impl $type_name {
            fn set(prec: usize, rm: astro_nalgebra::RoundingMode) {
                $singleton_name
                    .set((prec, rm))
                    .expect("Cannot set dynamic precision twice");
            }
            fn is_set() -> bool {
                $singleton_name.get().is_some()
            }
        }
        impl astro_nalgebra::BigFloatCtx for $type_name {
            fn get_prec() -> usize {
                $singleton_name.get().unwrap().0
            }
            fn get_rm() -> astro_nalgebra::RoundingMode {
                $singleton_name.get().unwrap().1
            }
        }
    };
}

/// Creates a dynamic context with a precision and rounding mode
/// that can be set once at run-time.
///
/// This macro takes in the name of a dynamic context (the new type to be made)
/// And the name of a global [`OnceLock`](std::sync::OnceLock) to store the precision and rounding mode.
/// The name of this global variable is not important, it just has to be unique
/// within the scope of the macro call.
///
/// The method `{ContextName}::set(precision, rounding_mode)` when called will
/// set the precision and rounding mode of that context. Calling set twice will cause a panic.
///
/// ## Example
/// ```rust
/// use astro_nalgebra::{BigFloat, make_dyn_ctx, RoundingMode};
///
/// make_dyn_ctx!(DynCtx, DYN_CTX_CELL);
///
/// type DynFloat = BigFloat<DynCtx>;
/// fn main() {
///     assert_eq!(DynCtx::is_set(), false);
///     let precision = 88;
///     let rounding_mode = RoundingMode::None;
///     // Sets the precision and rounding mode of the DynCtx context.
///     // This method will panic if it is called twice.
///     DynCtx::set(precision, rounding_mode);
///     // Context is now set, do not call DynCtx::set again
///     assert_eq!(DynCtx::is_set(), true);
///     let num: DynFloat = "120".parse().unwrap();
/// }
/// ```
#[cfg(not(std))]
#[macro_export]
macro_rules! make_dyn_ctx {
    ($type_name:ident, $singleton_name:ident) => {
        #[derive(Clone, PartialEq, Debug, Copy)]
        pub struct $type_name {
            _private: (),
        }
        static mut $singleton_name: (usize, astro_nalgebra::RoundingMode) =
            (0, astro_nalgebra::RoundingMode::None);
        impl $type_name {
            fn set(prec: usize, rm: astro_nalgebra::RoundingMode) {
                unsafe { $singleton_name = (prec, rm) }
            }
            fn is_set() -> bool {
                unsafe { $singleton_name.0 != 0 }
            }
        }
        impl astro_nalgebra::BigFloatCtx for $type_name {
            fn get_prec() -> usize {
                unsafe { $singleton_name.0 }
            }
            fn get_rm() -> astro_nalgebra::RoundingMode {
                unsafe { $singleton_name.1 }
            }
        }
    };
}
