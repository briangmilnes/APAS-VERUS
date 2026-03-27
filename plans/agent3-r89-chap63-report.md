# Agent 3 R89 — Chap63 ConnectivityMtEph Fix Report

## Objective

Fix 3 verification errors in `src/Chap63/ConnectivityMtEph.rs`.

## Errors Fixed

| # | Chap | File | Line | Error | Fix |
|---|------|------|------|-------|-----|
| 1 | 63 | ConnectivityMtEph.rs | 132 | `HashMapWithViewPlus::new()` precondition: missing `forall\|k1,k2\| k1@==k2@ ==> k1==k2` | Added to `compose_maps_parallel` requires |
| 2 | 63 | ConnectivityMtEph.rs | 138 | Loop invariant `forall\|k1,k2\|...` not satisfied before loop | Same fix as #1 (requires flows to invariant) |
| 3 | 63 | ConnectivityMtEph.rs | 139 | Loop invariant `result@.contains_key(k) ==> partition_map@.contains_key(k)` not maintained | Runtime `contains_key` check + `clone_view` for key |

## Changes

**`src/Chap63/ConnectivityMtEph.rs`** — `compose_maps_parallel` function:

1. Added `ClonePreservesView` to V bounds (needed for `clone_view` ensuring `u_key@ == u_ref@`).
2. Added `forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2` to requires (needed by `HashMapWithViewPlus::new()`).
3. Changed key cloning from `clone_plus()` to `clone_view()` for view-preservation guarantee.
4. Added runtime `partition_map.contains_key(u_ref)` check in loop body. The `contains_key` postcondition gives `is_in == partition_map@.contains_key(u_ref@)`, and the `if is_in` branch lets Verus prove the subset invariant. Always true at runtime since we're iterating partition_map.

## Technique: Runtime Check for Iterator Element Membership

The for-in loop ghost iterator (`HashMapWithViewPlusGhostIterator`) tracks position and kv_pairs but doesn't expose `pos >= 1` in the loop body (ghost_invariant only gives `0 <= pos <= len`). This makes index-based reasoning about the current element impossible without additional infrastructure.

The workaround: call `partition_map.contains_key(u_ref)` at runtime. Its postcondition provides `is_in == partition_map@.contains_key(u_ref@)`, and branching on `is_in` gives Verus the proof in the true branch. The false branch skips the insert, trivially maintaining the invariant. At runtime, the check is always true (O(1) amortized overhead per iteration).

## Verification Results

| Metric | Before | After |
|--------|--------|-------|
| Chap63 errors | 3 | 0 |
| Chap63 holes | 0 | 0 |
| Isolate verified | 1241 | 1243 |
| Full crate verified | — | 5254 |
| Full crate errors | 13 (all Chap61) | 13 (all Chap61, pre-existing) |

## Steps Used

3 of 20 (initial fix, bounds fix attempt, runtime check fix).
