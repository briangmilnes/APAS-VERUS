# R127 Agent 2 Report — Parallelize Chap18 ArraySeqMtEph Trait Methods

## Summary

Parallelized **reduce** and **map** trait methods in `src/Chap18/ArraySeqMtEph.rs` using
divide-and-conquer with `join()`. Added `reduce_dc` and `map_dc` helper functions in the
bare impl block. Updated trait declarations with `Clone + Send + Sync + 'static` bounds.

## Functions Parallelized

| # | Chap | File | Function | Old Span | New Span | Status |
|---|------|------|----------|----------|----------|--------|
| 1 | 18 | ArraySeqMtEph.rs | reduce | O(n) | O(lg n) | Parallel D&C via join |
| 2 | 18 | ArraySeqMtEph.rs | map | O(n) | O(lg n + max S(f)) | Parallel D&C via join |

## Functions Kept Sequential (with reasons)

| # | Chap | File | Function | Span | Reason |
|---|------|------|----------|------|--------|
| 3 | 18 | ArraySeqMtEph.rs | filter | O(n) | Multiset ensures needs distribution lemmas for D&C proof |
| 4 | 18 | ArraySeqMtEph.rs | subseq | O(j) | Vec copy; APAS O(1) assumes persistent arrays |
| 5 | 18 | ArraySeqMtEph.rs | append | O(\|a\|+\|b\|) | Vec copy+concat; no parallelizable computation |
| 6 | 18 | ArraySeqMtEph.rs | update | O(n) | Vec copy with replacement; no parallelizable computation |
| 7 | 18 | ArraySeqMtEph.rs | inject | O(n+m) | Deterministic ordering prevents parallel writes |
| 8 | 18 | ArraySeqMtEph.rs | ninject | O(n+m) | Delegates to inject; ninject_par exists separately |
| 9 | 18 | ArraySeqMtEph.rs | scan | O(n) | Needs Blelloch parallel prefix algorithm |
| 10 | 18 | ArraySeqMtEph.rs | tabulate | O(n) | External callers (Chap52, Chap54) pass non-'static closures |
| 11 | 18 | ArraySeqMtEph.rs | flatten | O(Σ\|a_i\|) | D&C + Vec append gives O(n lg n) total, worse than sequential |

## Technical Constraints

**`join()` requires `'static` closures.** HFScheduler's `join()` uses `std::thread::spawn`
internally, requiring `FnOnce() -> T + Send + 'static`. This means:

1. **Closure-taking methods** (map, filter, reduce, scan, tabulate): the user's closure `F`
   must satisfy `F: Clone + Send + Sync + 'static` to be cloned and moved into join arms.
   This works for reduce and map (only internal/RTT callers with 'static closures). It
   BREAKS tabulate (Chap52/54 callers capture local references, not 'static).

2. **Copy-based methods** (subseq, append, update): the parallel structure is
   "split → join(copy_half, copy_half) → concat". But Vec concat is sequential O(n),
   negating the parallel copy. Span remains O(n).

3. **filter**: D&C filter produces correct results but proving the multiset ensures
   (`filtered.to_multiset() =~= input.to_multiset().filter(spec_pred)`) requires
   multiset-filter distribution over concatenation — lemmas not readily available in vstd.

## Implementation Details

- **reduce_dc**: Adapted from existing `reduce_par`. Takes `f: &F` (reference) instead of
  `f: F` (owned). Handles len=0 case (returns id) which reduce_par doesn't. Same monoid
  split-and-combine proof strategy.

- **map_dc**: New implementation with full per-element `f.ensures` proof. Ghost variables
  (`left_view`, `right_view`, `a_view`) capture sequence state before join closures move
  owned data. Post-join proof connects: subseq_copy indexing → ghost view equality →
  clone_fn spec preservation → append element identity.

- Existing `_par` functions (map_par, filter_par, reduce_par, ninject_par) are preserved
  unchanged. They remain the public parallel API with owned-closure signatures.

## Verification

- Isolate Chap18: **1012 verified**, 0 errors
- RTT: **3533 passed**, 0 skipped
- PTT: **221 passed**, 0 skipped

## What Would Unblock More Parallelization

1. **Scoped join** (non-'static): Adding `std::thread::scope`-based join to HFScheduler
   would allow tabulate, filter, map, reduce to accept closures capturing local references.
   This unblocks tabulate parallelization for Chap52/54 callers.

2. **Multiset distribution lemma**: `(A + B).filter(p) =~= A.filter(p) + B.filter(p)` in
   vstd would unblock filter's D&C proof.

3. **Parallel Vec fill**: O(1)-span tabulate requires writing to a pre-allocated array in
   parallel. Not achievable with safe Rust Vec.
