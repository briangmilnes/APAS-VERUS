# vstdplus - Verus Standard Library Extensions

Extensions and additions to Verus's `vstd` library, created for APAS-VERUS.

## Modules

### Ordering Traits

#### `TotalOrder` (`total_order.rs`)
A trait for types with total ordering, connecting spec-level `le` to executable `cmp`.

**Properties**:
- Reflexive: `x ≤ x`
- Transitive: `x ≤ y ∧ y ≤ z ⟹ x ≤ z`
- Antisymmetric: `x ≤ y ∧ y ≤ x ⟹ x = y`
- Total: `x ≤ y ∨ y ≤ x` (all pairs comparable)

**Implementations**: All integer types (`u8`-`u128`, `i8`-`i128`, `usize`, `isize`)

**Verification**: ✅ 60 verified, 0 errors

**Example**:
```rust
use vstdplus::total_order::total_order::TotalOrder;

fn example(x: u64, y: u64) {
    let result = TotalOrder::cmp(&x, &y);
    match result {
        Ordering::Less => { /* x < y */ },
        Ordering::Equal => { /* x == y */ },
        Ordering::Greater => { /* x > y */ },
    }
}
```

#### `PartialOrder` (`partial_order.rs`)
A trait for types with partial ordering, where some pairs may be incomparable (e.g., NaN in floats).

**Properties**:
- Reflexive: `x ≤ x`
- Transitive: `x ≤ y ∧ y ≤ z ⟹ x ≤ z`
- Antisymmetric: `x ≤ y ∧ y ≤ x ⟹ x = y`
- **No totality**: Not all pairs need be comparable

**Key difference from TotalOrder**: Returns `Option<Ordering>` where `None` indicates incomparable elements.

**Implementations**: 
- All integer types (always return `Some`)
- **Float types** (`f32`, `f64`) using uninterpreted specs

**Verification**: ✅ 54 verified, 0 errors

**Float handling**:
```rust
use vstdplus::partial_order::partial_order::PartialOrder;

fn float_example() {
    // Normal comparisons return Some
    assert_eq!(PartialOrder::compare(&5.0f32, &3.0f32), Some(Ordering::Greater));
    
    // NaN comparisons return None (incomparable)
    let nan = f32::NAN;
    assert_eq!(PartialOrder::compare(&nan, &5.0f32), None);
    assert_eq!(PartialOrder::compare(&nan, &nan), None);
    
    // Infinity works as expected
    assert_eq!(PartialOrder::compare(&f32::INFINITY, &1.0f32), Some(Ordering::Greater));
}
```

**Why uninterpreted specs for floats?**
- Rust float operations are non-deterministic ([RFC 3514](https://github.com/rust-lang/rfcs/blob/master/text/3514-float-semantics.md))
- Verus has no spec-level `<=` for floats (no `spec_le`)
- Uses `arbitrary()` for spec `le` and `uninterp spec fn partial_order_ensures`
- Follows `vstd::std_specs::cmp` pattern for soundness

### Set Traits

#### `Set` (`set.rs`)
Basic set operations with no View dependency or specifications.

```rust
pub trait Set<T>: Sized {
    fn empty() -> Self;
    fn contains(&self, x: &T) -> bool;
    fn insert(&mut self, x: T);
    fn remove(&mut self, x: &T);
    fn union(&self, other: &Self) -> Self;
    fn intersect(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
```

#### `SetWithView` (`set_with_view.rs`)
Set trait with `View` and full specifications using `self@` syntax.

```rust
pub trait SetWithView<T: View>: Sized + View<V = vstd::set::Set<<T as View>::V>> {
    fn empty() -> (result: Self)
        ensures result@ == Set::<<T as View>::V>::empty();
    
    fn contains(&self, x: &T) -> (result: bool)
        ensures result == self@.contains(x@);
    
    fn insert(&mut self, x: T)
        ensures self@ == old(self)@.insert(x@);
    
    // ... etc
}
```

**Design**: Two-tier approach avoids `vstd::string::View` name resolution bug in trait definitions.

#### `HashSetWithView` (`hash_set_with_view.rs`)
Extended wrapper around `std::collections::HashSet` that exposes `clone()` and `iter()` methods (missing from `vstd::hash_set::HashSetWithView`).

**Verification**: ✅ 1 verified, 0 errors

## Testing

All modules have runtime tests in `tests/vstdplus/`:
- `test_total_order.rs`: 4 tests passing
- `test_partial_order.rs`: 11 tests passing (including NaN and infinity edge cases)

## Attic

The `attic/vstdplus/` directory contains:
- Alternative float implementations that don't compile (Verus limitation: no `spec_le` for floats)
- Comparison documentation explaining why uninterpreted specs are necessary

## Design Principles

1. **Follow vstd patterns**: Match vstd's style and conventions
2. **Minimal comments**: Avoid "jejune" comments that just restate the signature
3. **Sound verification**: Use uninterpreted specs where necessary (e.g., floats)
4. **Pragmatic**: Provide working implementations, document limitations

## Verification Summary

```
✅ set.rs:                0 verified, 0 errors
✅ set_with_view.rs:       0 verified, 0 errors  
✅ hash_set_with_view.rs:  1 verified, 0 errors
✅ total_order.rs:        60 verified, 0 errors
✅ partial_order.rs:      54 verified, 0 errors

Total: 115 verified, 0 errors
```

## Future Work

- Proof-time testing infrastructure (currently using runtime tests only)
- Generic blanket implementations (blocked by Verus limitations)
- Additional ordering traits (e.g., `StrictTotalOrder`, `PreOrder`)

