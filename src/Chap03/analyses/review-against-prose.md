# Chap03 Review Against Prose

## Phase 1: Inventory

Source files: `InsertionSortStEph.rs`

### InsertionSortStEph.rs

| # | Chap | File | Function | Mode | Trait | V!/--V! | SpecStr | Holes |
|---|------|------|----------|------|-------|---------|---------|-------|
| 1 | 03 | InsertionSortStEph.rs | `sorted_prefix` | spec | ML | V! | open | 0 |
| 2 | 03 | InsertionSortStEph.rs | `cross_sorted` | spec | ML | V! | open | 0 |
| 3 | 03 | InsertionSortStEph.rs | `is_sorted` | spec | ML | V! | open | 0 |
| 4 | 03 | InsertionSortStEph.rs | `insertion_sort` | exec | ML | V! | strong | 0 |

All functions are inside verus!. Zero holes. The exec function has strong ensures (length preservation, multiset equality, sorted output).

## Phase 2: Prose Inventory

Source: `prompts/Chap03.txt` (APAS Example 3.1)

| # | Kind | Name | Description |
|---|------|------|-------------|
| 1 | Example | 3.1 Insertion Sort | Recursive insSort using insert helper |
| 2 | Algorithm | insSort | `insSort f s = if |s|=0 then <> else insert f s[0] (insSort f (s[1,...,n-1]))` |
| 3 | Function | insert | Inserts element into sorted sequence at correct position |

The prose defines a recursive formulation using a comparison function `f` and a helper `insert`. No explicit cost annotation is given in the provided prose excerpt, but insertion sort is universally known as Theta(n^2) work/span sequential.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

| # | Chap | File | Function | APAS Cost | Agent Cost | Agreement |
|---|------|------|----------|-----------|------------|-----------|
| 1 | 03 | InsertionSortStEph.rs | `insertion_sort` | (no cost stated in excerpt) | W=Theta(n^2), S=Theta(n^2) | correct |

Cost annotation was already present in the source file. Sequential insertion sort is Theta(n^2) worst-case work and span (span = work for sequential algorithms). The annotation is accurate.

### Phase 3b: Implementation Deviations

1. **Recursive vs iterative**: The prose defines insertion sort recursively (`insSort f s = if |s|=0 then <> else insert f s[0] (insSort f (s[1,...,n-1]))`). The implementation uses an iterative in-place variant with two nested loops (outer loop `up`, inner loop `down` with swaps). This is a standard transformation from functional to imperative style that preserves the algorithm's semantics and cost class. The inner loop performs the `insert` operation by bubbling an element down to its sorted position.

2. **Comparison function vs trait**: The prose uses a comparison function `f` as a parameter. The implementation uses the `TotalOrder` trait bound, which provides `cmp()`. This is the standard APAS-VERUS pattern for generic ordered types.

3. **In-place mutation**: The prose builds a new sorted sequence. The implementation sorts in-place via `&mut [T]`. This is a common imperative optimization that preserves the algorithm while using O(1) extra space.

4. **Copy trait bound**: The implementation requires `T: Copy` for the swap operation (`let tmp = a[down]; a[down] = a[down-1]; a[down-1] = tmp`). The prose does not mention copy semantics. This is a Verus/Rust requirement for in-place element manipulation.

### Phase 3c: Ensures vs Prose Postconditions

The prose specifies that `insSort f s` produces a sorted permutation of `s`. The implementation's ensures:

```
ensures
    sorted.len() == old(a).len(),           // length preservation
    sorted@.to_multiset() == old(a)@.to_multiset(),  // permutation (multiset equality)
    is_sorted(sorted),                      // output is sorted
```

These three postconditions fully capture the prose specification:
- Length preservation: implicit in the prose (same sequence, reordered).
- Multiset equality: proves the output is a permutation of the input.
- `is_sorted`: proves the output is sorted by `T::le`.

**Spec strength: Strong.** The ensures fully match the prose intent.

**Note**: The `requires true` is flagged by veracity as a `requires_true` warning. This is a vacuous precondition -- insertion sort has no precondition, so `requires true` is technically correct but unnecessary (it could be omitted entirely).

## Phase 4: Parallelism Review

InsertionSortStEph is an StEph (sequential ephemeral) module. Parallelism review is not applicable. The prose defines insertion sort as a sequential algorithm.

There is no MtEph or MtPer variant of insertion sort, which is expected: insertion sort does not have a natural parallel formulation in the APAS textbook.

## Phase 5: Runtime Test Review

### TestInsertionSortStEph.rs (7 tests)

| # | Test | Description | Covered |
|---|------|-------------|---------|
| 1 | `insertion_sort_handles_empty` | Empty input | `insertion_sort` |
| 2 | `insertion_sort_single_element` | Single element | `insertion_sort` |
| 3 | `insertion_sort_already_sorted` | Pre-sorted input | `insertion_sort` |
| 4 | `insertion_sort_reverse_order` | Reverse-sorted (worst case) | `insertion_sort` |
| 5 | `insertion_sort_with_duplicates` | Duplicate elements | `insertion_sort` |
| 6 | `insertion_sort_random_slice` | Random order | `insertion_sort` |
| 7 | `insertion_sort_large_input_stress_test` | 10,000 elements reverse-sorted | `insertion_sort` |

### Coverage Summary

| # | Chap | File | Function | RTT Coverage |
|---|------|------|----------|--------------|
| 1 | 03 | InsertionSortStEph.rs | `insertion_sort` | covered (7 tests) |

Excellent test coverage. Edge cases (empty, single, sorted, reverse, duplicates), random input, and stress test are all present. The stress test verifies correctness on a large worst-case input.

## Phase 6: PTT Review

No PTTs exist for Chap03. No iterators exist. The single exec function has straightforward requires/ensures. PTTs are not needed.

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|------------|--------|
| 1 | `insSort f s` (Example 3.1) | Implemented as `insertion_sort` |
| 2 | `insert f x s` helper | Implemented inline as the inner `down` loop |

No prose gaps. The `insert` function from the prose is not a separate function in the implementation -- it is the inner loop body. This is a standard transformation.

### Code with no prose counterpart

| # | Function | Notes |
|---|----------|-------|
| 1 | `sorted_prefix` | Spec helper for loop invariant. No prose counterpart needed. |
| 2 | `cross_sorted` | Spec helper for loop invariant. No prose counterpart needed. |
| 3 | `is_sorted` | Spec for the "sorted" postcondition. Matches prose intent. |

All three spec functions are proof infrastructure required by the iterative implementation's loop invariants. They have no direct prose counterpart because the prose uses a recursive formulation that does not need loop invariants.

## Phase 8: TOC Review

### InsertionSortStEph.rs

Sections present:
- 1. module (`pub mod InsertionSortStEph`)
- 2. imports (`use vstd::multiset::*`, `use vstd::prelude::*`, `use vstd::relations::*`, `use crate::vstdplus::total_order::total_order::*`, `use core::cmp::Ordering`)
- 3. broadcast use (group_to_multiset_ensures, group_seq_axioms, group_feq_axioms, group_seq_properties)
- 6. spec fns (`sorted_prefix`, `cross_sorted`, `is_sorted`)
- 9. impls (`insertion_sort` -- actually a free fn, not an impl)

Everything is inside verus!. Ordering is correct: imports, broadcast use, spec fns, exec fn. No TOC comment block, but the file is short (115 lines).

**Observation**: No trait-impl pattern. The module has a single exec function as a free function. For a single-function utility module like insertion sort, this is acceptable.

**Observation**: Import order -- `use vstd::multiset::*` comes before `use vstd::prelude::*`. Per the use statement order standard, `vstd::prelude::*` should come first. This is a minor style issue.

### In/Out Placement

| # | Chap | File | Item | Expected | Actual | Status |
|---|------|------|------|----------|--------|--------|
| 1 | 03 | InsertionSortStEph.rs | Everything | in | in | correct |

No placement errors.

## Summary

| Metric | Value |
|--------|-------|
| Source files | 1 |
| Exec functions | 1 |
| Spec functions | 3 |
| Proof functions | 0 |
| Proof holes | 0 |
| Warnings | 1 (`requires_true`) |
| RTT tests | 7 |
| PTT tests | 0 (not needed) |
| Uncovered exec fns | 0 |
| Spec weaknesses | 0 |
| Prose gaps | 0 |
| TOC issues | 0 |
| Style issues | 1 (import order) |

Chap03 is clean. The single exec function is fully verified with strong specs (length, permutation, sorted). The implementation is an in-place iterative variant of the recursive prose algorithm, which is a standard transformation. The proof is complete with no holes.

### Minor Findings

1. **`requires true`**: Vacuous precondition on `insertion_sort`. Could be removed (insertion sort has no precondition), but leaving it is harmless.
2. **Import order**: `vstd::multiset::*` before `vstd::prelude::*`. Minor style issue.

---
Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
