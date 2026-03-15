# Chap36 Spec Audit — Quicksort

## Summary

All exec functions have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | QuickSortStEph.rs | quick_sort_first | a.len() <= MAX | multiset =~= sorted, len preserved | **strong** |
| 2 | QuickSortStEph.rs | quick_sort_median3 | a.len() <= MAX | multiset =~= sorted, len preserved | **strong** |
| 3 | QuickSortStEph.rs | quick_sort_random | a.len() <= MAX | multiset =~= sorted, len preserved | **strong** |
| 4 | QuickSortStEph.rs | median_of_three | — | result is one of inputs, == spec_median_of_three | **strong** |
| 5 | QuickSortStEph.rs | median3_pivot_idx | n >= 2 | idx in {0, n/2, n-1}, a[idx] == median | **strong** |
| 6 | QuickSortStEph.rs | concat_three | size sum <= MAX | out multiset == left + mid + right | **strong** |

## Notes

- All three sorting variants ensure permutation (multiset equality) + sorted output.
- Three pivot strategies: first element, median-of-three, random.
- Median-of-three specification is complete: result is neither min nor max.
- Faithfully encodes APAS Algorithm 36.1 with three pivot strategies.
