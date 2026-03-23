# Agent 3 — Round 61 Report

## Baseline → Final

| Metric | Baseline | Final | Delta |
|--------|----------|-------|-------|
| Verified | 4496 | 4502 | +6 |
| Errors | 0 | 0 | 0 |
| Holes | 12 | 11 | -1 |
| RTT | 2610 | 2610 | 0 |
| PTT | 147 | 147 | 0 |

## Summary

Proved the `assume(spec_seq_sorted_per(vals))` hole in Chap43 OrderedSetStPer.rs `select`
by creating a companion trait `AVLTreeSetStPerTotalOrderTrait` in Chap41 (mirroring the
existing StEph pattern), with `insert_sorted_per` that maintains sorted-under-TotalOrder
through binary-search insertion. Also transformed the equivalent StEph hole from an
algorithmic assume into an explicit contract requirement. Audited 10 eq_clone_workaround
warnings: all compliant.

## Per-File Changes

| # | Chap | File | Holes Before | Holes After | Notes |
|---|------|------|:------------:|:-----------:|-------|
| 1 | 41 | AVLTreeSetStPer.rs | 0 | 0 | Added companion trait + 4 lemmas + insert_sorted_per |
| 2 | 43 | OrderedSetStPer.rs | 1 | 0 | assume→assert, added spec_sorted_per requires |
| 3 | 43 | OrderedSetStEph.rs | 1 | 0 | assume→assert, added spec_sorted requires |
| 4 | 43 | OrderedSetMtEph.rs | 0 | 1 | New RWLOCK_GHOST assume for spec_sorted at boundary |

**Net: 12 → 11 holes (-1)**

## What Was Done

### Target 1: OrderedSetStPer.rs `select` — CLOSED

Created `AVLTreeSetStPerTotalOrderTrait` in Chap41/AVLTreeSetStPer.rs with:
- `spec_inorder_values_per` — recursive inorder traversal returning `Seq<T>` values
- `spec_seq_sorted_per` — sorted predicate using `TotalOrder::le`
- `lemma_inorder_values_maps_to_views_per` — connects value and view sequences
- `lemma_map_view_feq_implies_ext_eq_per` — map_values + feq → extensional equality
- `lemma_push_sorted_per` — appending ≥ last preserves sorted
- `lemma_subseq_sorted_per` — subrange of sorted is sorted
- `insert_sorted_per` — ~200 lines: binary search to find position, positional tracking
  in rebuild loops, 5-case sorted proof (both-before, before-at, before-after, at-after,
  both-after), feq bridge for clone case

In OrderedSetStPer.rs: added `spec_sorted_per()` to trait/impl, added to select's requires,
replaced `assume(spec_seq_sorted_per(vals))` with `assert`.

### Target 2: OrderedSetStEph.rs `select` — TRANSFORMED

Same contract change as StPer, reusing existing StEph infrastructure. Added `spec_sorted()`
to trait/impl, added to select's requires, replaced `assume(spec_seq_sorted(vals))` with
`assert`. The MtEph wrapper needed a new RWLOCK_GHOST assume for `inner.spec_sorted()` at
the lock boundary. Net 0 for StEph, but the hole moved from algorithmic (inside select body)
to structural (at RwLock boundary).

### Target 3: eq_clone_workaround Audit

Spot-checked 10 warnings across Chap05, 17, 18, 23, 37, 41, 43, 47, 50, 53:
- 8/8 in `PartialEq::eq` or `Clone::clone` bodies — correct
- 1 borderline: Chap47 `clone_elem` helper (not a trait impl, but functionally sound)
- 2 chapters had no eq_clone assumes (Chap50, Chap53)
- Zero violations found

## Techniques Used

1. **Companion trait pattern**: Gated sorted operations on `T: TotalOrder` via separate trait
2. **Binary search + positional tracking**: Loop invariants track element positions through
   rebuild, enabling 5-case sorted proof
3. **map_values + feq bridge lemma**: General lemma proving two sequences equal when their
   mapped views match and feq holds — solves the clone-preserves-sorted problem
4. **Requires propagation**: Made sorted an explicit precondition rather than a hidden assume

## Remaining Hole (Chap43)

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 43 | OrderedSetMtEph.rs | `assume(inner.spec_sorted())` | RWLOCK_GHOST: invariant doesn't track sorted |

To close: propagate `spec_sorted` to the MtEph trait, or add sorted to the RwLock invariant.
