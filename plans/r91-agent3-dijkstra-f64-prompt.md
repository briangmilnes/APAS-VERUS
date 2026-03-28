# R91 Agent 3 — Build DijkstraStEphF64 from U64 version, STEP 20

## Objective

Replace the stub `src/Chap57/DijkstraStEphF64.rs` with a real implementation
by copying from the working `DijkstraStEphU64.rs` and converting to WrappedF64.

## Method

1. Read `src/Chap57/DijkstraStEphU64.rs` (328 lines, 0 holes, fully proved)
2. Copy its contents into `src/Chap57/DijkstraStEphF64.rs`
3. Rename: module name, struct names, trait names, wf predicates — all
   `U64` → `F64` or `u64` → `WrappedF64` as appropriate
4. Replace weight arithmetic:
   - `u64` weight type → `WrappedF64`
   - `a + b` → `a.dist_add(&b)` (returns WrappedF64, ensures `r@ == f64_add_spec(a@, b@)`)
   - `a <= b` / `a < b` → `a.dist_le(&b)` / `a.dist_lt(&b)`
   - `0u64` → `zero_dist()` (returns WrappedF64 with `d@ == 0.0f64`)
   - `u64::MAX` or sentinel → `unreachable_dist()` (returns WrappedF64 with `!d.spec_is_finite()`)
5. Replace graph type: `WeightedDirGraphStEphU64` → `WeightedDirGraphStEphF64`
6. Replace result type: `SSSPResultStEphI64` → `SSSPResultStEphF64` (or U64 equivalent)
7. Add broadcast uses for float axioms:
   ```rust
   broadcast use crate::vstdplus::float::float::group_float_finite_total_order;
   broadcast use crate::vstdplus::float::float::group_float_arithmetic;
   ```
8. For spec functions involving weight comparison, use `FloatTotalOrder::le` instead
   of integer `<=`
9. PQEntry already exists in the stub with WrappedF64 — keep it, merge with the
   U64 version's TotalOrder impl pattern

## Key differences from U64

- **No overflow**: WrappedF64 addition can produce infinity (not finite), not
  integer overflow. Guard with `spec_is_finite()` instead of overflow checks.
- **Comparison**: f64 comparison goes through `le_ensures` / `FloatTotalOrder`,
  not native `<=`.
- **View**: `WrappedF64` has `View` with `type V = f64`, so `self@` is `f64`.
  Spec functions work on `f64` values.

## Read first

- `src/Chap57/DijkstraStEphU64.rs` — **your source template** (copy this)
- `src/Chap57/DijkstraStEphF64.rs` — stub to replace
- `src/vstdplus/float.rs` — WrappedF64 API, f64_add_spec, broadcast axioms
- `src/Chap56/SSSPResultStEphF64.rs` — F64 result type
- `src/Chap06/WeightedDirGraphStEphF64.rs` — F64 graph type

## lib.rs

Uncomment `pub mod DijkstraStEphF64;` in Chap57 block.

## Isolation

```bash
scripts/validate.sh isolate Chap57
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify DijkstraStEphU64.rs.
- Do NOT add assume or accept (except the standard eq/clone workaround pattern).
- Use `external_body` on functions where WrappedF64 proof is too complex, but
  try to prove everything — agent3 just proved ETSPMtEph with this pattern.
- Keep the same trait structure, same ensures, same proof strategy as U64.
  Just swap the weight type.

## STEP 20

## Report

Write `plans/agent3-r91-dijkstra-f64-report.md`.
