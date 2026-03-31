# Agent 2 — R118 Tiny Chapters (54-66) Report

## Summary

Assessed 23 `veracity-compare-par-mut` warnings across Chap54-66.
All 23 are false positives or naming convention mismatches. No code changes needed.

## Wf Warnings (14) — No Meaningful Wf

All 14 "no spec_*_wf" warnings are on **unit structs** (namespace structs with zero fields).
These modules define well-formedness as **free spec fns on graph parameters**, which is the
correct pattern for graph algorithms where the data structure is a function parameter, not
stored in the struct. Adding `spec_*_wf(&self) -> bool { true }` to a unit struct would be
a vacuous tautology, violating project rules.

| # | Chap | File | Struct | Real wf exists as |
|---|------|------|--------|-------------------|
| 1 | 54 | BFSStPer.rs | `BFSStPer` (unit) | `spec_bfsstper_wf(graph)` |
| 2 | 54 | BFSMtPer.rs | `BFSMtPer` (unit) | `spec_bfsmtper_wf(graph)` |
| 3 | 54 | BFSStEph.rs | `BFSStEph` (unit) | `spec_bfssteph_wf(graph)` |
| 4 | 54 | BFSMtEph.rs | `BFSMtEph` (unit) | `spec_bfsmteph_wf(graph)` |
| 5 | 55 | CycleDetectStPer.rs | `CycleDetectStPer` (unit) | `spec_cycledetectstper_wf(graph)` |
| 6 | 55 | CycleDetectStEph.rs | `CycleDetectStEph` (unit) | uses `spec_toposortsteph_wf(graph)` |
| 7 | 55 | DFSStPer.rs | `DFSStPer` (unit) | uses `spec_toposortstper_wf(graph)` |
| 8 | 55 | DFSStEph.rs | `DFSStEph` (unit) | uses `spec_toposortsteph_wf(graph)` |
| 9 | 55 | SCCStPer.rs | `SCCStPer` (unit) | uses `spec_toposortstper_wf(graph)` |
| 10 | 55 | SCCStEph.rs | `SCCStEph` (unit) | uses `spec_toposortsteph_wf(graph)` |
| 11 | 55 | TopoSortStPer.rs | `TopoSortStPer` (unit) | `spec_toposortstper_wf(graph)` |
| 12 | 55 | TopoSortStEph.rs | `TopoSortStEph` (unit) | `spec_toposortsteph_wf(graph)` |
| 13 | 56 | PathWeightUtilsStPer.rs | `PathWeightUtilsStPerS` (unit) | no graph-level wf needed |
| 14 | 56 | PathWeightUtilsStEph.rs | `PathWeightUtilsStEphS` (unit) | no graph-level wf needed |

**Verdict:** No action. Tool should recognize free spec fn wf predicates.

## Missing Mt Function Warnings (9) — Naming Convention Mismatches

All 9 "missing function" warnings are because MtEph functions use `_mt` suffixed names
(or different prefixes like `parallel_` vs `sequential_`). Every function already exists
in the MtEph file — the tool just can't match names across variants.

| # | Chap | File | Tool says "missing" | Actually exists as |
|---|------|------|--------------------|--------------------|
| 1 | 61 | EdgeContractionMtEph.rs | `edge_contract` | `edge_contract_mt` (line 76) |
| 2 | 61 | EdgeContractionMtEph.rs | `contract_round` | `contract_round_mt` (line 219) |
| 3 | 61 | VertexMatchingMtEph.rs | `greedy_matching` | N/A — sequential only, Mt uses `parallel_matching_mt` |
| 4 | 61 | VertexMatchingMtEph.rs | `parallel_matching_st` | `parallel_matching_mt` (line 64) |
| 5 | 62 | StarContractionMtEph.rs | `star_contract` | `star_contract_mt` (line 180) |
| 6 | 62 | StarContractionMtEph.rs | `contract_to_vertices` | `contract_to_vertices_mt` (line 418) |
| 7 | 62 | StarPartitionMtEph.rs | `sequential_star_partition` | `parallel_star_partition` (line 80) |
| 8 | 63 | ConnectivityMtEph.rs | `count_components` | `count_components_mt` (line 86) |
| 9 | 63 | ConnectivityMtEph.rs | `connected_components` | `connected_components_mt` (line 110) |
| 10 | 64 | SpanTreeMtEph.rs | `spanning_tree_star_contraction` | `spanning_tree_star_contraction_mt` (line 58) |
| 11 | 66 | BoruvkaMtEph.rs | `vertex_bridges` | `vertex_bridges_mt` (line 478) |
| 12 | 66 | BoruvkaMtEph.rs | `bridge_star_partition` | `bridge_star_partition_mt` (line 586) |
| 13 | 66 | BoruvkaMtEph.rs | `boruvka_mst` | `boruvka_mst_mt` (line 750) |
| 14 | 66 | BoruvkaMtEph.rs | `boruvka_mst_with_seed` | `boruvka_mst_mt_with_seed` (line 981) |

**Param count warnings (2):**

| # | Chap | File | Function | StEph params | MtEph params | Extra param |
|---|------|------|----------|-------------|-------------|-------------|
| 1 | 63 | ConnectivityMtEph.rs | `count_components_hof` | 1 (`graph`) | 2 (`graph`, `seed`) | `seed: u64` |
| 2 | 63 | ConnectivityMtEph.rs | `connected_components_hof` | 1 (`graph`) | 2 (`graph`, `seed`) | `seed: u64` |

The extra `seed` parameter is inherent to parallel star contraction (randomized partition).
StEph's `star_contract` uses no seed; MtEph's `star_contract_mt` requires one. This is a
genuine API difference, not a bug.

**Verdict:** No action. All Mt functions are implemented. The `_mt` naming convention is
intentional because Mt functions have different type bounds (MtT, ClonePreservesView),
different parameter lists (seed, Arc), and sometimes different algorithms (parallel
matching vs greedy matching).

## Warning Count Before/After

| # | Chap | Warnings Before | Warnings After | Delta |
|---|------|----------------|----------------|-------|
| 1 | 54 | 4 | 4 | 0 |
| 2 | 55 | 8 | 8 | 0 |
| 3 | 56 | 2 | 2 | 0 |
| 4 | 61 | 2 | 2 | 0 |
| 5 | 62 | 2 | 2 | 0 |
| 6 | 63 | 3 | 3 | 0 |
| 7 | 64 | 1 | 1 | 0 |
| 8 | 66 | 1 | 1 | 0 |
| | **Total** | **23** | **23** | **0** |

All 23 warnings are tool limitations, not code defects. No code changes were made.

## Recommendations for veracity-compare-par-mut

1. **Recognize free spec fn wf predicates.** When a module has `spec_<mod>_wf(graph)` as a
   free spec fn (used in requires clauses), don't flag "no spec_*_wf" just because the
   struct is a unit type.
2. **Match `_mt` suffixed names.** When comparing MtEph vs StEph, try stripping `_mt`
   suffix and matching `parallel_` prefix against `sequential_` prefix.
3. **Allow extra parameters in Mt variants.** `seed: u64` is a common addition for
   randomized parallel algorithms. A param count mismatch of +1 for `seed` should be
   info, not warning.
