# R91 Agent 4 — Build BellmanFordStEphF64 + JohnsonF64 from I64 versions, STEP 20

## Objective

Replace the stubs for BellmanFordStEphF64, JohnsonStEphF64, and JohnsonMtEphF64
with real implementations by copying from the working I64 versions and converting
to WrappedF64.

## Files to create (3)

### 1. BellmanFordStEphF64 (from BellmanFordStEphI64, 327 lines)

Copy `src/Chap58/BellmanFordStEphI64.rs` → `src/Chap58/BellmanFordStEphF64.rs`.
- `i64` weight → `WrappedF64`
- `a + b` → `a.dist_add(&b)`
- `a <= b` → `a.dist_le(&b)`
- `0i64` → `zero_dist()`
- `i64::MAX` sentinel → `unreachable_dist()`
- `WeightedDirGraphStEphI128` → `WeightedDirGraphStEphF64`
- `SSSPResultStEphI64` → `SSSPResultStEphF64`
- `AllPairsResultStEphI64` → `AllPairsResultStEphF64`

### 2. JohnsonStEphF64 (from JohnsonStEphI64, 492 lines)

Copy `src/Chap59/JohnsonStEphF64.rs`. Johnson calls BellmanFord + Dijkstra.
- Same arithmetic conversions as above
- Import the new F64 versions of Dijkstra and BellmanFord
- `adjust_distance` helper: i128 intermediate → use f64_add/f64_sub directly
  (no overflow concern with floats, just finiteness)

### 3. JohnsonMtEphF64 (from JohnsonMtEphI64, 488 lines)

Copy `src/Chap59/JohnsonMtEphF64.rs`. Same as StEph but with ParaPair!
parallel Dijkstra.

## Key conversion pattern

```rust
// I64 version:
let new_dist = dist_u + weight;
if new_dist < dist_v { ... }

// F64 version:
let new_dist = dist_u.dist_add(&weight);
if new_dist.dist_lt(&dist_v) { ... }
```

For spec functions:
```rust
// I64: spec fn uses native arithmetic
open spec fn spec_relaxed(d_u: i64, w: i64, d_v: i64) -> bool { d_u + w < d_v }

// F64: spec fn uses float specs
open spec fn spec_relaxed(d_u: f64, w: f64, d_v: f64) -> bool {
    f64_add_spec(d_u, w).le(d_v) && f64_add_spec(d_u, w) != d_v
}
```

## Broadcast axioms

Add to each file:
```rust
broadcast use {
    crate::vstdplus::float::float::group_float_finite_total_order,
    crate::vstdplus::float::float::group_float_arithmetic,
};
```

## Read first

- `src/Chap58/BellmanFordStEphI64.rs` — **source template for BellmanFord**
- `src/Chap59/JohnsonStEphI64.rs` — **source template for Johnson St**
- `src/Chap59/JohnsonMtEphI64.rs` — **source template for Johnson Mt**
- `src/vstdplus/float.rs` — WrappedF64 API
- `src/Chap56/SSSPResultStEphF64.rs` — F64 result type
- `src/Chap56/AllPairsResultStEphF64.rs` — F64 all-pairs result type

## lib.rs

Uncomment all 3:
- `pub mod BellmanFordStEphF64;` in Chap58
- `pub mod JohnsonStEphF64;` in Chap59
- `pub mod JohnsonMtEphF64;` in Chap59

## Isolation

Start with BellmanFord alone:
```bash
scripts/validate.sh isolate Chap58
```

Then Johnson (pulls in BellmanFord + Dijkstra):
```bash
scripts/validate.sh isolate Chap59
```

**IMPORTANT:** DijkstraStEphF64 is being built by agent3 concurrently. If Johnson
needs Dijkstra and it's not ready, use `external_body` on the Johnson functions
that call Dijkstra, and note the dependency. BellmanFord is independent — do that first.

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify the I64 source files.
- Do NOT add assume or accept.
- Use external_body where WrappedF64 proofs are too complex.
- Prioritize: BellmanFord first (independent), then JohnsonStEph, then JohnsonMtEph.
- If Johnson can't link to DijkstraF64 (agent3 building it), external_body the
  calling functions and move on.

## STEP 20

## Report

Write `plans/agent4-r91-bellmanford-johnson-f64-report.md`.
