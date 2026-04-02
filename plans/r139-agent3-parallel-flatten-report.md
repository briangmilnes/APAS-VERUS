# R139 Agent 3 Report — Parallel Flatten for ArraySeqMtEphSlice

## Summary

Implemented parallel D&C flatten for `ArraySeqMtEphSliceS`, added Clone/PartialEq/Eq
impls, strengthened the AdjTableGraphMtPer capacity assume with a bounded-size helper,
and added 5 RTTs. Full crate verifies clean: 5592 verified, 0 errors; 3621 RTTs pass;
221 PTTs pass.

## Changes

### 1. Parallel flatten (Chap19/ArraySeqMtEphSlice.rs)

Added `flatten` and `flatten_dc_vec` as free functions (not trait methods) due to type
bound nesting: the trait requires `T: Eq + Clone`, and `ArraySeqMtEphSliceS<T>` can't
satisfy `Eq` without `T: View` — but adding View to the trait bound would be a breaking
change.

**Algorithm**: D&C on O(1) outer slices. Base: empty returns empty Vec, single-element
returns `to_vec()` of the inner sequence. Recursive: split outer at midpoint, join two
recursive flattens, extend left Vec with right Vec.

**Spec**: `spec_nested_wf` predicate checks outer window validity and all inner windows.
Ensures `flattened.spec_arrayseqmtephslice_wf()`.

**Cost**: Work O(sum |a[i]|), Span O(lg^2 |a| + max |a[i]|) — differs from APAS
O(lg |a|) due to Vec concat at each D&C level.

### 2. Clone/PartialEq/Eq for ArraySeqMtEphSliceS (Chap19/ArraySeqMtEphSlice.rs)

Added to enable `ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>` nesting:

- **Clone**: O(1) — Arc::clone + copy start/len.
- **PartialEq**: Compares start/len fields, assumes view equality (standard assume bridge).
  Conservative: returns false for equal slices with different backing arcs.
- **Eq**: Marker trait, no body.
- **PartialEqSpecImpl**: cfg-gated, inside verus!.

### 3. AdjTableGraphMtPer capacity fix (Chap52/AdjTableGraphMtPer.rs)

Added `assert_avltreesetmtper_bounded_size` helper in Chap41/AVLTreeSetMtPer.rs that
calls `size()` (returns usize) to prove `neighbors@.len() <= usize::MAX`. The assume
remains for the strict `< usize::MAX` bound that `delete` requires — the off-by-one
gap (a neighbor set of exactly usize::MAX elements) is unreachable in practice but
cannot be proved from the type invariant alone.

**Hole status**: unchanged (1 assume), but now documented with bounded-size proof.

### 4. fn_missing_requires on helpers (Chap41/AVLTreeSetMtPer.rs)

Both `assert_avltreesetmtper_always_wf` and `assert_avltreesetmtper_bounded_size` have
no requires — they work on any `&AVLTreeSetMtPer<T>` via ParamBST's type_invariant.
Left for user to annotate with `// veracity: no_requires`.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 19 | ArraySeqMtEphSlice.rs | flatten, flatten_dc_vec, spec_nested_wf, Clone, PartialEq, Eq |
| 2 | 41 | AVLTreeSetMtPer.rs | assert_avltreesetmtper_bounded_size helper |
| 3 | 52 | AdjTableGraphMtPer.rs | Replace raw assume with bounded_size + documented assume |
| 4 | 19 | tests/TestArraySeqMtEphSlice.rs | 5 flatten RTTs |

## Verification

| Metric | Count |
|--------|-------|
| Verified | 5592 |
| Errors | 0 |
| RTTs | 3621 pass |
| PTTs | 221 pass |
| Trigger warnings | 0 |

## Holes

| # | Chap | File | Line | Type | Description |
|---|------|------|------|------|-------------|
| 1 | 52 | AdjTableGraphMtPer.rs | 472 | assume | neighbors@.len() < usize::MAX (proved <=, gap on <) |

New assumes added in Chap19: 1 (PartialEq assume bridge, standard pattern).
New accepts: 0.
