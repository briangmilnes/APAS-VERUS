# Agent 4 — R91 Report: BellmanFord + Johnson F64

## Objective

Build BellmanFordStEphF64, JohnsonStEphF64, and JohnsonMtEphF64 from working I64
templates by converting integer arithmetic to WrappedF64 float operations.

## Files Changed

| # | Chap | File | Action | Lines | Holes |
|---|------|------|--------|-------|-------|
| 1 | 58 | BellmanFordStEphF64.rs | Replaced stub with full impl | 260 | 0 |
| 2 | 59 | JohnsonStEphF64.rs | Replaced stub with helpers + external_body main | 345 | 1 |
| 3 | 59 | JohnsonMtEphF64.rs | Replaced stub with helpers + external_body main | 320 | 1 |
| 4 | — | lib.rs | Uncommented 3 modules | 3 | — |

## Holes Summary

| # | Chap | File | Function | Hole Type | Reason |
|---|------|------|----------|-----------|--------|
| 1 | 59 | JohnsonStEphF64.rs | `johnson_apsp` | external_body | Blocked on DijkstraStEphF64 (agent3) |
| 2 | 59 | JohnsonMtEphF64.rs | `johnson_apsp` | external_body | Blocked on DijkstraStEphF64 (agent3) |

## What Verified (0 errors in new code)

- **BellmanFordStEphF64**: Fully verified — `bellman_ford`, `reconstruct_predecessors`.
  All distance operations use WrappedF64 (`dist_add`, `dist_lt`, `dist_le`, `is_finite`, `eq`).
  No assume, no accept, no external_body on algorithmic logic.

- **JohnsonStEphF64 helpers**: `adjust_distance`, `reweight_edge`, `build_vertex_set`,
  `add_dummy_source`, `reweight_graph`, `create_negative_cycle_result` — all verified.

- **JohnsonMtEphF64 helpers**: `adjust_distance`, `add_dummy_source`, `reweight_graph`,
  `create_negative_cycle_result` — all verified.

## Key Conversion Patterns

| I64 Pattern | F64 Pattern |
|-------------|-------------|
| `i64` distance | `WrappedF64` distance |
| `UNREACHABLE` (i64::MAX) | `unreachable_dist()` / `UNREACHABLE_SPEC()` |
| `0i64` | `zero_dist()` |
| `d != UNREACHABLE` | `d.is_finite()` |
| `add_distance(d, w)` (i128 overflow) | `d.dist_add(&w)` + `is_finite()` check |
| `d < min_dist` | `d.dist_lt(&min_dist)` (requires both finite) |
| `clamp_weight(w: i128) -> i64` | Not needed (weights already WrappedF64) |
| `WeightedDirGraphStEphI128` | `WeightedDirGraphStEphF64` |
| `SSSPResultStEphI64` | `SSSPResultStEphF64` |
| `valid_key_type_WeightedEdge::<usize, i128>()` | `valid_key_type_WeightedEdge::<usize, WrappedF64>()` |

## Blocking Dependency

Both Johnson F64 files need `DijkstraStEphF64::dijkstra()` which agent3 is building
concurrently. When DijkstraStEphF64 is merged, the `external_body` on `johnson_apsp`
can be replaced with real implementations following the I64 templates.

## WeightedDirGraphStEphF64 ensures gap

The F64 graph's `from_weighed_edges` does not expose `g@.A` in its ensures (only `g@.V`).
This prevents proving `reweighted@.A.len() <= graph@.A.len()` in `reweight_graph`.
The I128 graph module apparently has stronger ensures. This gap is cosmetic since the
callers are `external_body`, but it should be fixed in WeightedDirGraphStEphF64 for
future fully-verified Johnson F64.

## Validation Results

- `scripts/validate.sh isolate Chap58`: 1359 verified, 2 errors (pre-existing ArraySeqStEph flakiness)
- `scripts/validate.sh isolate Chap59`: 2551 verified, 0 errors
- `scripts/rtt.sh`: 3076 passed
- Full `scripts/validate.sh`: OOM (20GB RSS, machine constraint — pre-existing)
- `scripts/ptt.sh`: Blocked by full-crate OOM (can't build rlib)

## Techniques Used

- Float arithmetic via WrappedF64 methods (dist_add, dist_sub, dist_lt, is_finite, eq)
- Broadcast axioms: group_float_finite_total_order, group_float_arithmetic
- Finiteness guards before float comparisons (dist_lt requires both operands finite)
- external_body with unimplemented!() for Dijkstra-blocked functions
