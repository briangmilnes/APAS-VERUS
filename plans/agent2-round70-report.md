# Agent 2 Round 70 Report — OrderedTableStPer Iterator Assume Elimination

## Summary

Eliminated the last algorithmic `assume` in `OrderedTableStPer.rs` by refactoring the
iterator to delegate to `std::vec::IntoIter<Pair<K, V>>` instead of maintaining manual
`pos`/`len` state.

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 43 | OrderedTableStPer.rs | 1 | 0 |

## Technique

**Problem:** `Iterator::next()` cannot declare `requires` clauses (Verus enforces trait
contract fidelity). The custom iterator manually tracked `pos`, `len`, and `sorted`
(an `ArraySeqStPerS<Pair<K,V>>`), requiring `assume(iter_invariant(self))` at entry to
`next()` because the position bounds couldn't be proved without a precondition.

**Solution:** Replaced the manual state with `std::vec::IntoIter<Pair<K, V>>`, which
has vstd-provided specs for `next()` in `vstd::std_specs::vec`. The `next()` body becomes
a single delegation call: `self.inner.next()`. The IntoIter's ensures directly satisfy the
outer ensures without any assume.

**Changes:**
1. `OrderedTableStPer.rs` — Replaced `{sorted, pos, len}` struct fields with `{inner: IntoIter<Pair<K,V>>}`.
   Changed View type from `(int, Seq<(K::V, V::V)>)` to `(int, Seq<Pair<K, V>>)`.
   Simplified `iter_invariant` to just `0 <= it@.0 <= it@.1.len()`.
   Removed `obeys_feq_full` from `iter()` and `into_iter()` requires (no longer needed
   since elements are moved, not cloned). `next()` now delegates to `self.inner.next()`.
2. `AugOrderedTableStPer.rs` — No changes needed (derives from base `iter()`).
3. `ProveOrderedTableStPer.rs` — Updated PTT loop patterns for new View type
   (`Seq<Pair<u64,u64>>` instead of `Seq<(u64,u64)>`).

## Remaining Warnings

| # | Chap | File | Line | Type | Note |
|---|------|------|------|------|------|
| 1 | 43 | OrderedTableStPer.rs | 3313 | assume_eq_clone_workaround | PartialEq::eq (allowed) |
| 2 | 43 | OrderedTableStPer.rs | 3325 | assume_eq_clone_workaround | Clone::clone (allowed) |
| 3 | 43 | OrderedTableStPer.rs | 3330 | fn_missing_wf_ensures | from_sorted_entries missing wf ensures |

The eq/clone workaround assumes are the standard pattern allowed by project rules.
The `from_sorted_entries` wf ensures requires adding `spec_key_unique_pairs_set` as a
precondition and proving it as a loop invariant — deferred to a future round.

## Verification

- `scripts/validate.sh`: 4435 verified, 0 errors
- `scripts/rtt.sh`: 2528 passed, 0 skipped
- `scripts/ptt.sh`: 145 passed, 0 skipped
