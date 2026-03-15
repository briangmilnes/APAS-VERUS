# Chap26 Spec Audit — Divide and Conquer

Audited: 2026-03-15, Agent 4, Round 19.
Prose source: prompts/Chap26.txt (Algorithms 26.2, 26.4, 26.5, 26.7).

## Summary

8 files, 0 holes, all verified specs **strong**. No changes needed.

Chapter 26 implements four D&C algorithms: reduce (Section 5), merge sort (Section 2),
scan (Section 3), and eTSP (Section 4). Each has StPer (sequential) and MtPer/MtEph
(parallel) variants.

## Per-File Classification

| # | File | Holes | Fns | Classification | Notes |
|---|------|:-----:|:---:|:--------------:|-------|
| 1 | DivConReduceStPer.rs | 0 | 5 | Strong | Alg 26.2 + Section 5 |
| 2 | DivConReduceMtPer.rs | 0 | 8 | Strong | Parallel reduce |
| 3 | MergeSortStPer.rs | 0 | 3 | Strong | Alg 26.4 |
| 4 | MergeSortMtPer.rs | 0 | 8 | Strong | Parallel merge sort |
| 5 | ScanDCStPer.rs | 0 | 3 | Strong | Alg 26.5 |
| 6 | ScanDCMtPer.rs | 0 | 3 | Strong | Parallel scan |
| 7 | ETSPStEph.rs | 0 | 7+3 | Strong/N/A | Alg 26.7 (3 outside verus!) |
| 8 | ETSPMtEph.rs | 0 | 8+4 | Strong/N/A | Parallel eTSP (4 outside verus!) |

## Spec-vs-Prose Detail

### DivConReduceStPer.rs (Example 26.2 + Section 5)

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `max_element` | forall a[i] <= max, exists a[i] == max | Ex 26.2: max element | Strong |
| 2 | `sum` | total == spec_iterate(s, +, 0) | Sec 5: reduce (+) 0 | Strong |
| 3 | `product` | total == spec_iterate(s, *, 1) | Sec 5: reduce (*) 1 | Strong |
| 4 | `any` | result == spec_iterate(s, \|\|, false) | Sec 5: reduce OR | Strong |
| 5 | `all` | result == spec_iterate(s, &&, true) | Sec 5: reduce AND | Strong |

`max_element` is especially strong: both upper-bound (forall) and achievability (exists)
postconditions match the textbook's "find the maximal element."

### MergeSortStPer.rs (Algorithm 26.4)

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `merge` | sorted + permutation (multiset eq) | Merge subroutine | Strong |
| 2 | `merge_sort` | sorted + permutation (multiset eq) | Alg 26.4 | Strong |

Postconditions directly encode Def 26.3 (comparison sorting): result is sorted
(forall i < j, r[i] <= r[j]) and a permutation (multiset equality with input).
The textbook correctness proof is: "by induction, l' and r' are sorted versions
of l and r... merge(l', r') returns a sorted version of a." The spec captures
exactly this.

### ScanDCStPer.rs (Algorithm 26.5)

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `scan_dc` | spec_scan_post: prefix fold + total | Alg 26.5 | Strong |
| 2 | `prefix_sums_dc` | spec_scan_post with (+, 0) | Convenience wrapper | Strong |

`spec_scan_post` asserts: `prefixes[i] == fold_left(input[0..i], id, f)` for all i,
and `total == fold_left(input, id, f)`. This is exactly Algorithm 26.5's
"prefixes[i] = f(id, a[0], ..., a[i-1])" and the total.

### ETSPStEph.rs (Algorithm 26.7)

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `etsp` | cycle of correct length, valid edges | Alg 26.7 | Strong |
| 2 | `etsp_inner` | cycle of correct length, valid edges | Recursive core | Strong |
| 3 | `sort_and_split` | structural split invariants | Split subroutine | Strong |
| 4 | `find_best_swap` | valid edge swap | Swap subroutine | Strong |
| 5-7 | lemmas | proof helpers | N/A | Strong |
| 8-10 | `*_impl` | N/A (outside verus!) | Reference impls | N/A |

The eTSP specs focus on structural correctness (valid cycle, correct length,
all points visited) rather than optimality, which is appropriate since eTSP is
a heuristic (NP-hard, Alg 26.7 is approximate). The `_impl` functions outside
verus! are reference implementations with no specs (as expected per convention).

## Verdict

No spec changes needed. All Chap26 specs faithfully encode Algorithms 26.2, 26.4,
26.5, and 26.7.
