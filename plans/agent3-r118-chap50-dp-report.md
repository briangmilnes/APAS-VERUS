# Agent 3 — R118 Chap50 DP Spec Strengthening Report

## Summary

Strengthened specs across 4 Chap50 files. Added wf predicates to St variants,
renamed misnamed wf in MtEph, strengthened RwLock invariant for OBST keys,
and tightened constructor/accessor ensures.

## Validation

- **Verified**: 798 (isolate Chap50)
- **RTT**: 3529 passed
- **PTT**: 221 passed
- **Trigger warnings**: 0

## Changes by File

| # | Chap | File | Change | Warnings Fixed |
|---|------|------|--------|----------------|
| 1 | 50 | MatrixChainStPer.rs | Added `spec_matrixchainstper_wf` (= `spec_memo_correct`), wf ensures on 3 constructors | 1 |
| 2 | 50 | MatrixChainStEph.rs | Added `spec_matrixchainsteph_wf` (= `spec_memo_correct`), wf ensures on 3 constructors + 3 mutation methods | 1 |
| 3 | 50 | MatrixChainMtEph.rs | Strengthened `from_dimensions` ensures: `len() ==` → `=~=` | 1 |
| 4 | 50 | OptBinSearchTreeMtEph.rs | Renamed `spec_obstmteph_wf` → `spec_optbinsearchtreemteph_wf`; upgraded `OptBSTMtEphKeysInv` from len-only to content-tracking (`expected_keys: Seq`); strengthened `keys()` ensures to `=~=`; rewrote `set_key_prob`/`update_prob` to clone-modify-rebuild pattern | 3 |

**Total warnings fixed: 6**

## Warnings Not Fixable (Architectural)

The remaining ~38 warnings from `veracity-compare-par-mut` are about memo-related
ensures present in St variants but absent from Mt variants. These are architecturally
correct differences:

1. **Mt views don't expose memo.** `MatrixChainMtPerV`, `MatrixChainMtEphV`,
   `OBSTMtPerV`, `OBSTMtEphV` have no `memo` field. The memo lives behind
   `Arc<RwLock<...>>` and correctness is maintained via `RwLockPredicate::inv`.
   Adding memo to the view would break the concurrency abstraction.

2. **Mt memo_size has no meaningful ensures.** The memo is behind a shared lock;
   by the time the caller uses the returned count, another thread may have changed
   it. The St variants can guarantee `n == self@.memo.len()` because they have
   exclusive access.

3. **Mt constructor memo-empty ensures are implicit.** Constructors create fresh
   `HashMapWithViewPlus::new()` and wire the RwLock with the correct predicate.
   The empty-memo property is guaranteed by construction but not expressible through
   the view type.

4. **Mt matrix_chain_rec memo ensures are enforced by RwLock inv.** The MtPer/MtEph
   `MatrixChainMt*MemoInv` includes `spec_memo_correct(self.dims, v@)` in its inv.
   Every `release_write` proves memo correctness. This is equivalent to the St
   ensures but expressed through the lock invariant, not the function postcondition.

These are not regressions — they are the expected consequence of the Mt architecture.
The comparison tool flags them because it doesn't understand that RwLock invariants
serve the same purpose as view-level memo specs.

## Techniques Used

- **RwLock invariant strengthening**: Upgraded `OptBSTMtEphKeysInv` from `expected_len: nat`
  to `expected_keys: Seq<KeyProb<T>>`, enabling content-level ensures on `keys()`.
- **Clone-modify-rebuild**: Replaced in-place RwLock mutation in `set_key_prob`/`update_prob`
  with clone→modify→`new_arc_rwlock` to maintain the stronger invariant.
- **Wf introduction**: Added `spec_memo_correct`-based wf predicates to St variants
  for consistency with the project's wf standard.
