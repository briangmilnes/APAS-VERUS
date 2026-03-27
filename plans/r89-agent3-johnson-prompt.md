# R89 Agent 3 — Prove JohnsonMtEphI64 parallel_dijkstra_all, STEP 20

## Objective

Remove the `external_body` from `parallel_dijkstra_all` in
`src/Chap59/JohnsonMtEphI64.rs`. This is the only hole in Chap59.

## The Function

`parallel_dijkstra_all` runs Dijkstra from each source vertex in parallel using
binary divide-and-conquer with `ParaPair!`. It splits the source range [start, end)
at the midpoint, recurses on both halves in parallel, then appends results.

The external_body comment says: "feq_clone preconditions on
ArraySeqStEphS::singleton/append and i64 overflow in distance adjustment."

## What blocks the proof

Two issues noted in the BYPASSED comment:

1. **feq_clone preconditions**: `ArraySeqStEphS::singleton` and `append` require
   `obeys_feq_clone::<T>()`. The closures for ParaPair! need this in their requires
   or it must be provable from the outer context.

2. **i64 overflow in distance adjustment**: After Dijkstra returns u64 distances,
   they're adjusted back to i64 using potentials. This subtraction could overflow.
   Need to either prove it doesn't (from Bellman-Ford bounds) or add a requires bound.

## Strategy

Read the proof body that's preserved inside the external_body. The structure is
already there — recursive split, ParaPair!, append. The fix is likely:

1. Add `obeys_feq_clone::<ArraySeqStEphS<i64>>()` and similar to requires
2. Thread feq_clone through the closure requires
3. For the overflow, check if the StEph version (`JohnsonStEphI64.rs`) has the
   same issue and how it handles it

## Read first

- `src/Chap59/JohnsonMtEphI64.rs` — your file
- `src/Chap59/JohnsonStEphI64.rs` — working St version (0 holes)
- `src/Chap57/DijkstraStEphU64.rs` — Dijkstra that Johnson calls
- `src/Chap19/ArraySeqStEph.rs` — singleton/append API and their requires

## lib.rs — do NOT modify

The file is already uncommented.

## Isolation

```bash
scripts/validate.sh isolate Chap59
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify JohnsonStEphI64.rs or any file outside Chap59.
- Do NOT add assume or accept.
- If the feq_clone threading through closures is too complex, try adding the
  preconditions to the function's requires and see if the proof goes through.
- If overflow is genuinely unprovable without bounds from BellmanFord, add a
  requires bound and document it.

## STEP 20

## Report

Write `plans/agent3-r89-johnson-report.md`.
