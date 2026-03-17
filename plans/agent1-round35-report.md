# Agent 1 — Round 35 Report

## Summary

Proved 12 external_body ordering operations across OrderedSetStEph.rs and
OrderedSetStPer.rs in Chapter 43 using linear-scan proofs with TotalOrder.

## Verification

- Before: 4194 verified, 0 errors
- After: 4214 verified, 0 errors (+20)
- RTT: 2613 tests, 2613 passed
- Total holes: 146 (project-wide, excludes vstdplus/standards/experiments)

## Techniques

- **Linear scan with TotalOrder**: Instead of adding sortedness to struct bounds
  (which would cascade TotalOrder into MtEph via MtKey), each ordering operation
  scans all elements via `TotalOrder::cmp` and tracks min/max/best.
- **Ghost value sequences**: StPer lacks `spec_inorder_values`, so ghost sequences
  `ghost_vals: Seq<T>` built incrementally via `ghost_vals = ghost_vals.push(*elem_ref)`
  bridge between Seq<T::V> (views) and actual T values for TotalOrder proofs.
- **feq view-injective bridging**: `obeys_feq_full::<T>()` + `lemma_cloned_view_eq`
  bridge clone operations; feq view-injective property connects arbitrary `t: T` to
  specific sequence elements via `t@ == elements@[j] ==> t == ghost_vals[j]`.
- **Provenance-based disjointness**: For split_rank, tracked which indices contributed
  to left vs right sets. Combined with `no_duplicates()`, proved disjointness by
  showing any element in both sets would require equal views at different indices.

## Holes Closed Per File

| # | Chap | File | Before | After | Closed |
|---|------|------|--------|-------|--------|
| 1 | 43 | OrderedSetStEph.rs | 11 | 4 | 7 |
| 2 | 43 | OrderedSetStPer.rs | 9 | 4 | 5 |

## Operations Proved

| # | Chap | File | Operation | Technique |
|---|------|------|-----------|-----------|
| 1 | 43 | OrderedSetStEph.rs | first | Linear scan min, TotalOrder |
| 2 | 43 | OrderedSetStEph.rs | last | Linear scan max, TotalOrder |
| 3 | 43 | OrderedSetStEph.rs | previous | Linear scan best predecessor |
| 4 | 43 | OrderedSetStEph.rs | next | Linear scan best successor |
| 5 | 43 | OrderedSetStEph.rs | from_seq | Iterate + insert (added requires) |
| 6 | 43 | OrderedSetStEph.rs | get_range | Iterate + filter + subset_of |
| 7 | 43 | OrderedSetStEph.rs | split_rank | Provenance-based partition |
| 8 | 43 | OrderedSetStPer.rs | first | Ghost seq + linear scan min |
| 9 | 43 | OrderedSetStPer.rs | last | Ghost seq + linear scan max |
| 10 | 43 | OrderedSetStPer.rs | previous | Ghost seq + best predecessor |
| 11 | 43 | OrderedSetStPer.rs | next | Ghost seq + best successor |
| 12 | 43 | OrderedSetStPer.rs | split_rank | Provenance-based partition |

## Remaining Holes

| # | Chap | File | Operation | Blocker |
|---|------|------|-----------|---------|
| 1 | 43 | OrderedSetStEph.rs | split | Needs TotalOrder on trait; cascades to MtEph |
| 2 | 43 | OrderedSetStEph.rs | rank | filter-based spec with existential quantifiers |
| 3 | 43 | OrderedSetStEph.rs | select | filter-based spec with existential quantifiers |
| 4 | 43 | OrderedSetStEph.rs | Iterator::next | STD_TRAIT_IMPL FP |
| 5 | 43 | OrderedSetStPer.rs | split | Needs TotalOrder on trait; cascades to MtEph |
| 6 | 43 | OrderedSetStPer.rs | rank | filter-based spec with existential quantifiers |
| 7 | 43 | OrderedSetStPer.rs | select | filter-based spec with existential quantifiers |
| 8 | 43 | OrderedSetStPer.rs | Iterator::next | STD_TRAIT_IMPL FP |

## Other Changes

- Made `lemma_inorder_values_maps_to_views` public in AVLTreeSetStEph.rs (Chap41).
- Added `requires seq.spec_avltreeseqstper_wf()` to `from_seq` in OrderedSetStEph trait.
- Added `obeys_feq_full` to imports in both files.
- Added `requires self.spec_orderedsetsteph_wf()` to trait methods: previous, next,
  split, get_range, rank, split_rank (StEph).

## fn_missing_requires

`from_sorted_elements` in both StEph and StPer is a free function taking `Vec<T>` with
no precondition. Left for user to annotate with `// veracity: no_requires`.
