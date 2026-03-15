# Chap49 Spec Audit — Dynamic Programming (Subset Sum, Min Edit Distance)

## Summary

Spec functions (spec_subset_sum, spec_med) exist and are mathematically correct but are **disconnected from exec code**. Trait methods lack ensures linking results to spec functions. Accessors are weak (structural only).

## SubsetSumStEph.rs / SubsetSumStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | obeys_feq_clone | multiset_len == 0 | **weak** |
| 2 | from_multiset | — | multiset_len == input.len | **weak** |
| 3 | subset_sum (Eph) | — | multiset_len preserved | **weak** |
| 4 | subset_sum (Per) | — | (none) | **missing** |
| 5 | multiset | — | ms.len == multiset_len | **weak** |
| 6 | set (Eph only) | index < len | multiset_len preserved | **weak** |
| 7 | clear_memo (Eph only) | — | multiset_len preserved | **weak** |
| 8 | memo_size | — | (none) | **missing** |

## MinEditDistStEph.rs / MinEditDistStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | obeys_feq_clone | source_len==0, target_len==0 | **weak** |
| 2 | from_sequences | — | source_len, target_len match inputs | **weak** |
| 3 | min_edit_distance (Eph) | sum < MAX | source_len, target_len preserved | **weak** |
| 4 | min_edit_distance (Per) | sum < MAX | (none) | **missing** |
| 5 | source | — | s.len == source_len | **weak** |
| 6 | target | — | t.len == target_len | **weak** |
| 7 | set_source (Eph only) | index < source_len | dims preserved | **weak** |
| 8 | set_target (Eph only) | index < target_len | dims preserved | **weak** |
| 9 | clear_memo (Eph only) | — | dims preserved | **weak** |
| 10 | memo_size | — | (none) | **missing** |

## Spec Functions Present

- `spec_subset_sum(s: Seq<int>, i: nat, j: int) -> bool` — correct recursive definition.
- `spec_med(s: Seq<int>, t: Seq<int>, i: nat, j: nat) -> nat` — correct recursive MED definition.
- `spec_memo_bounded` — memo entry bounds.

## Why No Code Fix

The spec functions operate on `Seq<int>` but the exec code uses generic `T: StT` with `Into<i32>` conversion. There is no spec-level bridge from `T` elements to `int` values. Fixing requires:

1. Define a spec function mapping `Seq<T>` to `Seq<int>` (e.g., `spec_to_int_seq`).
2. Add ensures to `subset_sum_rec` / `min_edit_distance_rec` connecting result to spec via that mapping.
3. Propagate ensures to trait methods.

This is substantial proof infrastructure work, not a simple spec annotation.

## Strong Spec Should Be

```
// SubsetSum
fn subset_sum(&mut self, target: i32) -> (found: bool)
    ensures found == spec_subset_sum(spec_to_int_seq(self.multiset@), self.spec_multiset_len(), target as int);

// MinEditDistance
fn min_edit_distance(&mut self) -> (dist: usize)
    ensures dist as nat == spec_med(spec_to_int_seq(self.source@), spec_to_int_seq(self.target@), self.spec_source_len(), self.spec_target_len());
```
