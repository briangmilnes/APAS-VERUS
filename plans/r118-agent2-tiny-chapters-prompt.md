# R118 Agent 2 — Sweep tiny chapters (54-66). AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 22 warnings across 8 small chapters
(Chap54-66). Two patterns: missing wf predicates on algorithm modules,
and missing Mt implementations of St functions.

## Warnings by chapter

### Chap54 — BFS (4 warnings)

All 4 files missing `spec_*_wf`. BFS result structs hold distances/parents
arrays. Check if there's a meaningful invariant (e.g., parent indices in
bounds, distances consistent). If the struct is always valid by construction,
report that and skip.

- `BFSStPer.rs`: no spec_*_wf
- `BFSMtPer.rs`: no spec_*_wf
- `BFSStEph.rs`: no spec_*_wf
- `BFSMtEph.rs`: no spec_*_wf

### Chap55 — DFS/CycleDetect/SCC/TopoSort (8 warnings)

All missing `spec_*_wf`. Same assessment needed — are these result structs
with meaningful invariants or always-valid containers?

- `CycleDetectStPer.rs`, `CycleDetectStEph.rs`: 2 warnings
- `DFSStPer.rs`, `DFSStEph.rs`: 2 warnings
- `SCCStPer.rs`, `SCCStEph.rs`: 2 warnings
- `TopoSortStPer.rs`, `TopoSortStEph.rs`: 2 warnings

Note: these are St-only modules (no Mt variants). The wf warnings are
StPer vs StEph comparisons.

### Chap56 — PathWeightUtils (2 warnings)

- `PathWeightUtilsStPer.rs`: no spec_*_wf
- `PathWeightUtilsStEph.rs`: no spec_*_wf

St-only, utility module. Check if wf is meaningful.

### Chap61 — EdgeContraction/VertexMatching (2 warnings)

- `EdgeContractionMtEph.rs`: missing `edge_contract`, `contract_round`
- `VertexMatchingMtEph.rs`: missing `greedy_matching`, `parallel_matching_st`

These are parallel graph algorithm functions. Check if they exist as free
functions or are genuinely unimplemented. If unimplemented, document what's
needed (these are non-trivial parallel algorithms).

### Chap62 — StarContraction/StarPartition (2 warnings)

- `StarContractionMtEph.rs`: missing `star_contract`, `contract_to_vertices`
- `StarPartitionMtEph.rs`: missing `sequential_star_partition`

Same pattern as Chap61. Check and document.

### Chap63 — Connectivity (3 warnings)

- `ConnectivityMtEph.rs`: missing `count_components`, `connected_components`
- `count_components_hof` has 2 params but StEph has 1
- `connected_components_hof` has 2 params but StEph has 1

The param count mismatches may be false positives (Mt adds a lock/scheduler
parameter). Check and document.

### Chap64 — SpanTree (1 warning)

- `SpanTreeMtEph.rs`: missing `spanning_tree_star_contraction`

### Chap66 — Boruvka (1 warning)

- `BoruvkaMtEph.rs`: missing 4 fns: `vertex_bridges`, `bridge_star_partition`,
  `boruvka_mst`, `boruvka_mst_with_seed`

## Strategy

1. **Wf predicates first** (Chap54/55/56 — 14 warnings). Read each struct,
   decide if wf is meaningful. Add if yes, report "no meaningful wf" if no.
2. **Missing Mt functions** (Chap61-66 — 8 warnings). Read the StEph traits
   and MtEph files. These are likely unimplemented parallel algorithms — assess
   feasibility but don't implement complex parallel algorithms. Document.
3. Validate each chapter: `scripts/validate.sh isolate ChapNN`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- For missing wf: add real invariants only. No vacuous `requires true` or
  tautological predicates.
- For missing Mt fns: implement only if the function body is straightforward
  (lock, delegate to inner St, unlock). If the function requires non-trivial
  parallel logic, document what's needed and skip.
- No subagents.

## STEP 25

## Report

Write `plans/agent2-r118-tiny-chapters-report.md`. Include before/after
warning count per chapter.
