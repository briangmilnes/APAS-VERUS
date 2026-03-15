# Chap35 Spec Audit — Order Statistics (Select)

## Summary

All exec functions have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | OrderStatSelectStEph.rs | select | a.spec_len() <= MAX | k>=len→None; k<len→Some(kth order statistic) | **strong** |
| 2 | OrderStatSelectStPer.rs | select | a.spec_len() <= MAX | k>=len→None; k<len→Some(kth order statistic) | **strong** |

## Notes

- Postcondition directly asserts kth-order statistic (kth smallest element via seq.sort_by).
- Three-way partition invariant: left < pivot, middle == pivot, right > pivot.
- Multiset preservation through partitioning.
- Faithfully encodes APAS Algorithm 35.2 (contraction-based select).
