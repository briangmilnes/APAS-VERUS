# Agent 2 Round 44 Report

## Summary

Closed 7 proof holes across Chap63 (Connectivity) and Chap59 (Johnson APSP).
Baseline: 15 holes assigned. Result: 8 remaining (7 closed).

- Verification: 4370 verified, 0 errors
- RTT: 2613 tests pass
- PTT: 143/147 pass (4 pre-existing Chap43 failures)

## Holes Before/After

| # | Chap | File | Before | After | Closed |
|---|------|------|--------|-------|--------|
| 1 | 63 | ConnectivityStEph.rs | 4 | 2 | 2 |
| 2 | 63 | ConnectivityMtEph.rs | 6 | 4 | 2 |
| 3 | 59 | JohnsonMtEphI64.rs | 5 | 2 | 3 |
| | | **Total** | **15** | **8** | **7** |

## Warnings Fixed

| # | Chap | File | Warning | Fix |
|---|------|------|---------|-----|
| 1 | 59 | JohnsonMtEphI64.rs | fn_missing_requires on `create_negative_cycle_result` | Added `requires n < usize::MAX` |

## Warnings Remaining (cannot fix without upstream changes)

| # | Chap | File | Warning | Reason |
|---|------|------|---------|--------|
| 1 | 63 | ConnectivityStEph.rs | fn_missing_ensures on `count_components_hof` | `star_contract` has no ensures |
| 2 | 63 | ConnectivityStEph.rs | fn_missing_ensures on `connected_components_hof` | `star_contract` has no ensures |
| 3 | 63 | ConnectivityMtEph.rs | fn_missing_ensures on `count_components_hof` | `star_contract_mt` has no ensures |
| 4 | 63 | ConnectivityMtEph.rs | fn_missing_ensures on `connected_components_hof` | `star_contract_mt` has no ensures |
| 5 | 59 | JohnsonStEphI64.rs | fn_missing_requires on `adjust_distance` | No real precondition |
| 6 | 59 | JohnsonStEphI64.rs | fn_missing_requires on `reweight_edge` | No real precondition |
| 7 | 59 | JohnsonMtEphI64.rs | fn_missing_requires on `adjust_distance_mt` | No real precondition |

## Chapters Closed

None fully closed. Both Chap63 and Chap59 have remaining holes.

## Techniques Used

1. **Closure verification inside `verus!`**: Removed `external_body` from `_hof` functions by adding explicit `requires`/`ensures` on closure parameters, using `iter:` syntax for verified iterator loops within closures.

2. **`compose_maps_parallel` proof**: Added ensures `forall|k: V::V| #[trigger] result@.contains_key(k) ==> partition_map@.contains_key(k)` with matching loop invariant. Already had a verified body; just needed spec annotations.

3. **Johnson APSP function proofs**: Proved `add_dummy_source`, `reweight_graph`, and `johnson_apsp` by adding requires/ensures matching the StEph patterns and writing while-loop/for-loop invariants for vertex construction and edge iteration.

4. **`#[cfg(verus_keep_ghost)]` gating**: Applied to `use vstd::std_specs::hash::obeys_key_model` imports to prevent RTT compilation failures (vstd::std_specs is only available under verus_keep_ghost).

## Remaining Holes and Blockers

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 63 | ConnectivityStEph.rs | `count_components` | Recursive; `sequential_star_partition` has no ensures for termination proof |
| 2 | 63 | ConnectivityStEph.rs | `connected_components` | Same as above |
| 3 | 63 | ConnectivityMtEph.rs | `count_components_mt` | Recursive; `parallel_star_partition` has no ensures |
| 4 | 63 | ConnectivityMtEph.rs | `connected_components_mt` | Same as above |
| 5 | 63 | ConnectivityMtEph.rs | `build_quotient_edges_parallel` | Uses `Arc::new` + `route_edges_parallel` |
| 6 | 63 | ConnectivityMtEph.rs | `route_edges_parallel` | Uses `ParaPair!` macro (thread spawning) |
| 7 | 59 | JohnsonMtEphI64.rs | `parallel_dijkstra_all` | Uses `ParaPair!` macro (thread spawning) |
| 8 | 59 | JohnsonStEphI64.rs | assume at line 329 in `reweight_graph` | Needs graph partition lemma `result@.A.len() <= graph@.A.len()` |

Holes 1-4 require Chap62 StarPartition/StarContraction to gain ensures clauses (termination decreases proofs).
Holes 5-7 are thread-spawn boundaries requiring `external_body` per project convention.
Hole 8 (StEphI64 assume) needs a lemma about edge-set size after reweighting — not attempted this round.
