# R86 Agent 1 — Integerize Prim + Kruskal (WrappedF64 → u64), STEP 20

## Objective

Replace `WrappedF64` edge weights with `u64` in both `PrimStEph.rs` and
`KrustalStEph.rs`. This eliminates the 2 Prim float holes and removes all
float axiom dependencies from Chap65.

Do NOT change filenames.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

Read both files fully. Map every `WrappedF64` / float reference. Then make all
changes at once.

## Changes needed

### Type changes (both files)

- `WrappedF64` → `u64` everywhere (type params, struct fields, function sigs)
- `LabUnDirGraphStEph<V, WrappedF64>` → `LabUnDirGraphStEph<V, u64>`
- `LabEdge<V, WrappedF64>` → `LabEdge<V, u64>`

### PrimStEph.rs

- `PQEntry.priority: WrappedF64` → `PQEntry.priority: u64`
- Remove the entire `TotalOrder for PQEntry` block (lines ~51-75) that has the
  4 float assumes (reflexive/transitive/antisymmetric/total). Replace with a
  TotalOrder impl that uses u64's proven ordering. PQEntry ordering: compare
  by priority (u64), then by vertex as tiebreaker.
- `zero_dist()` → `0u64`
- `pq_entry_new(zero_dist(), ...)` → `pq_entry_new(0u64, ...)`
- `spec_is_finite()` preconditions → remove (integers always finite)
- `dist_add()` → `+` with overflow check (`requires a + b <= u64::MAX` or
  use checked addition)
- Remove `use crate::vstdplus::float::float::*` imports
- The `mst_weight` function sums edge weights — change from `dist_add` to `+`

### KrustalStEph.rs

- `sort_edges_by_weight`: change `val.le()` comparisons to `TotalOrder::le()`
  or just `<=` on u64. Remove `spec_is_finite()` from requires and invariants.
- Replace `<f64 as FloatTotalOrder>::reflexive/transitive/totality` proof calls
  with `<u64 as TotalOrder>::reflexive/transitive/total` — these have empty
  proof bodies for integers, so Z3 handles them automatically. You may not
  even need explicit proof calls.
- `WrappedF64 { val: 0.0 }` → `0u64`
- Remove `broadcast use group_float_finite_total_order`
- Remove float-related imports and `FloatBitsProperties`
- The `kruskal_greedy_phase` and `kruskal_process_edge` keep their existing
  external_body — don't try to prove them (Z3 &mut encoding issue, separate
  from float)
- The finiteness precondition on `kruskal_mst` (`forall|e| graph@.A.contains(e)
  ==> e.2.is_finite_spec()`) → remove entirely

### Test files

Check `tests/Chap65/TestKrustalStEph.rs` and `tests/Chap65/TestPrimStEph.rs` —
update test values from `WrappedF64 { val: X.X }` to `Xu64`.

## Expected result

- PrimStEph: 0 holes (the 2 float holes disappear — u64 TotalOrder is auto-proved)
- KrustalStEph: 1 hole remains (kruskal_process_edge — Z3 &mut issue, unrelated to float)
- All Chap65 holes go from 6 → 4

## Important

- Do NOT modify UnionFindStEph.rs.
- Do NOT change filenames.
- Do NOT add `assume` or `accept`.
- Read `src/vstdplus/total_order.rs` for the u64 TotalOrder impl.

## STEP 20

## Report

Write `plans/agent1-round86-report.md`.
