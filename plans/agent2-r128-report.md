# R128 Agent 2 Report

## Summary

Wrote multiset filter distribution lemmas in vstdplus and parallelized the `filter` trait
method in three Mt files using divide-and-conquer with `join()`.

## Task 1: Multiset Filter Distribution Lemma

Added three lemmas to `src/vstdplus/multiset.rs`:

| # | Lemma | Purpose |
|---|-------|---------|
| 1 | `lemma_spec_filter_len_concat` | `spec_filter_len(a + b, p) == spec_filter_len(a, p) + spec_filter_len(b, p)` |
| 2 | `lemma_multiset_filter_distributes_over_add` | `m1.add(m2).filter(f) =~= m1.filter(f).add(m2.filter(f))` |
| 3 | `lemma_seq_concat_to_multiset_filter` | `(a + b).to_multiset().filter(f) =~= a.to_multiset().filter(f).add(b.to_multiset().filter(f))` |

All three verified on first attempt.

## Task 2–4: Parallel Filter

| # | Chap | File | Old Span | New Span | Status |
|---|------|------|----------|----------|--------|
| 1 | 18 | ArraySeqMtEph.rs | O(n) | O(lg n) | Verified |
| 2 | 18 | ArraySeqMtPer.rs | O(n) | O(lg n) | Verified |
| 3 | 19 | ArraySeqMtEph.rs | O(n) | O(lg n) | Verified |

### Pattern

Each file got a `filter_dc` helper that does:
- Base len 0: return empty, prove multiset filter of empty is empty
- Base len 1: test pred, return singleton or empty with multiset proof
- Recursive: split at mid, `join(filter_dc(left), filter_dc(right))`, append results,
  prove multiset filter distributes via `lemma_seq_concat_to_multiset_filter`

Trait `filter` signature updated to require `F: Clone + Send + Sync + 'static` and
`T: Send + Sync + 'static` (same as `map` and `reduce` already had).

The `filter` impl calls `filter_dc` with a bridge proof connecting `filter_dc`'s ensures
(expressed as `a.seq@`) to the trait's ensures (expressed as `Seq::new(a.seq@.len(), ...)`).

### Key proof technique

The postcondition uses `Seq::new(a.seq@.len(), |i: int| a.seq@[i])` while `filter_dc`
works with `a.seq@` directly. These are extensionally equal (`=~=`) but Verus treats each
lambda as a distinct closure. The bridge proof in `filter` asserts `s =~= a.seq@` where
`s = Seq::new(...)`, giving the SMT solver the equality it needs.

## Verification

```
Full validate: 5510 verified, 0 errors
RTT: 3534 passed, 0 skipped
PTT: 221 passed, 0 skipped
```
