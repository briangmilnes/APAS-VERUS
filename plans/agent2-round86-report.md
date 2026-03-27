---
round: 86
agent: 2
---

# Agent 2 — Round 86 Report

## Objective

Remove `external_body` from `topological_sort_opt` in TopoSortStEph.rs and
TopoSortStPer.rs (2 holes).

## Approach

Delegated `topological_sort_opt` to the already-proved `has_cycle` (CycleDetect)
and `topo_sort` (TopoSort) functions:

```rust
if CycleDetectStEph::has_cycle(graph) {
    None   // has_cycle == !spec_is_dag(graph)
} else {
    Some(TopoSortStEph::topo_sort(graph))
    // spec_is_dag(graph) ==> spec_is_topo_order(graph, order@)
}
```

This satisfies both ensures:
- `is_some() <==> spec_is_dag(graph)` — from `has_cycle` ensures
- `is_some() ==> spec_is_topo_order(...)` — from `topo_sort` ensures under DAG

For StPer, bridged `spec_toposortstper_wf` to `spec_cycledetectstper_wf` (identical
open spec fn bodies, Z3 unfolds both automatically).

## Results

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|-------------|
| 1 | 55 | TopoSortStEph.rs | 1 | 0 |
| 2 | 55 | TopoSortStPer.rs | 1 | 0 |

## Chap55 Status

All 8 modules clean, 0 holes, 48 proof functions verified.

## Verification

```
scripts/validate.sh isolate Chap55
verification results:: 2145 verified, 0 errors
```

## Techniques

- Proof by delegation: reuse already-proved infrastructure rather than
  re-proving the same properties through ghost parameter threading.
- Open spec fn equivalence bridging (StPer wf predicates).

## Steps Used

3 of 20 (edit StEph, edit StPer, validate).
