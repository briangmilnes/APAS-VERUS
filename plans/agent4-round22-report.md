# Agent 4 — Round 22 Report: Verusify Chap58 Bellman-Ford

## Mission

Move BellmanFordStEphI64.rs from outside `verus!` to fully verified inside `verus!` with
specs, loop invariants, and no assumes/accepts.

## Results

| # | Chap | File | Holes Before | Holes After | Change |
|---|------|------|-------------|------------|--------|
| 1 | 58 | BellmanFordStEphI64.rs | 0 (invisible) | 2 | +2 (net real gain) |
| 2 | 58 | BellmanFordStEphF64.rs | 0 (stub) | 0 | 0 |

Before: Algorithm was behind `#[cfg(not(verus_keep_ghost))]` — Verus never saw it, so
veracity reported 0 holes. The code was entirely unverified.

After: Algorithm fully inside `verus!` with specs, loop invariants, and 7 verified functions.
2 remaining holes are `external_body` on String creation helpers (non-algorithmic).

## What Changed

### BellmanFordStEphI64.rs — Complete Rewrite

1. **Replaced HashMap with Vec**: `HashMap<usize, i64>` → `Vec<i64>` indexed by vertex
   (vertices are 0..n). Required for Verus verification.

2. **Moved everything inside `verus!`**: Removed `#[cfg(not(verus_keep_ghost))]` gate.
   All algorithmic code now verified.

3. **Added trait with specs**:
   - `requires`: `source < graph@.V.len()`, `spec_labgraphview_wf(graph@)`,
     `valid_key_type_WeightedEdge::<usize, i128>()`
   - `ensures`: result distances length == vertex count, source preserved

4. **Helper functions with full specs**:
   - `clamp_weight(w: i128) -> i64`: 3-branch ensures covering all i128 ranges
   - `add_distance(d: i64, w: i64) -> i64`: overflow-safe via i128 intermediate,
     3-branch ensures for overflow/underflow/normal cases
   - `reconstruct_predecessors`: full spec with ghost tracking of original distances

5. **Loop invariants**: 5 loops with complete invariants (init, main relaxation outer,
   main relaxation inner neighbor scan, convergence copy, predecessor reconstruction
   with nested neighbor scan).

6. **Broadcast uses**: `group_hash_set_with_view_plus_axioms` (SetStEph finiteness) +
   `group_Pair_axioms` (valid_key_type for Pair types). Required for SetStEph iteration.

7. **`external_body` on string helpers only**: `neg_cycle_error_string()` and
   `algorithm_error_string()` — Verus cannot verify `String::from()` / `.to_string()`.
   Not algorithmic logic.

### BellmanFordStEphF64.rs — No Changes

Remains a stub. Blocked on `WeightedDirGraphStEphF64` not existing.

## Techniques

- **i128 intermediate arithmetic**: Two i64 values cast to i128 can't overflow when added.
  Enables verified overflow-safe distance computation without assumes.
- **`assert(n == graph@.V.len())`**: Bridge between exec `vertices().size()` and spec
  `graph@.V.len()`. Carried through loop invariants to connect postconditions.
- **Broadcast axioms for SetStEph iteration**: `in_neighbors_weighed` returns
  `SetStEph<Pair<usize, i128>>`. Iterating requires `spec_setsteph_wf()` which needs
  both `self@.finite()` (from hash_set broadcast) and `valid_key_type::<Pair<K,V>>()`
  (from Pair axioms broadcast).
- **Ghost variable for predecessor reconstruction**: `let ghost original_distances`
  tracks that `set_predecessor` doesn't modify distances.

## Verification

- 3966 verified, 0 errors
- 2600 RTT pass (all 10 BellmanFord tests pass)
- 2 holes (both `external_body` on String helpers)

## Remaining Holes

| # | Chap | File | Line | Type | Reason |
|---|------|------|------|------|--------|
| 1 | 58 | BellmanFordStEphI64.rs | 83 | external_body | `neg_cycle_error_string()` — String creation |
| 2 | 58 | BellmanFordStEphI64.rs | 88 | external_body | `algorithm_error_string()` — String creation |

Both are non-algorithmic. Verus cannot verify Rust's `String::from()` / `.to_string()`.
Standard pattern used across the codebase (same as DijkstraStEphI64.rs).
