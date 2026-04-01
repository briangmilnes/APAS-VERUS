# R133 Agent 4 — Alg Analysis Annotations Report

## Summary

Added `/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(...), Span O(...)` 
annotations to **841 functions** across **28 chapters** and **115 files**. Every
annotation was verified by reading the function body.

## Approach

### Phase 1: Initial annotation (841 functions)

1. **Manual Edit tool** for Chap12, 17, 23, 26, 27, 28, 30, 35, 36, 44, 47, 53,
   54, 55, 57, 61-66 (327 functions) — read each function body, determined 
   complexity, added annotation.
2. **Batch sed insertion** for Chap05, 19, 38, 42, 49, 51, 56 (514 functions) —
   pattern-based annotation using function name to complexity mapping.

### Phase 2: Audit of sed-inserted annotations (514 functions)

The sed approach was wrong — it guessed complexity from function names without
reading function bodies. 10 subagents (7 audit + 3 fix-up) read every function
body and fixed the annotations.

**Problems found and fixed:**

| # | Category | Count | Examples |
|---|----------|-------|---------|
| 1 | Misplaced annotations inside function bodies | ~280 | Inside proof blocks, match arms, let bindings, loop invariants, ensures clauses |
| 2 | Wrong complexity values | ~30 | Table find O(n) not O(log n) — flat array, not BST; BST join_mid O(1) not O(log n) — parametric wraps without rebalancing |
| 3 | Wrong St Span (Span < Work, impossible for sequential) | ~20 | min_edit_distance_rec Span O(S+T) changed to O(S*T); subset_sum_rec same pattern |
| 4 | Mt functions falsely marked parallel | ~15 | med_bottom_up_parallel has no join in body; filter_dc uses D&C but sequential append dominates span |
| 5 | Annotations removed but not re-added | 201 | Fix-up subagents re-added after reading each body |

**Key wrong-complexity fixes:**

| # | Chap | Function | Was | Corrected To | Reason |
|---|------|----------|-----|-------------|--------|
| 1 | 42 | find/find_ref | O(log n) | O(n) | Flat-array table, linear scan |
| 2 | 42 | insert_wf/delete_wf | O(log n) | O(n) | Flat-array table, linear scan + rebuild |
| 3 | 38 | join_mid/join_m | O(log n) | O(1) | Parametric BST wraps node without rebalancing |
| 4 | 19 | Mt filter/map Span | O(lg n) | O(n) | Sequential append/concat_seqs dominates |
| 5 | 49 | St min_edit_distance_rec Span | O(S+T) | O(S*T) | Sequential: Span = Work |
| 6 | 49 | Mt min_edit_distance Span | O(S*T) | O(S+T) | Has join(): actually parallel |
| 7 | 56 | validate_subpath | O(V^2) | O(k) | Single-loop over path, not all triples |
| 8 | 56 | SSSP set_distance | O(n) | O(1) | Single Vec::set call |
| 9 | 05 | intersection | O(a+b) | O(a) | Only iterates self |
| 10 | 05 | is_functional_vec | O(v) | O(v^2) | Calls O(v) check for each element |
| 11 | 51 | initialize_base_cases | O(n+m) | O(n*m) | Allocates full (n+1)*(m+1) table |

## DIFFERS from APAS

Convention: only DIFFERS is called out explicitly; silence means the code review
agrees with the APAS textbook cost.

Only **2 new annotations** in this round explicitly mark "DIFFERS" from APAS. The
remaining agree with APAS or have no APAS annotation to compare against 
(scaffolding helpers, impl delegates, etc.).

## Chapters Completed

| # | Chap | Files | Functions | Notes |
|---|------|-------|-----------|-------|
| 1 | 05 | 5 | 68 | Set, Relation, Mapping, Kleene |
| 2 | 12 | 3 | 10 | SpinLock, CAS, ConcurrentStack exercises |
| 3 | 17 | 1 | 17 | MathSeq array operations |
| 4 | 19 | 4 | 90 | ArraySeq St/Mt Eph/Per + Slice |
| 5 | 23 | 2 | 24 | BalBinTree, PrimTreeSeq |
| 6 | 26 | 8 | 30 | DivConReduce, MergeSort, ScanDC, ETSP |
| 7 | 27 | 4 | 6 | ReduceContract, ScanContract |
| 8 | 28 | 10 | 10 | MaxContigSubSum (all variants) |
| 9 | 30 | 1 | 4 | Probability |
| 10 | 35 | 4 | 4 | OrderStatSelect |
| 11 | 36 | 3 | 27 | QuickSort (all 3 pivot strategies x St/Mt) |
| 12 | 38 | 2 | 60 | BSTParaSt/Mt |
| 13 | 42 | 3 | 68 | TableSt/Mt (flat-array ordered table) |
| 14 | 44 | 1 | 16 | DocumentIndex |
| 15 | 47 | 5 | 13 | Hash tables (Flat, LinkedList, Vec, Struct, Para) |
| 16 | 49 | 8 | 62 | MinEditDist, SubsetSum (St/Mt x Eph/Per) |
| 17 | 51 | 8 | 102 | TopDownDP, BottomUpDP (St/Mt x Eph/Per) |
| 18 | 53 | 5 | 38 | GraphSearch (3 variants) + PQMin (2 variants) |
| 19 | 54 | 4 | 30 | BFS (St/Mt x Eph/Per) |
| 20 | 55 | 8 | 30 | DFS, CycleDetect, TopoSort, SCC |
| 21 | 56 | 10 | 64 | SSSP/AllPairs results + PathWeightUtils |
| 22 | 57 | 1 | 6 | Stack |
| 23 | 61 | 4 | 6 | EdgeContraction, VertexMatching |
| 24 | 62 | 4 | 12 | StarContraction, StarPartition |
| 25 | 63 | 2 | 8 | Connectivity |
| 26 | 64 | 3 | 9 | SpanTree, TSPApprox |
| 27 | 65 | 3 | 15 | Kruskal, Prim, UnionFind |
| 28 | 66 | 2 | 12 | Boruvka MST |

## Verification

Final `veracity-analyze-alg-analysis` count: **0 missing** annotations in assigned
chapters.
