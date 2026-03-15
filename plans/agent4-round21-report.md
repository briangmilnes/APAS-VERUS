# Agent 4 — Round 21 Report

**Date**: 2026-03-15
**Task**: Review Against Prose — DP & Graphs (Chap49–59, 61–66)
**Commit**: (pending)

## Summary

Full 8-phase review-against-prose for 17 chapters (92 source files). Wrote review
documents for all chapters and added cost annotations to 27 source files.

## Deliverables

- 17 `review-against-prose.md` files (one per chapter)
- Cost annotations (APAS + Claude-Opus-4.6 lines) added to 27 source files
- 3957 verified, 0 errors
- 238 total project holes (203 clean modules, 54 holed)

## Chapters Reviewed

| # | Chap | Topic | Files | Holes | Spec Strength | Key Finding |
|---|------|-------|:-----:|:-----:|---------------|-------------|
| 1 | 49 | MinEditDist, SubsetSum | 8 | 0 | Weak | spec_med/spec_subset_sum defined but disconnected (T→int bridge gap) |
| 2 | 50 | MatrixChain, OBST | 8 | 0 | Mixed | MatrixChain strong; OBST missing spec (f64 blocker) |
| 3 | 51 | BottomUpDP, TopDownDP | 8 | 0 | Strong | Both med fns prove result == spec_med |
| 4 | 52 | Graph Representations | 14 | 5 | Strong | 4 reps verified; 5 EdgeSet external_body (closure/filter) |
| 5 | 53 | GraphSearch, PQMin | 5 | 0 | Partial | Generic search OK; ensures source-inclusion only, not reachability |
| 6 | 54 | BFS | 4 | 0 | Partial | Genuine fork-join in Mt; missing shortest-path-distance postcond |
| 7 | 55 | DFS, CycleDetect, SCC, TopoSort | 8 | 0 | Strong | DFS spec_reachable, CycleDetect spec_is_dag, TopoSort spec_is_topo_order |
| 8 | 56 | SSSP/AllPairs Results | 10 | 0 | I64 Strong, F64 Weak | Definitional chapter; I64 fully verified, F64 weaker |
| 9 | 57 | Dijkstra, Stack | 3 | 0 | Stack Strong, Dijkstra Weak | No spec_delta_G; ensures structural only |
| 10 | 58 | BellmanFord | 2 | 0 | None | Core algorithm entirely outside verus! |
| 11 | 59 | Johnson | 4 | 0 | None | Depends on unverified BellmanFord; Mt + St outside verus! |
| 12 | 61 | EdgeContraction, VertexMatching | 4 | 0 | Weak | Partial parallelism (ParaPair! for edges) |
| 13 | 62 | StarContraction, StarPartition | 4 | 0 | Weak | StarPartitionMtEph entirely sequential despite Mt name |
| 14 | 63 | Connectivity | 2 | 0 | Weak | compose_maps sequential in Mt |
| 15 | 64 | SpanTree, TSPApprox | 3 | 0 | Weak | join() for expand phases, inner loops sequential |
| 16 | 65 | Kruskal, Prim, UnionFind | 3 | 0 | UF Strong, rest Weak | UnionFind best-verified: full functional postconditions |
| 17 | 66 | Boruvka | 2 | 0 | Partial | Only fully parallel Mt module (ParaPair! everywhere) |

## Parallelism Assessment (Mt Modules)

| # | Chap | File | Genuinely Parallel? | Notes |
|---|------|------|:-------------------:|-------|
| 1 | 49 | MinEditDistMtEph | Yes | HFScheduler fork-join |
| 2 | 49 | SubsetSumMtEph | Yes | HFScheduler fork-join |
| 3 | 50 | MatrixChainMtEph | Partial | Thread-safe memo, sequential k-loop |
| 4 | 50 | OptBinSearchTreeMtEph | Partial | Thread-safe memo, sequential k-loop |
| 5 | 51 | BottomUpDPMtEph | No | Sequential despite Mt name |
| 6 | 51 | TopDownDPMtEph | Yes | med_memoized_parallel with fork-join |
| 7 | 54 | BFSMtEph | Yes | Genuine fork-join for frontier expansion |
| 8 | 59 | JohnsonMtEphI64 | Yes | Parallel Dijkstra calls |
| 9 | 61 | EdgeContractionMtEph | Partial | ParaPair! for edge build, sequential vertex map |
| 10 | 61 | VertexMatchingMtEph | Partial | ParaPair! for edge selection, sequential coin flip |
| 11 | 62 | StarPartitionMtEph | No | Entirely sequential |
| 12 | 62 | StarContractionMtEph | Partial | ParaPair! for quotient edges |
| 13 | 63 | ConnectivityMtEph | Partial | Edge routing parallel, compose_maps sequential |
| 14 | 64 | SpanTreeMtEph | Partial | join() for expand phases |
| 15 | 66 | BoruvkaMtEph | Yes | Full ParaPair! divide-and-conquer |

## Critical Gaps Identified

1. **No spec_delta_G** — No abstract shortest-path-distance spec fn exists. Blocks correctness theorems for Dijkstra/BellmanFord/Johnson.
2. **BellmanFord outside verus!** — Core algorithm unverified (uses HashMap, not Verus-compatible).
3. **Johnson outside verus!** — Both St and Mt variants outside verus!, depends on unverified BellmanFord.
4. **OBST missing spec** — No spec_obst_cost (Probability wraps f64, no spec arithmetic).
5. **Chap49 spec disconnect** — spec_subset_sum/spec_med defined but no Seq<T>→Seq<int> bridge.
6. **StarPartitionMtEph sequential** — Cascades to ConnectivityMtEph and StarContractionMtEph.
7. **F64 graph algorithm stubs** — DijkstraF64, BellmanFordF64, JohnsonF64 are empty/stubs.

## Verification State

- 3957 verified, 0 errors
- 238 total holes (project-wide)
- 203 clean modules, 54 holed
- 2600 RTT pass

## Techniques Used

- Parallel agent execution (4 agents, one per chapter group)
- APAS prose comparison for cost annotations and spec fidelity
- Parallelism classification (genuine/partial/sequential) for all Mt modules

## Files Modified

27 source files (cost annotations), 17 new review files, 1 report file.
