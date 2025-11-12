<style>
body, .markdown-body {
    max-width: 1600px !important;
    margin: 0 auto;
}
</style>

# Verus Standard and APAS-VERUS Iterator Traits

This document provides a comprehensive reference of iterator-related traits and implementations in both Verus standard library and the APAS-VERUS project.

---

# Part 1: Verus Standard Library

## Overview

Verus provides built-in support for verified iteration through the `ForLoopGhostIterator` trait system, primarily designed for `for` loops. The standard library includes verified iterators for all major Rust collection types.

---

## Traits

| Trait Name | File | Line | Purpose |
|------------|------|------|---------|
| `ExIterator` | `vstd/std_specs/core.rs` | 91 | External trait specification for Rust's `core::iter::Iterator`. Provides `next()` specification. |
| `ExIntoIterator` | `vstd/std_specs/core.rs` | 100 | External trait specification for Rust's `core::iter::IntoIterator`. |
| `StepSpec` | `vstd/std_specs/range.rs` | 12 | Trait for steppable types: `spec_is_lt`, `spec_forward_checked`, etc. Required by `Range<A>`. |
| `ForLoopGhostIterator` | `vstd/pervasive.rs` | 42 | Ghost state trait for `for` loops. Defines `exec_invariant`, `ghost_invariant`, `ghost_ensures`, `ghost_decrease`, `ghost_peek_next`, `ghost_advance`. |
| `ForLoopGhostIteratorNew` | `vstd/pervasive.rs` | 84 | Companion trait to create ghost iterator from exec iterator. Defines `ghost_iter()` method. |

**Loop Types Supported:**

| Loop Type | Range (`0..10`) | General Collections |
|-----------|-----------------|---------------------|
| `while` | ✓ | ✓ |
| `loop` | ✓ | ✓ |
| `for` (Range) | ✓ | N/A |
| `for` (General) | N/A | ✓ |

---

## Range Iterator Example

| Type | Name | File | Line | Description |
|------|------|------|------|-------------|
| struct | `ExRange<Idx>` | `vstd/std_specs/range.rs` | 10 | External type specification for `core::ops::Range<Idx>`. |
| struct | `RangeGhostIterator<A>` | `vstd/std_specs/range.rs` | 50 | Ghost state: `start: A`, `cur: A`, `end: A` |

**Implementations:**
- `impl ForLoopGhostIteratorNew for Range<A>` (line 56)
- `impl ForLoopGhostIterator for RangeGhostIterator<A>` (line 64)
- `impl View for RangeGhostIterator<A>` (line 109) - View as `Seq<A>`

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

---

# Part 2: APAS-VERUS Alternative Approach

## Overview

APAS-VERUS has developed custom traits (`GhostIteratorTrait` and `ExecIteratorTrait`) specifically for manual `while` and `loop` patterns where explicit control over the iterator is needed. This approach provides more flexibility for complex verification scenarios but requires manual invariant management.

---

## Custom Traits

| Trait Name | File | Line | Purpose |
|------------|------|------|---------|
| `GhostIteratorTrait` | `src/experiments/verus_iterator.rs` | 5 | Custom ghost state for manual `while`/`loop` iteration. Defines same methods as `ForLoopGhostIterator` but designed for explicit `next()` calls. |
| `ExecIteratorTrait` | `src/experiments/verus_iterator.rs` | 18 | Custom executable iterator with `iter()` and `next()` methods. Provides ghost state connections via `requires`/`ensures`. |

**Loop Types Supported:**

| Loop Type | Vec Custom (`Vec<usize>`) |
|-----------|---------------------------|
| `while` | ✅ (287 functions verified) |
| `loop` | ✅ (287 functions verified) |
| `for` (Range) | N/A |
| `for` (General) | ⚠️ Buggy (incomplete) |

---

## Vec Iterator Implementation

| Type | Name | File | Line | Description |
|------|------|------|------|-------------|
| struct | `VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 6 | Ghost state: `cur: int`, `end: int`, `data: Seq<usize>` |
| struct | `VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 59 | Exec state: `data: Vec<usize>`, `cur: usize` |
| struct | `VecCollection` | `src/experiments/verus_vec_iterator.rs` | 76 | Wrapper providing `iter()` method |

**Implementations:**
- `impl GhostIteratorTrait for VecGhostIter` (line 12) - Custom ghost trait ✅
- `impl ExecIteratorTrait for VecCollection` (line 80) - Custom exec trait ✅
- `impl View for VecExecIter` (line 64) - Connects exec to ghost state ✅
- `impl Iterator for VecExecIter` (line 168) - Standard Rust Iterator ✅
- `impl ForLoopGhostIteratorNew for VecExecIter` (line 112) - ⚠️ Buggy
- `impl ForLoopGhostIterator for VecGhostIter` (line 120) - ⚠️ Buggy

---



