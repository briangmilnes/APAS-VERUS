<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 4 Round 58 Report

## Summary

Two tasks completed: added `fn_missing_requires` to 2 Chap65 functions (Task A, easy) and
closed 2 clone bridge holes in Chap38 (Task B, medium). All 4 changes verified clean.
The pre-existing Chap43 flaky error (4484 verified, 1 error) was present before and after
the changes — confirmed by running validate on the stashed (unchanged) code.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 38 | BSTParaStEph.rs | 1 | 0 | −1 |
| 2 | 38 | BSTParaMtEph.rs | 1 | 0 | −1 |
| 3 | 65 | KruskalStEph.rs | 0 (warn×1) | 0 | 0 |
| 4 | 65 | PrimStEph.rs | 0 (warn×1) | 0 | 0 |

**Total holes closed: 2** (both `assume()` algorithmic holes in Chap38).
**Warnings resolved: 2** (`fn_missing_requires` in Chap65 — not counted as holes).

## Task A: fn_missing_requires in Chap65

### A1: `sort_edges_by_weight` in `src/Chap65/KruskalStEph.rs`

Added:
```rust
requires forall|i: int| 0 <= i < edges@.len() ==> #[trigger] edges@[i].2.spec_is_finite(),
```

**Rationale**: The sort uses `float_cmp` and `WrappedF64::transitive`, both of which require
finite (non-NaN, non-Inf) float values for IEEE 754 ordering to be well-defined. This is the
real precondition: all edge weights must be finite floats.

### A2: `pq_entry_new` in `src/Chap65/PrimStEph.rs`

Added:
```rust
requires priority.spec_is_finite(),
```

**Rationale**: Priority queue entries in Prim's algorithm are meaningfully compared via
`Ord` on `PQEntry<V>` (which delegates to `WrappedF64::cmp`). A non-finite priority
(NaN, Inf) would break the priority ordering invariants. The real precondition documents
that only finite priorities are admitted. Both call sites satisfy this: `zero_dist()` ensures
`d.spec_is_finite()`, and graph edge weights in a well-formed graph are finite.

## Task B: Clone Bridge Holes in Chap38

The two holes were `assume(obeys_feq_clone::<T>())` in:
- `BSTParaStEph.rs:479` inside `fn expose` impl
- `BSTParaMtEph.rs:712` inside `fn expose_internal`

Both were needed so `clone_elem` (which requires `obeys_feq_clone::<T>()`) could be called.

### Fix

Replaced `assume` with a broadcast trigger assertion:

```rust
// BEFORE (both files):
proof { assume(obeys_feq_clone::<T>()); } // assume_eq_clone_workaround

// AFTER (both files):
proof { assert(obeys_feq_full_trigger::<T>()); }
```

**How it works**: Both modules have `broadcast use crate::vstdplus::feq::feq::group_feq_axioms`
which includes `axiom_obeys_feq_full`. This broadcast axiom says: for any
`T: Eq + View + Clone + Sized`, `obeys_feq_full::<T>()` holds (triggered by
`obeys_feq_full_trigger::<T>()`). Since `obeys_feq_full_trigger::<T>() == true` always,
asserting it fires the trigger. Z3 then derives `obeys_feq_full::<T>()`, which unfolds to
include `obeys_feq_clone::<T>()` as a conjunct. `clone_elem` can then be called.

This pattern matches the one used in `PrimStEph.rs` line 113:
```rust
proof { assert(obeys_feq_full_trigger::<PQEntry<V>>()); }
```

**No cascade required**: Because the assertion is inside the function body, no requires
changes are needed on `expose`/`expose_internal` or their callers.

Added imports to both files:
```rust
#[cfg(verus_keep_ghost)]
use crate::vstdplus::feq::feq::obeys_feq_full_trigger;
```

## Verification Results

```
verification results:: 4484 verified, 1 errors
```

The 1 error is in `Chap43/OrderedSetStPer.rs:910` — pre-existing flaky SMT behavior.
Confirmed by running validate on the unchanged (stashed) codebase, which also produced
`4484 verified, 1 errors`. Not caused by my changes.

Chap38 holes after: **0**. Chap65 warnings after: **0**.
