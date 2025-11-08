# vstdplus - Extensions to Verus Standard Library

This module contains traits and wrappers that extend `vstd` with additional functionality needed for APAS-VERUS.

## Modules

### `set.rs` - Pure Set Operations Trait
Basic set trait without View requirements - for generic algorithms that don't need spec-level reasoning.

### `set_with_view.rs` - Verified Set Trait
Set trait with `View<V = vstd::set::Set<T::V>>` and complete specifications. All methods include `requires`/`ensures` clauses for verification.

**Design Choice**: `empty()` requires `obeys_key_model::<T>()` as a precondition. This is Verus's standard pattern for hash-based collections. Callers must satisfy this precondition, either:
- For primitive types: use `broadcast use vstd::std_specs::hash::group_hash_axioms` which provides `obeys_key_model` axioms
- For custom types: assume or prove `obeys_key_model` at call sites
- For generic code: propagate the `requires obeys_key_model::<T>()` clause

### `hash_set_with_view_plus.rs` - Extended HashSetWithView
Wrapper around `std::collections::HashSet` that provides:
- Complete `View` specification (uninterpreted due to std internals)
- Basic operations: `new`, `len`, `contains`, `insert`, `remove`
- Set operations: `clone`, `union`, `intersection`, `difference`
- Iteration: `iter()` with spec

**Verification Holes**: 25 `#[verifier::external_body]` markers. These are **inherent limitations**:
1. `std::collections::HashSet` internals are not accessible to Verus
2. The `view()` function must be uninterpreted (cannot look inside `HashSet`)
3. All methods that manipulate the underlying `HashSet` require `external_body`
4. The `ensures` clauses are **trusted** but match Rust's actual semantics

This is the **same approach** used by `vstd::hash_set::HashSetWithView` - we extend it with additional operations.

### `total_order.rs` - Total Ordering Trait
Connects spec-level `le` function to executable `cmp` method, following the pattern from Verus PR #1569 (Chris Hawblitzel).

Uses `vstd::std_specs::cmp::obeys_cmp_spec` and `vstd::laws_cmp::group_laws_cmp` for built-in types.

Implemented for: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize

### `partial_order.rs` - Partial Ordering Trait
Similar to `TotalOrder` but for types that may not have a total order.

**Float Support**: `f32` and `f64` implementations use **uninterpreted specifications** (`arbitrary()` for `le`, `admit()` for proofs, `#[verifier::external_body]` for `compare`). This is **Verus's standard pattern** for floats because:
1. Rust floating-point operations are **non-deterministic** (RFC 3514)
2. Verus cannot provide deterministic specifications for non-deterministic operations
3. Users must axiomatize float behavior for their specific use cases

This matches `vstd::std_specs::cmp::ExPartialOrd` approach.

## Known Limitations

### 1. Clone + View Axiom (RelationStEph::mem)
**Issue**: Cannot prove that `x.clone()@ == x@` for generic types with `Clone + View`.

**Why**: `clone()` is an exec function and cannot be called in spec context.

**Solution Exists**: `vstd::pervasive::strictly_cloned<T>` uses `call_ensures(T::clone, (&a,), b)` to connect exec cloning to spec.

**Status**: Currently using `#[verifier::external_body]` with complete specs. Future work: implement the `call_ensures` pattern.

### 2. Set Length Zero Implies Empty (SetStEph::is_empty)
**Issue**: Cannot prove `self@.len() == 0 ==> self@ == Set::empty()`.

**Why**: vstd has `axiom_set_empty_len` (forward direction: empty set has length 0) but not the reverse (length 0 implies empty).

**Solution Needed**: Add axiom to vstd: `self@.len() == 0 <==> (forall |a| !self@.contains(a))`.

**Status**: Using `#[verifier::external_body]` with complete spec.

### 3. ForLoopGhostIterator for Vec<T> (SetStEph::FromVec, etc.)
**Issue**: Cannot verify `for` loops over `Vec::into_iter()` for generic `T: StT + Hash`.

**Why**: Verus's `ForLoopGhostIterator` support for `Vec<T>` appears incomplete for custom `T`.

**Status**: Using `#[verifier::external_body]` with complete specs. All these methods work correctly at runtime.

### 4. ForLoopGhostIterator for Newtype Wrappers (RelationStEph::domain, range)
**Issue**: `PairIter` (newtype wrapper for `std::collections::hash_set::Iter<Pair<T,U>>`) doesn't work with Verus's `for` loop verification, even with complete `ForLoopGhostIteratorNew` and `ForLoopGhostIterator` implementations.

**Why**: Suspected Verus compiler limitation - the `verus_builtin` mechanism doesn't recognize newtype wrappers.

**Workaround Attempted**: Implemented full ghost iterator infrastructure in `Types.rs`.

**Status**: Using `#[verifier::external_body]` with complete specs. Documented as TODO in code.

### 5. Formatter/Hasher Specs (SetStEph::Debug/Display/Hash)
**Issue**: `core::fmt::Formatter`, `core::fmt::Result`, and `std::hash::Hasher` types are not supported by Verus in verified code.

**Why**: These types involve complex trait machinery and side effects that Verus doesn't model.

**Status**: Implementations are **outside** `verus!` blocks. These are pedagogical runtime traits for display/debugging, not part of verified specifications.

## Summary of Proof Holes

| Category | Count | Status |
|----------|-------|--------|
| obeys_key_model preconditions | ~10 | **Documented** - Verus standard pattern |
| HashSetWithViewPlus external_body | 25 | **Inherent** - std internals unprovable |
| Clone+View axiom | 1 | **Solvable** - need call_ensures pattern |
| Set len==0 axiom | 1 | **Needs vstd fix** - missing reverse axiom |
| Vec ForLoopGhostIterator | 3 | **Verus limitation** - generic Vec iteration |
| Newtype ForLoopGhostIterator | 2 | **Verus limitation** - compiler issue |
| Float admits | 6 | **Inherent** - non-deterministic floats |
| Debug/Display/Hash | 3 | **Expected** - outside verification scope |

**Total**: ~51 markers, of which:
- **34 are inherent/expected** (HashSetWithViewPlus, floats, display traits)
- **10 are design choices** (obeys_key_model preconditions)
- **7 are solvable** (Clone+View axiom, len==0 axiom, loop iterators)

## Testing

All modules have runtime tests:
- `tests/vstdplus/test_total_order.rs` - TotalOrder trait tests
- `tests/vstdplus/test_partial_order.rs` - PartialOrder trait tests  

Chapter 3 and Chapter 5 tests verify that the traits work correctly for APAS algorithms.
