# R139 Agent 4 Report: Blelloch Parallel Prefix Scan

## Summary

Implemented parallel inclusive scan on `ArraySeqMtEphSliceS<T>` using D&C with
fork-join parallelism via `join()`. The scan splits the sequence at the midpoint
using O(1) slices, recursively scans both halves in parallel, then adjusts the
right-half prefixes by combining with the left-half total.

## What was done

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | 19 | ArraySeqMtEphSlice.rs | Added `spec_prefix_fold` spec fn |
| 2 | 19 | ArraySeqMtEphSlice.rs | Added `lemma_prefix_fold_matching` proof fn |
| 3 | 19 | ArraySeqMtEphSlice.rs | Added `lemma_prefix_fold_split` proof fn |
| 4 | 19 | ArraySeqMtEphSlice.rs | Added `lemma_prefix_fold_eq_fold_left` proof fn |
| 5 | 19 | ArraySeqMtEphSlice.rs | Added `scan` to trait + impl |
| 6 | 19 | ArraySeqMtEphSlice.rs | Added `scan_dc_vec` D&C helper |
| 7 | 19 | TestArraySeqMtEphSlice.rs | Added 8 scan RTTs |

## Algorithm

D&C parallel scan:
1. Split at midpoint (O(1) with slices)
2. Recursively scan left and right halves in parallel via `join()`
3. Left prefixes are correct as-is
4. Right prefixes need adjustment: `result[mid+j] = f(left_total, right_prefix[j])`
5. Total = `f(left_total, right_total)`

Work: O(n lg n) — adjustment step is O(n/2) at each level.
Span: O(n) — sequential adjustment loop at each level.

APAS specifies O(n) work, O(lg n) span, which requires the contraction-based
approach (Chap27). The D&C approach here trades optimal work for simpler code
and natural fit with O(1) slice splitting.

## Proof technique

The proof required a custom `spec_prefix_fold` spec fn that computes prefix
folds using direct recursion on indices, avoiding `Seq::take` and `fold_left`.
This was necessary because Z3 cannot reason through nested `Seq::new` /
`subrange` / `take` operations — extensional equality (`=~=`) fails when
comparing sequences derived from different base sequences.

Key lemmas:
- `lemma_prefix_fold_matching`: If two index functions agree on 0..n, their
  prefix folds agree. (Trivial induction.)
- `lemma_prefix_fold_split`: Monoid split: `prefix_fold(a, f, id, m+k) =
  f(prefix_fold(a, f, id, m), prefix_fold(shifted_a, f, id, k))`.
- `lemma_prefix_fold_eq_fold_left`: Connects `spec_prefix_fold` back to
  `spec_backing_seq().take(n).fold_left(id, f)` at the trait boundary.

## Verification

- Full validate: 5594 verified, 0 errors
- RTT: 3623 passed (8 new scan tests)
- PTT: 221 passed

## Scan ensures

The trait ensures matches the existing Chap18/Chap19 inclusive scan pattern:
```
scanned.0.spec_index(i) == self.spec_backing_seq().take(i + 1).fold_left(id, spec_f)
scanned.1 == self.spec_backing_seq().fold_left(id, spec_f)
```
