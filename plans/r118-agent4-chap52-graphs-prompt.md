# R118 Agent 4 — Strengthen Chap52 graph representation specs. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 25 warnings on Chap52. Five graph
representation files have missing functions, weak requires, and weak ensures.
You did Chap06 graphs in R117 — similar patterns here.

## Files and warning counts

| # | Chap | File | Warnings | Pattern |
|---|------|------|----------|---------|
| 1 | 52 | AdjMatrixGraphMtPer.rs | 8 | Missing 2 fns, weak has_edge/out_neighbors/out_degree requires |
| 2 | 52 | AdjMatrixGraphMtEph.rs | 4 | Weak has_edge/out_degree ensures vs MtPer |
| 3 | 52 | AdjSeqGraphStEph.rs | 2 | Missing insert_edge, delete_edge from StPer |
| 4 | 52 | AdjSeqGraphMtPer.rs | 3 | Missing from_seq, insert_edge, delete_edge |
| 5 | 52 | AdjSeqGraphMtEph.rs | 2 | Missing from_seq, set_neighbors from StEph |
| 6 | 52 | AdjTableGraphStEph.rs | 3 | Weak vertices/has_edge/out_neighbors ensures vs StPer |
| 7 | 52 | EdgeSetGraphMtPer.rs | 3 | Missing spec_out_neighbors, weak insert requires |

## Warnings detail

### AdjMatrixGraphMtPer.rs (8 warnings)

Missing functions:
- `from_matrix` — constructor from raw matrix
- `set_edge` — mutate edge

Missing requires (bounds checks from StPer):
- `has_edge`: missing `u < self.spec_n()`, `v < self.spec_n()`
- `out_neighbors`: missing `u < self.spec_n()`
- `out_degree`: missing `u < self.spec_n()`

### AdjMatrixGraphMtEph.rs (4 warnings)

Weak ensures vs MtPer:
- `has_edge`: missing `(u >= spec_n() || v >= spec_n()) ==> !found`
- `out_degree`: missing `u >= spec_n() ==> d == 0`

### AdjSeqGraphStEph.rs (2 warnings)

Missing `insert_edge`, `delete_edge` from StPer. These are mutation
operations. StEph should have them (Eph supports mutation). Check if they
exist under different names or need implementing.

### AdjSeqGraphMtPer.rs (3 warnings)

Missing `from_seq`, `insert_edge`, `delete_edge`. Same pattern — check
if the inner St has them and the Mt wrapper just needs delegation.

### AdjSeqGraphMtEph.rs (2 warnings)

Missing `from_seq`, `set_neighbors` from StEph. Check and implement or
document.

### AdjTableGraphStEph.rs (3 warnings)

StPer has stronger ensures on:
- `vertices`: StPer has ensures, StEph doesn't
- `has_edge`: StPer has ensures, StEph doesn't
- `out_neighbors`: StPer has ensures, StEph doesn't

Check what StPer's ensures clauses are and add to StEph.

### EdgeSetGraphMtPer.rs (3 warnings)

- Missing `spec_out_neighbors` spec fn
- `insert_vertex`: missing `spec_vertices().len() + 1 < usize::MAX` requires
- `insert_edge`: missing 2 capacity requires from StPer

## Strategy

1. Read all files (St variants for reference, then Mt variants).
2. Start with AdjTableGraphStEph (3 warnings, StEph strengthening — easy).
3. AdjMatrixGraphMtPer requires (3 warnings, mechanical bounds checks).
4. AdjMatrixGraphMtEph ensures (4 warnings, same pattern).
5. Missing functions: assess each. Implement if straightforward
   (lock-delegate-unlock or copy from St). Document if complex.
6. EdgeSetGraphMtPer (3 warnings, spec fn + requires).
7. Validate: `scripts/validate.sh isolate Chap52`.
8. RTT: `scripts/rtt.sh Chap52`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from St counterparts.
- Adding requires may break callers in Chap53+ — check and fix.
- No subagents.

## STEP 25

## Report

Write `plans/agent4-r118-chap52-graphs-report.md`. Include before/after
warning count per file.
