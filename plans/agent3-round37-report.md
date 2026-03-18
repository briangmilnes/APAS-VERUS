# Agent 3 — Round 37 Report

## Summary

Proved `split` in OrderedSetStEph.rs and OrderedSetStPer.rs, and `calculate_reduction`
in AugOrderedTableMtEph.rs. Net result: **-2 actionable holes** (75 → 73).

Verification: 4288 verified, 0 errors.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedSetStEph.rs | 3 | 2 | -1 |
| 2 | 43 | OrderedSetStPer.rs | 3 | 2 | -1 |
| 3 | 43 | AugOrderedTableMtEph.rs | 2 | 2 | 0 |

**OrderedSetStEph/StPer**: Each had 3 external_body (split, rank, select). Split proved
(-1 each). Rank/select remain external_body.

**AugOrderedTableMtEph**: calculate_reduction external_body removed, but replaced with
assume for closure totality (`forall|v1, v2| reducer.requires((v1, v2))`). Net 0 for this
file since veracity counts the assume.

## Techniques

### split (both StEph and StPer)

Adapted the proven `split_rank` loop pattern with 3-way partitioning (left/right/found):

- Loop over `base_set.elements` using index `j`, calling `nth(j)` for element access.
- Used `feq(elem_ref, k)` from `vstdplus/feq.rs` to bridge exec `==` to spec view equality.
- Disjointness proved via provenance-by-index + no_duplicates: if `x` is in both left and
  right, two distinct indices map to the same value, contradicting no_duplicates.
- Coverage proved by showing every visited index is accounted for (left, right, or == k@).
- `found == old_view.contains(k@)` proved by contradiction: if !found but k@ in view,
  then k@ in elements sequence, so some visited index has k@, but all were shown != k@.

### calculate_reduction (AugOrderedTableMtEph)

Mirrored the StEph pattern: `assume(forall|v1, v2| reducer.requires((v1, v2)))` for
closure totality (unavoidable without cascading requires through 12+ callers). Used
`collect()` to get pairs, then while loop reducing from index 1.

### Iterator::next (attempted, reverted)

Removed external_body, added `assume(iter_invariant(self))` in body. Verified successfully.
However, veracity classifies external_body on `std::iter::Iterator::next` as a structural
false positive (STD_TRAIT_IMPL), so it was never counted as a hole. The assume IS counted.
Net effect would have been +2 holes, so changes were reverted.

## Remaining Holes in Agent 3 Files

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 43 | OrderedSetStEph.rs | rank external_body | Set::filter + existential quantifiers |
| 2 | 43 | OrderedSetStEph.rs | select external_body | Same as rank |
| 3 | 43 | OrderedSetStPer.rs | rank external_body | Same as rank |
| 4 | 43 | OrderedSetStPer.rs | select external_body | Same as rank |
| 5 | 43 | AugOrderedTableMtEph.rs | assume (closure totality) | Closure requires cascade |
| 6 | 43 | AugOrderedTableMtEph.rs | reduce_range_parallel ext_body | ParaPair! fork-join |

**rank/select** require connecting sorted-sequence indexing to set-filter cardinality through
TotalOrder + spec_inorder_values. The spec uses `Set::filter` with existential quantifiers
(`exists|i| 0 <= i < rank && sorted[i] == x`), making the proof extremely difficult.
Estimated 2-4 hours each. Deferred to future round.
