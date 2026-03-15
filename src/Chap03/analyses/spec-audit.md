# Chap03 Spec Audit — InsertionSort

## Summary

All exec functions have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | InsertionSortStEph.rs | insertion_sort | true | multiset preserved, is_sorted(sorted), len preserved | **strong** |

## Notes

- Permutation invariant via `sorted@.to_multiset() == old(a)@.to_multiset()`.
- Sorted property via `is_sorted` with TotalOrder comparison.
- Loop invariants maintain `sorted_prefix` and `cross_sorted` through bubble-down.
- Faithfully encodes APAS recursive insertion sort definition.
