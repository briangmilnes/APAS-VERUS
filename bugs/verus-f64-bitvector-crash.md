# Verus crash: f64 by(bit_vector) panics at bitvector_to_air.rs:424

**Verus revision:** `c78aa4958372cfa69e6cb38fd31997c881473271`
**Date:** 2026-03-27
**Severity:** Blocks native f64 bitvector proofs

## Reproducer

```rust
use vstd::prelude::*;

verus! {

proof fn f64_reflexive(x: f64)
    ensures !x.is_nan_spec() ==> x <= x,
{
    assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
}

} // verus!
```

## Expected

Verus attempts the bitvector proof (may succeed or hit rlimit).

## Actual

Verus panics:
```
bitvector_to_air.rs:424: unexpected float to bits coercion
```

## Notes

- f32 `by(bit_vector)` works for 1-2 variable assertions with range bounds.
- f64 `by(bit_vector)` crashes on all attempts.
- `is_nan_spec()` inside `by(bit_vector)` crashes on both f32 and f64.
- See `src/experiments/f64_ieee_total_order.rs` (RESULT: FAILS) for full test matrix.
