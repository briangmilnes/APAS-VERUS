<style>
body, .markdown-body {
    max-width: 1600px !important;
    margin: 0 auto;
}
</style>

# Verus Standard Library Iterator Traits

This document provides a comprehensive reference of iterator-related traits and implementations in the Verus standard library.

## Overview

Verus provides built-in support for verified iteration through the `ForLoopGhostIterator` trait system, primarily designed for `for` loops. The standard library includes verified iterators for all major Rust collection types.

---

## Traits

| Name | File | Line | Used in Loop Types | Purpose |
|------|------|------|---------------------------------|---------|
| `ExIterator` | `vstd/std_specs/core.rs` | 91 | while ☐, loop ☐, for (range) ☐, for (general) ☐ | External trait specification for Rust's `core::iter::Iterator`. Provides `next()` specification. |
| `ExIntoIterator` | `vstd/std_specs/core.rs` | 100 | while ☐, loop ☐, for (range) ☐, for (general) ☐ | External trait specification for Rust's `core::iter::IntoIterator`. |
| `StepSpec` | `vstd/std_specs/range.rs` | 12 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | Trait for steppable types: `spec_is_lt`, `spec_forward_checked`, etc. Required by `Range<A>`. |
| `ForLoopGhostIterator` | `vstd/pervasive.rs` | 42 | while ☐, loop ☐, for (range) ✅, for (general) ✅ | Ghost state trait for `for` loops. Defines `exec_invariant`, `ghost_invariant`, `ghost_ensures`, `ghost_decrease`, `ghost_peek_next`, `ghost_advance`. |
| `ForLoopGhostIteratorNew` | `vstd/pervasive.rs` | 84 | while ☐, loop ☐, for (range) ✅, for (general) ✅ | Companion trait to create ghost iterator from exec iterator. Defines `ghost_iter()` method. |

## Structs

| Name | File | Line | Used in Loop Types | Purpose |
|------|------|------|---------------------------------|---------|
| `ExRange<Idx>` | `vstd/std_specs/range.rs` | 10 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | External type specification for `core::ops::Range<Idx>`. |
| `RangeGhostIterator<A>` | `vstd/std_specs/range.rs` | 50 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | Ghost state: `start: A`, `cur: A`, `end: A`. |
| `IterGhostIterator<'a, T>` (slice) | `vstd/std_specs/slice.rs` | ~80 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for slice iteration. |
| `IntoIterGhostIterator<T, A>` (Vec) | `vstd/std_specs/vec.rs` | ~330 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for Vec::IntoIter. |
| `IterGhostIterator<'a, T>` (VecDeque) | `vstd/std_specs/vecdeque.rs` | ~280 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for VecDeque::Iter. |
| `KeysGhostIterator<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | ~300 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for HashMap::Keys. |
| `ValuesGhostIterator<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | ~410 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for HashMap::Values. |
| `MapIterGhostIterator<'a, Key, Value>` (HashMap) | `vstd/std_specs/hash.rs` | ~530 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for HashMap::Iter. |
| `SetIterGhostIterator<'a, Key>` (HashSet) | `vstd/std_specs/hash.rs` | ~1100 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for HashSet::Iter. |
| `CharsGhostIterator<'a>` (String) | `vstd/string.rs` | ~380 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | Ghost state for String::Chars. |

## Implementations

| Name | File | Line | Used in Loop Types | Associated Types | Purpose |
|------|------|------|---------------------------------|------------------|---------|
| `ForLoopGhostIteratorNew for Range<A>` | `vstd/std_specs/range.rs` | 56 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | `GhostIter = RangeGhostIterator<A>` | Creates `RangeGhostIterator` from `Range<A>`. |
| `ForLoopGhostIterator for RangeGhostIterator<A>` | `vstd/std_specs/range.rs` | 64 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | `ExecIter = Range<A>`, `Item = A`, `Decrease = int` | Implements ghost iterator behavior for Range. |
| `View for RangeGhostIterator<A>` | `vstd/std_specs/range.rs` | 109 | while ☐, loop ☐, for (range) ✅, for (general) ☐ | `V = Seq<A>` | View as `Seq<A>`: generates `seq![start, start+1, ..., cur-1]`. |
| `ForLoopGhostIteratorNew for Iter<'a, T>` (slice) | `vstd/std_specs/slice.rs` | 90 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = IterGhostIterator<'a, T>` | Creates ghost iterator for slice::Iter. |
| `ForLoopGhostIterator for IterGhostIterator<'a, T>` (slice) | `vstd/std_specs/slice.rs` | 98 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = Iter<'a, T>`, `Item = T`, `Decrease = int` | Implements ghost iterator for slice. |
| `ForLoopGhostIteratorNew for IntoIter<T, A>` (Vec) | `vstd/std_specs/vec.rs` | 339 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = IntoIterGhostIterator<T, A>` | Creates ghost iterator for Vec::IntoIter. |
| `ForLoopGhostIterator for IntoIterGhostIterator<T, A>` (Vec) | `vstd/std_specs/vec.rs` | 347 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = IntoIter<T, A>`, `Item = T`, `Decrease = int` | Implements ghost iterator for Vec. |
| `ForLoopGhostIteratorNew for Iter<'a, T>` (VecDeque) | `vstd/std_specs/vecdeque.rs` | 291 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = IterGhostIterator<'a, T>` | Creates ghost iterator for VecDeque::Iter. |
| `ForLoopGhostIterator for IterGhostIterator<'a, T>` (VecDeque) | `vstd/std_specs/vecdeque.rs` | 299 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = Iter<'a, T>`, `Item = T`, `Decrease = int` | Implements ghost iterator for VecDeque. |
| `ForLoopGhostIteratorNew for Keys<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 313 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = KeysGhostIterator<'a, Key, Value>` | Creates ghost iterator for HashMap::Keys. |
| `ForLoopGhostIterator for KeysGhostIterator<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 321 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = Keys<'a, Key, Value>`, `Item = Key`, `Decrease = int` | Implements ghost iterator for HashMap::Keys. |
| `ForLoopGhostIteratorNew for Values<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 422 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = ValuesGhostIterator<'a, Key, Value>` | Creates ghost iterator for HashMap::Values. |
| `ForLoopGhostIterator for ValuesGhostIterator<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 430 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = Values<'a, Key, Value>`, `Item = Value`, `Decrease = int` | Implements ghost iterator for HashMap::Values. |
| `ForLoopGhostIteratorNew for hash_map::Iter<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 538 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = MapIterGhostIterator<'a, Key, Value>` | Creates ghost iterator for HashMap::Iter. |
| `ForLoopGhostIterator for MapIterGhostIterator<'a, Key, Value>` | `vstd/std_specs/hash.rs` | 550 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = hash_map::Iter<'a, Key, Value>`, `Item = (Key, Value)`, `Decrease = int` | Implements ghost iterator for HashMap::Iter. |
| `ForLoopGhostIteratorNew for hash_set::Iter<'a, Key>` | `vstd/std_specs/hash.rs` | 1114 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = SetIterGhostIterator<'a, Key>` | Creates ghost iterator for HashSet::Iter. |
| `ForLoopGhostIterator for SetIterGhostIterator<'a, Key>` | `vstd/std_specs/hash.rs` | 1122 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = hash_set::Iter<'a, Key>`, `Item = Key`, `Decrease = int` | Implements ghost iterator for HashSet. |
| `ForLoopGhostIteratorNew for Chars<'a>` | `vstd/string.rs` | 390 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `GhostIter = CharsGhostIterator<'a>` | Creates ghost iterator for String::Chars. |
| `ForLoopGhostIterator for CharsGhostIterator<'a>` | `vstd/string.rs` | 399 | while ☐, loop ☐, for (range) ☐, for (general) ✅ | `ExecIter = Chars<'a>`, `Item = char`, `Decrease = int` | Implements ghost iterator for String::Chars. |

---

## Architecture (Verus Standard)

**Design:**
- Implements only `ForLoopGhostIterator` + `ForLoopGhostIteratorNew`
- Designed exclusively for `for` loops
- `ghost_peek_next()` returns `Some(self.cur)` - the value equals current position
- Direct mapping: iteration variable `x` equals `iter.cur` in the loop body

**Usage:**
```rust
for x in iter: 0..10
    invariant n == iter.cur * 3,
{
    assert(x == iter.cur);  // This holds!
    n += 3;
}
```

**Status:** ✅ Production-ready, fully supported by Verus for `for` loops

---

## Standard Library Coverage

Verus provides `ForLoopGhostIterator` implementations for:
- `Range<A>` - numeric ranges
- `slice::Iter<'a, T>` - slice iteration
- `Vec::IntoIter<T, A>` - consuming Vec iteration
- `VecDeque::Iter<'a, T>` - VecDeque iteration
- `HashMap::Keys`, `HashMap::Values`, `HashMap::Iter` - HashMap iteration
- `HashSet::Iter<'a, Key>` - HashSet iteration
- `String::Chars<'a>` - String character iteration

See `docs/VerusIteratorImpls.md` for complete implementation details.



