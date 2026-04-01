# Agent 4 — R127 Report: Parallelize Chap19 ArraySeqMtEph trait methods

## Summary

Parallelized 2 of 11 DIFFERS trait methods in `src/Chap19/ArraySeqMtEph.rs` via
D&C fork-join using `join()`. Added `map_dc` and `concat_seqs` helpers on the
bare impl. Delegated `reduce` to existing `reduce_par`.

## Functions parallelized

| # | Chap | File | Function | Old Span | New Span | Status |
|---|------|------|----------|----------|----------|--------|
| 1 | 19 | ArraySeqMtEph.rs | reduce | O(Sigma S(f)) | O(lg \|a\| * max S(f)) | Parallel via reduce_par |
| 2 | 19 | ArraySeqMtEph.rs | map | O(Sigma S(f(x))) | O(1 + max S(f(x))) | Parallel via map_dc |

## Functions kept sequential

| # | Chap | File | Function | Span | Reason |
|---|------|------|----------|------|--------|
| 1 | 19 | ArraySeqMtEph.rs | subseq | O(length) | Vec-backed copy inherent |
| 2 | 19 | ArraySeqMtEph.rs | append | O(\|a\|+\|b\|) | Vec-backed copy inherent |
| 3 | 19 | ArraySeqMtEph.rs | filter | O(\|a\| * (1+S(f))) | D&C multiset proof too complex |
| 4 | 19 | ArraySeqMtEph.rs | update | O(\|a\|) | Vec clone inherent |
| 5 | 19 | ArraySeqMtEph.rs | inject | O(\|a\|+\|b\|) | Ordered updates, inherently sequential |
| 6 | 19 | ArraySeqMtEph.rs | ninject | O(\|a\|+\|b\|) | Delegates to inject |
| 7 | 19 | ArraySeqMtEph.rs | scan | O(\|a\|) | Parallel scan (Blelloch) too complex |
| 8 | 19 | ArraySeqMtEph.rs | tabulate | O(Sigma S(f(i))) | External callers have non-'static closures |
| 9 | 19 | ArraySeqMtEph.rs | flatten | O(\|a\|+sum\|a_i\|) | D&C needs T: View for outer subseq_copy |

## New helpers added

| # | Chap | File | Function | Purpose |
|---|------|------|----------|---------|
| 1 | 19 | ArraySeqMtEph.rs | map_dc | D&C map with full element-level ensures |
| 2 | 19 | ArraySeqMtEph.rs | concat_seqs | Sequential Vec concat via into_iter (no Clone/Eq) |

## Trait signature changes

- `reduce`: Added `F: + Send + Sync + Clone + 'static`, `T: + Send + Sync + 'static`
- `map`: Added `U: + Send + Sync + 'static`, `F: + Send + Sync + Clone + 'static`,
  `where T: Clone + Eq + Send + Sync + 'static`, `obeys_feq_clone::<T>()` to requires

## Why not more methods?

Three fundamental constraints limited parallelization:

1. **`join()` requires `'static` closures.** The D&C closures must own all captured data.
   For methods taking `f: &F`, the reference isn't `'static`. Cloning `f` via `clone_fn`
   requires `F: Clone`, which Verus doesn't recognize for inline closures. This blocked
   filter (whose body constructs an inline closure for map).

2. **`subseq_copy` on `ArraySeqMtEphS<ArraySeqMtEphS<T>>` requires `T: View`** (through
   `Eq` on `ArraySeqMtEphS<T>`). Adding `T: View` cascades through filter and other
   callers. This blocked flatten.

3. **Multiset splitting proof** for D&C filter requires proving that
   `(m1 + m2).filter(p) = m1.filter(p) + m2.filter(p)` — a property not in vstd.
   This blocked the D&C filter proof.

## Verification

- Isolate Chap19: 847 verified, 0 errors
- Full crate: 5476 verified, 0 errors
- RTT: 3533 passed, 0 skipped
