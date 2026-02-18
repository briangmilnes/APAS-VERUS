<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap03 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-17
**Prose file:** `prompts/Chap03.txt`
**Source files:** `InsertionSortStEph.rs`

## Phase 1: Inventory

From `analyses/veracity-review-module-fn-impls.json` (Chap03 scope):

| # | File | Lines | Functions | Proof holes |
|---|------|-------|-----------|-------------|
| 1 | InsertionSortStEph.rs | 110 | 1 exec (insertion_sort), spec fns (sorted_prefix, cross_sorted, is_sorted) | 0 — clean |

| # | Function | Trait | ML | V! | SpecStr |
|---|----------|:-----:|:--:|:--:|---------|
| 1 | insertion_sort |  | Y | Y | strong |

## Phase 2: Prose Inventory

From `prompts/Chap03.txt`:

| # | Item | Type |
|---|------|------|
| 1 | Example 3.1: Insertion sort pseudocode — recursive, functional style | Algorithm |
| 2 | insSort f s — comparison function f, input sequence s | Pseudocode |
| 3 | insert f x s — insert x into sorted sequence s | Sub-algorithm |

The prose gives a recursive functional insertion sort. No cost bounds are
explicitly stated in the provided excerpt, but insertion sort is classically
Work Theta(n^2), Span Theta(n^2) (sequential).

## Phase 3: Algorithmic Analysis (Cost Annotations)

| # | Function | APAS | Claude-Opus-4.6 | Status |
|---|----------|------|-----------------|--------|
| 1 | insertion_sort | Work Θ(n²), Span Θ(n²) | Work Θ(n²), Span Θ(n²) — agrees | ✅ Present |

All exec functions have cost annotations. No disagreements.

## Phase 4: Parallelism Review

N/A — no Mt modules in Chap03.

## Phase 5: RTT Review

| # | Test file | Tests | Notes |
|---|-----------|-------|-------|
| 1 | tests/Chap03/TestInsertionSortStEph.rs | 7 | empty, single, sorted, reverse, duplicates, random, large (10k) |

Chap03 is included when `full_verify` is enabled (default for validate). Good coverage.

## Phase 6: PTT Review

No PTTs for Chap03. No iterators or ForLoopGhostIterator; no complicated callability.
**Verdict:** No PTTs needed.

## Phase 7: Gap Analysis

**Prose items with no implementation:**
- The `insert f x s` sub-function is not a separate function. The code
  inlines insertion into the inner loop of `insertion_sort`. This is the
  standard approach for the iterative variant.

**Code with no prose counterpart:**
- `sorted_prefix` spec function — proof helper.
- `cross_sorted` spec function — proof helper for bridging the inner loop
  invariant to the outer loop invariant.
- `is_sorted` — uses `TotalOrder::le`.
- `TotalOrder` trait and its laws — in `src/vstdplus/total_order.rs`.

## Phase 8: TOC Review

`veracity-review-verus-style -r -n` reports: definition order correct, 0 files would be reordered.
File is small (110 lines) with minimal structure (broadcast use, spec fns, 1 exec fn). No TOC
comment present; style tool passes. No action needed.

## Implementation Fidelity

The prose gives a **recursive functional** insertion sort:
```
insSort f s = if |s| = 0 then <> else insert f s[0] (insSort f s[1..n-1])
```

The code implements an **iterative in-place** variant on mutable slices with
a `TotalOrder` trait instead of a comparison function parameter. This is a
standard deviation for imperative verification — the recursive functional
version would require allocating new sequences on each call.

Key differences:
- Prose: recursive, allocates new sequences, comparison function parameter.
- Code: iterative, in-place mutation via swap, `TotalOrder` trait bound.
- Same asymptotic cost. The in-place variant is more practical for Rust.

## Spec Fidelity

| # | Function | Spec strength | Notes |
|---|----------|--------------|-------|
| 1 | insertion_sort | Strong | ensures sorted.len() == old(a).len(), multiset preservation, is_sorted(sorted) |

The spec captures both correctness properties:
1. **Permutation**: `sorted@.to_multiset() == old(a)@.to_multiset()` — the
   output is a permutation of the input.
2. **Sortedness**: `is_sorted(sorted)` — the output is sorted.

The sortedness proof uses:
- `cross_sorted` spec function for the inner loop invariant.
- Explicit transitivity proofs via `TotalOrder::transitive`.
- Lexicographic decreases for the inner loop (`down, if swapped { 1 } else { 0 }`).

The prose says "sorted by f" (comparison function). The code uses
`TotalOrder` trait which connects `cmp` to `spec fn le` with reflexivity,
antisymmetry, transitivity, and totality. This is strictly stronger than
an arbitrary comparison function — it guarantees a total order, which the
prose also assumes but doesn't formalize.

## Proof Holes

```
✓ InsertionSortStEph.rs

═══════════════════════════════════════════════════════════════
SUMMARY
═══════════════════════════════════════════════════════════════

Modules:
   1 clean (no holes)
   0 holed (contains holes)
   1 total

Holes Found: 0 total

🎉 No proof holes or warnings found! All proofs are complete.
```

## Summary

Chap03 is a single-file chapter implementing insertion sort. The code is
fully verified with strong specs (permutation + sortedness). The main
deviation from prose is iterative in-place vs recursive functional, which
is standard for imperative verification. No cost disagreements. No proof
holes. All 8 phases verified.
