# R44 Agent 2: Chap63 + Chap59 (15 holes)

## Assignment

Prove external_body functions in Chap63 (Connectivity) and Chap59 (Johnson APSP).
You proved 2 functions in Chap63 last round — continue with the same patterns.

## Baseline

125 holes total. 4366 verified. Your chapters: Chap63 (10), Chap59 (5).

## Target Holes

| # | Chap | File | Function | Line | Type |
|---|------|------|----------|------|------|
| 1 | 63 | ConnectivityStEph.rs | count_components | 77 | external_body |
| 2 | 63 | ConnectivityStEph.rs | connected_components | 105 | external_body |
| 3 | 63 | ConnectivityStEph.rs | count_components_hof | 188 | external_body |
| 4 | 63 | ConnectivityStEph.rs | connected_components_hof | 203 | external_body |
| 5 | 63 | ConnectivityMtEph.rs | count_components_mt | 85 | external_body |
| 6 | 63 | ConnectivityMtEph.rs | connected_components_mt | 113 | external_body |
| 7 | 63 | ConnectivityMtEph.rs | build_quotient_edges_parallel | 142 | external_body |
| 8 | 63 | ConnectivityMtEph.rs | route_edges_parallel | 160 | external_body |
| 9 | 63 | ConnectivityMtEph.rs | count_components_hof | 237 | external_body |
| 10 | 63 | ConnectivityMtEph.rs | connected_components_hof | 250 | external_body |
| 11 | 63 | ConnectivityMtEph.rs | compose_maps_parallel | 212 | fn_missing_ensures |
| 12 | 59 | JohnsonMtEphI64.rs | johnson_apsp | 49 | external_body |
| 13 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | 80 | external_body |
| 14 | 59 | JohnsonMtEphI64.rs | add_dummy_source | 142 | external_body |
| 15 | 59 | JohnsonMtEphI64.rs | reweight_graph | 169 | external_body |

Also fix these warnings (not counted as holes but should be addressed):
- Chap59/JohnsonStEphI64.rs: `reweight_graph` assume at line 329
- Chap59/JohnsonStEphI64.rs: `adjust_distance` fn_missing_requires
- Chap59/JohnsonStEphI64.rs: `reweight_edge` fn_missing_requires
- Chap59/JohnsonMtEphI64.rs: `create_negative_cycle_result` fn_missing_requires

## Strategy

### Chap63 — Connectivity

Start with **ConnectivityStEph.rs** (St version first):

1. `count_components` and `connected_components` are **recursive** — they call
   `sequential_star_partition`, build a quotient graph, then recurse. These may need to
   keep external_body if Verus can't verify termination. Try removing external_body; if
   Verus complains about decreases, keep it and move on.

2. `count_components_hof` and `connected_components_hof` delegate to `star_contract`
   with closure arguments. These are higher-order wrappers — removing external_body may
   work if the closure specs propagate.

3. `compose_maps_parallel` is already proved but missing ensures — add the ensures clause.

Then adapt to **ConnectivityMtEph.rs** (Mt versions).

### Chap59 — Johnson

JohnsonMtEphI64.rs has 4 external_body functions. These use `HashMapWithViewPlus` (already
migrated) and call `dijkstra`/`bellman_ford` from Chap57/58.

- `add_dummy_source` and `reweight_graph` are helper functions with loops — use the
  standard iterator proof pattern.
- `johnson_apsp` is the main algorithm — may be harder.
- `parallel_dijkstra_all` uses threads — may need to keep external_body on the spawn.

### Proof patterns (from your R43 work):

1. **Iterator loops**: `for x in collection.iter()` with invariant.
2. **clone_plus()**: Use for owned copies in verified code.
3. **SetStEph::empty()**: For initial empty sets.
4. **Delegation**: Functions that just call another function — remove external_body.

### Key imports:
```rust
use crate::vstdplus::clone_plus::clone_plus::*;
```

### What NOT to do:
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT replace HashMap with std::collections::HashMap — we already migrated to
  HashMapWithViewPlus. Keep it.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes. Run `scripts/holes.sh src/Chap63/ src/Chap59/`.
Write your report to `plans/agent2-round44-report.md`.
