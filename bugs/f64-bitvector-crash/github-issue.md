# Title: Panic in bitvector_to_air when using is_nan_spec() inside by(bit_vector) on f64

## Description

Using `is_nan_spec()` (from `FloatBitsProperties`) inside a `by(bit_vector)` assertion on `f64` causes Verus to panic with "internal error: unexpected float to bits coercion" at `bitvector_to_air.rs:424`.

## PoC

```rust
use vstd::prelude::*;
use vstd::float::FloatBitsProperties;

verus! {

proof fn f64_reflexive_nan(x: f64)
    ensures !x.is_nan_spec() ==> x <= x,
{
    assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
}

} // verus!

fn main() {}
```

## Panic Log

```
thread '<unnamed>' panicked at vir/src/bitvector_to_air.rs:424:37:
internal error: unexpected float to bits coercion
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: vir::bitvector_to_air::bv_exp_to_expr
   ...
   8: vir::bitvector_to_air::bv_maybe_split
   ...
  16: vir::sst_to_air::body_stm_to_air
  17: vir::sst_to_air_func::func_sst_to_air
```

## Notes

- Range-bound f64 assertions without `is_nan_spec` work: `assert(0.0f64 <= x ==> x <= x) by(bit_vector)` verifies.
- f32 with `is_nan_spec` also crashes.
- The crash is in the bitvector-to-AIR translation, not in Z3.
