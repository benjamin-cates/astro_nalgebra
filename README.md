# `astro_float` for `nalgebra`
This package implements the traits required to use the linear algebra library [nalgebra](https://docs.rs/nalgebra/latest/nalgebra) on the type [`BigFloat`](https://docs.rs/astro-float/latest/astro_float/struct.BigFloat.html) in [astro_float](https://docs.rs/astro_float/latest/astro_float). This library exports it's own type [`BigFloat<CTX>`] where `CTX` is a computational context that stores precision and rounding mode.

## Examples

### Compile-time const precision
Using this library with a precision and rounding mode that is known at compile-time is straight forward using the [`ConstCtx`] computational context. It is recommended to store `BigFloat<ConstCtx<P,RM>>` in a type alias so the code is more readable.
```rust
use astro_nalgebra::{BigFloat, ConstCtx, RoundingMode};
use nalgebra::Vector2;

// This defines a type that has a precision upper bound of
// 1024 bits in the mantissa and no explicit rounding mode 
type BF1024 = BigFloat<ConstCtx<1024>>;

// See the documentation on ConstCtx for how to specify a rounding mode

fn main() {
    let two: BF1024 = "2".parse().unwrap();
    let six: BF1024 = "6".parse().unwrap();
    let vec: Vector2<BF1024> = Vector2::new(two,six);
    let seven: BF1024 = "7".parse().unwrap();
    // Prints [2/7, 6/7] as decimals until 1024 bits
    println!("{}", vec / seven);
}
```

While it is completely allowed to name the type something like `f1024`, it does technically break the floating point naming scheme because the type `BigFloat<ConstCtx<64>>` has 64 bits in the mantissa, while types like `f64` only have 52 bits in the mantissa with 12 bits reserved for sign and exponent. So `f64` and `BigFloat<ConstCtx<64>>` are not the same.

### Run-time dynamic precision
Dynamic precision is more tricky to implement because some methods outlined in `nalgebra::RealField` do not have any arguments, so the precision has to be stored in the type. However run-time determined variables cannot be stored in a const generic, so there has to be a dummy type with the methods `get_prec` and `get_rm`. There is a macro to quickly define this dummy type which references a global, thread-safe `OnceLock` that has to be set at runtime.

Example:
```rust
use astro_nalgebra::{BigFloat, make_dyn_ctx, RoundingMode};

// This macro takes in the name of a dynamic context (the new type to be made)
// And the name of a global OnceLock to store the precision and rounding mode.
// The name of this global variable is not important, it just has to be unique
// within the scope of the macro call.
make_dyn_ctx!(DynCtx, DYN_CTX_CELL);

type DynFloat = BigFloat<DynCtx>;
fn main() {
    let precision = 88;
    let rounding_mode = RoundingMode::None;
    // Sets the precision and rounding mode of the DynCtx context.
    // This method can only be called once or it will panic.
    DynCtx::set(precision, rounding_mode);
    
    let num: DynFloat = "120".parse().unwrap();
}

```

## Why they have to be implemented with generics
There are two possible ways this library could have been implemented:
1. A very simple wrapper around `astro_float::BigFloat` that stored precision in the struct itself.
2. A wrapper that stores precision as a generic in the type.

The only way to guarantee proper accuracy is to use the latter technique. Here is an example of where the former technique would break:
```rust
use nalgebra::RealField;
fn mul_by_pi<T: RealField>(val: T) -> T {
    val * T::pi()
}
```
If the desired precision of the calculation was stored in val, then the call to `BigFloat::pi()` would not be able to access the precision because `RealField::pi()` does not take any arguments. Therefore, the desired precision has to be stored in the type itself.

