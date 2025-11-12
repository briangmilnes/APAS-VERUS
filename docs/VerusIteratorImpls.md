<style>
body, .markdown-body {
    max-width: 1600px !important;
    margin: 0 auto;
}
</style>

# Verus Iterator Trait Implementations

This document lists all implementations of iterator-related traits in Verus standard library and APAS-VERUS.

---

## ExIterator and ExIntoIterator

**No implementations exist** for these traits. They are `#[verifier::external_trait_specification]` attributes that provide Verus specifications for Rust's standard `core::iter::Iterator` and `core::iter::IntoIterator` traits. They describe how Verus understands these existing Rust traits but are not implemented directly.

---

## ForLoopGhostIteratorNew Implementations (Verus Standard Library)

| Type | File | Line |
|------|------|------|
| `Range<A>` | `vstd/std_specs/range.rs` | 56 |
| `Iter<'a, T>` (slice) | `vstd/std_specs/slice.rs` | 90 |
| `IntoIter<T, A>` (Vec) | `vstd/std_specs/vec.rs` | 339 |
| `Iter<'a, T>` (VecDeque) | `vstd/std_specs/vecdeque.rs` | 291 |
| `Keys<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | 313 |
| `Values<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | 422 |
| `hash_map::Iter<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 538 |
| `hash_set::Iter<'a, Key>` (HashSet) | `vstd/std_specs/hash.rs` | 1114 |
| `Chars<'a>` (String) | `vstd/string.rs` | 390 |

---

## ForLoopGhostIterator Implementations (Verus Standard Library)

| Type | File | Line |
|------|------|------|
| `RangeGhostIterator<A>` | `vstd/std_specs/range.rs` | 64-66 |
| `IterGhostIterator<'a, T>` (slice) | `vstd/std_specs/slice.rs` | 98 |
| `IntoIterGhostIterator<T, A>` (Vec) | `vstd/std_specs/vec.rs` | 347 |
| `IterGhostIterator<'a, T>` (VecDeque) | `vstd/std_specs/vecdeque.rs` | 299 |
| `KeysGhostIterator<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | 321 |
| `ValuesGhostIterator<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | 430 |
| `MapIterGhostIterator<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 550 |
| `SetIterGhostIterator<'a, Key>` (HashSet) | `vstd/std_specs/hash.rs` | 1122 |
| `CharsGhostIterator<'a>` (String) | `vstd/string.rs` | 399 |

---

## APAS-VERUS Custom Trait Implementations

### GhostIteratorTrait (Custom)

| Type | File | Line |
|------|------|------|
| `VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 12 |

### ExecIteratorTrait (Custom)

| Type | File | Line |
|------|------|------|
| `VecCollection` | `src/experiments/verus_vec_iterator.rs` | 80 |

### ForLoopGhostIteratorNew (APAS-VERUS)

| Type | File | Line |
|------|------|------|
| `VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 112 |

### ForLoopGhostIterator (APAS-VERUS)

| Type | File | Line |
|------|------|------|
| `VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 120 |

---

## Summary

**Verus Standard Library:**
- 9 implementations of `ForLoopGhostIteratorNew` (exec → ghost conversion)
- 9 implementations of `ForLoopGhostIterator` (ghost iterator behavior)
- Covers: `Range`, `slice::Iter`, `Vec::IntoIter`, `VecDeque::Iter`, `HashMap` (Keys, Values, Iter), `HashSet::Iter`, `String::Chars`

**APAS-VERUS:**
- 1 implementation of custom `GhostIteratorTrait`
- 1 implementation of custom `ExecIteratorTrait`
- 1 implementation of `ForLoopGhostIteratorNew` (attempted, buggy ⚠️)
- 1 implementation of `ForLoopGhostIterator` (attempted, buggy ⚠️)
- Covers: `Vec<usize>` iteration

**Key Observation:** The Verus standard library provides verified iterators for all major Rust collection types. Each collection has a pair of implementations:
1. `ForLoopGhostIteratorNew` on the executable iterator type (e.g., `Iter<'a, T>`)
2. `ForLoopGhostIterator` on the ghost iterator type (e.g., `IterGhostIterator<'a, T>`)

This pattern is consistent across all collections and is what APAS-VERUS attempted to replicate for `Vec<usize>`.

