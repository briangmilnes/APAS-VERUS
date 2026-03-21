<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Plan: Fold `obeys_feq_full` into `spec_X_wf`

## Goal

Eliminate ~490 standalone `obeys_feq_full` lines and ~175 `obeys_feq_full_trigger` assertions
from chapter files by folding functional-equality requirements into each module's
well-formedness predicate. Since every function already `requires self.spec_X_wf()`, feq
propagates automatically.

## Current State

| Metric | Count |
|---|---|
| `obeys_feq_full` lines in chapters | 490 |
| `obeys_feq_full_trigger` in chapters | 175 |
| `obeys_view_eq` in chapters | 123 |
| Files affected | ~30 chapter files |
| Already migrated | ArraySetStEph, UnionFindStEph |

## Approach: Option A (from discussion)

For each module that uses `obeys_feq_full::<T>()` in `requires` clauses:

1. Add `obeys_feq_full::<T>()` (or K, V, Pair) to `spec_X_wf(&self)`.
2. Delete standalone `obeys_feq_full` from every `requires` clause that already has wf.
3. Delete `assert(obeys_feq_full_trigger::<T>())` from proof blocks (wf already provides feq).
4. Validate. Fix any Z3 regressions by adding `assert` hints where needed.

This is a **strengthening** of wf (more conditions), not a weakening of specs.

## Phase 0: Add `obeys_feq_fulls<T, U>` helper to `feq.rs`

```rust
pub open spec fn obeys_feq_fulls<
    T: Eq + View + Clone + Sized,
    U: Eq + View + Clone + Sized,
>() -> bool {
    obeys_feq_full::<T>() && obeys_feq_full::<U>()
}
```

No `Pair<K,V>` variant in vstdplus — that would create a dependency on `Types.rs`.
Each table module defines its own local helper if needed:

```rust
open spec fn feq_kv() -> bool {
    obeys_feq_full::<K>() && obeys_feq_full::<V>() && obeys_feq_full::<Pair<K, V>>()
}
```

## Phase 1: Single-type modules (T only)

Add `&& obeys_feq_full::<T>()` to wf, delete standalone feq lines.

| # | Chap | File | feq lines | Est. savings |
|---|:----:|---|:---:|:---:|
| 1 | 37 | AVLTreeSeq.rs | 16 | ~14 |
| 2 | 37 | AVLTreeSeqStEph.rs | 18 | ~16 |
| 3 | 37 | AVLTreeSeqStPer.rs | 8 | ~7 |
| 4 | 37 | AVLTreeSeqMtPer.rs | 5 | ~4 |
| 5 | 41 | AVLTreeSetStEph.rs | 28 | ~26 |
| 6 | 41 | AVLTreeSetStPer.rs | 19 | ~17 |
| 7 | 17 | MathSeq.rs | 1 | ~0 |
| 8 | 45 | BalancedTreePQ.rs | 3 | ~2 |
| 9 | 66 | BoruvkaStEph.rs | 6 | ~5 |
| 10 | 05 | SetStEph.rs | 1 | ~0 |
| 11 | 05 | SetMtEph.rs | 1 | ~0 |
| | | **Phase 1 total** | **106** | **~91** |

## Phase 2: Two-type modules (K, V, Pair<K,V>)

Add `&& obeys_feq_full::<K>() && obeys_feq_full::<V>() && obeys_feq_full::<Pair<K,V>>()`
to wf. Delete standalone feq + trigger lines.

| # | Chap | File | feq lines | Est. savings |
|---|:----:|---|:---:|:---:|
| 12 | 42 | TableStEph.rs | 19 | ~16 |
| 13 | 42 | TableStPer.rs | 26 | ~22 |
| 14 | 42 | TableMtEph.rs | 29 | ~26 |
| 15 | 43 | OrderedTableStEph.rs | 76 | ~70 |
| 16 | 43 | OrderedTableStPer.rs | 60 | ~55 |
| 17 | 43 | OrderedTableMtEph.rs | 18 | ~15 |
| 18 | 43 | OrderedTableMtPer.rs | 10 | ~8 |
| 19 | 43 | AugOrderedTableStEph.rs | 12 | ~10 |
| 20 | 43 | AugOrderedTableStPer.rs | 12 | ~10 |
| 21 | 43 | AugOrderedTableMtEph.rs | 7 | ~5 |
| 22 | 43 | OrderedSetStEph.rs | 20 | ~18 |
| 23 | 43 | OrderedSetStPer.rs | 17 | ~15 |
| 24 | 43 | OrderedSetMtEph.rs | 3 | ~2 |
| 25 | 65 | KruskalStEph.rs | 2 | ~1 |
| 26 | 65 | PrimStEph.rs | 2 | ~1 |
| | | **Phase 2 total** | **313** | **~274** |

## Phase 3: Clean up `obeys_view_eq`

`obeys_feq_full` implies `obeys_view_eq` (eq + view properties). After Phases 1-2,
`obeys_view_eq::<K>()` lines in requires should be provable from wf alone.

- Try removing `obeys_view_eq` lines (~123 in chapters).
- If Z3 loses the connection, add a one-line lemma that derives view_eq from feq_full.
- This phase is lower priority; do it after the main feq cleanup is stable.

## Phase 4: Types.rs broadcast cleanup

`Types.rs` has 15 `obeys_feq_full` lines (broadcast proofs for Edge, Pair, etc.).
These stay — they are the axioms that make feq_full true for concrete types.
No changes needed here.

## Constructor Concern

Constructors like `empty()` that `ensures result.spec_X_wf()` will now need to prove
`obeys_feq_full::<T>()` as part of wf. Since feq is type-level (not instance-level),
the proof needs feq to come from somewhere:

- If the constructor already has `assert(obeys_feq_full_trigger::<T>())` (like
  ArraySetStEph does), the broadcast fires and provides feq. Keep the trigger assertion
  in the constructor body (it's the one place where it's genuinely needed).
- If the constructor doesn't ensure wf (like OrderedTableStEph `empty()`), no change
  needed.
- If a constructor ensures wf but lacks the trigger, add it.

## Risk Assessment

| Risk | Likelihood | Mitigation |
|---|---|---|
| Z3 can't extract feq from wf | Low | wf is `open spec fn`; solver unfolds it |
| Proof time regression | Medium | Batch by chapter; validate after each |
| Constructor can't prove feq | Low | Broadcast axiom fires on trigger assert |
| obeys_view_eq breaks | Medium | Phase 3 is separate; revert if needed |

## Execution Order

1. Phase 0 (feq.rs helper) — 10 min, validate
2. Phase 1 files 1-4 (Chap37 AVLTreeSeq family) — validate
3. Phase 1 files 5-6 (Chap41 AVLTreeSet family) — validate
4. Phase 1 files 7-11 (misc single-type) — validate
5. Phase 2 files 12-14 (Chap42 Table family) — validate
6. Phase 2 files 15-18 (Chap43 OrderedTable) — validate
7. Phase 2 files 19-21 (Chap43 AugOrderedTable) — validate
8. Phase 2 files 22-24 (Chap43 OrderedSet) — validate
9. Phase 2 files 25-26 (Chap65) — validate
10. Phase 3 (obeys_view_eq cleanup) — validate
11. Full RTT + PTT

## Estimated Outcome

| Metric | Before | After |
|---|---|---|
| `obeys_feq_full` in chapters | 490 | ~125 |
| `obeys_feq_full_trigger` | 175 | ~20 |
| `obeys_view_eq` | 123 | ~0 (Phase 3) |
| **Total lines saved** | | **~640** |

The remaining ~125 lines are in proof blocks, static functions, and the `feq.rs`
definitions themselves.

## Agent Assignment

This is mechanical, high-volume refactoring with validation gates. Ideal for 2-3 agents:

- **Agent A**: Phase 1 (single-type, ~11 files)
- **Agent B**: Phase 2 (two-type, ~15 files)
- **Agent C**: Phase 3 (view_eq cleanup) after A+B merge

Or 4 agents splitting Phase 2 by chapter (Chap42 / Chap43-OT / Chap43-Aug / Chap43-Set).
