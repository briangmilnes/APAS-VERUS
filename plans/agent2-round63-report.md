# Agent 2 — Round 63 Report

## Summary

Closed the `chain_lookup` clone bridge assume in Chap47 `StructChainedHashTable.rs`.

## Change

Replaced `assume(v == node.value)` after `.clone()` in `chain_lookup` (line 201) with the
standard `obeys_feq_clone` pattern:

1. Added `requires obeys_feq_clone::<Value>()` to `chain_lookup`, widened `Value` bound
   from `Clone` to `Eq + Clone`.
2. Replaced `node.value.clone()` + assume with `clone_elem(&node.value)` (which has
   `ensures c == *x` via the feq broadcast axiom).
3. Removed `// veracity: no_requires` annotation.
4. Propagated to callers:
   - `ParaHashTableStEphTrait::lookup` (line 375): already had `obeys_feq_clone::<Value>()`
     in requires — no change needed.
   - `EntryTrait::lookup` for `ChainList` (line 285): widened impl bounds from
     `Value: Clone` to `Value: Eq + View + Clone`, added
     `proof { assert(obeys_feq_full_trigger::<Value>()); }` to trigger the broadcast
     axiom that provides `obeys_feq_clone::<Value>()`.
   - Recursive call (line 206): `obeys_feq_clone` is a global property, propagates
     automatically.

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 47 | StructChainedHashTable.rs | 1 | 0 |

**Total project holes: 6 → 5.**

## Verification

- 4504 verified, 0 errors
- 2610 RTT passed
- 147 PTT passed
