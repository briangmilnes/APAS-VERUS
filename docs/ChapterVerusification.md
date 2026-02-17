<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter Verusification State

Generated: 2026-02-13

**Totals:** 1476 verified, 2491 RTTs, 0 errors

## State Legend

| State | Meaning |
|-------|---------|
| **Verified** | All modules go through Verus verification (have specs/proofs) |
| **Partial** | Some modules Verusified, some cargo-only or gated |
| **Ported** | All modules cargo-only (from APAS-AI, not yet Verusified) |
| **Blocked** | Some/all modules behind `all_chapters` (won't compile without feature flag) |

## Foundation Modules

| # | Module | Topic | Verusified | Cargo-only | Gated | RTTs | Verified | State |
|---|--------|-------|------------|------------|-------|------|----------|-------|
| 1 | Types | Common types and traits | 1 | 0 | 0 | — | 14 | Verified |
| 2 | Concurrency | MT traits | 1 | 0 | 0 | — | 0 | Verified |
| 3 | ParaPairs | Parallel pair combinators | 1 | 0 | 0 | — | 2 | Verified |
| 4 | vstdplus | Verified std extensions | 18 | 0 | 0 | 34 | 558 | Verified |
| 5 | experiments | Experimental modules | 1 | 0 | 0 | — | 4 | Verified |

## Chapter Modules

| # | Chapter | Topic | Verusified | Cargo-only | Gated | RTTs | Verified | State |
|---|---------|-------|------------|------------|-------|------|----------|-------|
| 1 | Chap02 | HF Scheduler | 2 | 0 | 0 | 19 | 8 | Verified |
| 2 | Chap03 | Insertion Sort | 1 | 0 | 0 | 7 | 1 | Verified |
| 3 | Chap05 | Sets, Relations, Mappings, Kleene | 5 | 0 | 0 | 79 | 111 | Verified |
| 4 | Chap06 | Graphs (Dir, UnDir, Lab, Weighted) | 10 | 0 | 0 | 171 | 140 | Verified |
| 5 | Chap11 | Fibonacci (St, Mt variants) | 5 | 0 | 0 | 11 | 20 | Verified |
| 6 | Chap12 | Concurrency Exercises | 3 | 0 | 0 | 18 | 4 | Verified |
| 7 | Chap17 | MathSeq | 1 | 0 | 0 | 28 | 25 | Verified |
| 8 | Chap18 | ArraySeq, LinkedList (base) | 7 | 0 | 0 | 41 | 323 | Verified |
| 9 | Chap19 | ArraySeq (scan/filter/inject) | 3 | 0 | 1 | — | 126 | Partial |
| 10 | Chap21 | Loop Algorithms | 10 | 0 | 0 | — | 45 | Verified |
| 11 | Chap23 | Trees (PrimTreeSeq, BalBinTree) | 2 | 0 | 0 | 69 | 59 | Verified |
| 12 | Chap26 | Divide & Conquer | 8 | 0 | 0 | 54 | 28 | Verified |
| 13 | Chap27 | Reduce/Scan Contract | 4 | 0 | 0 | 30 | 6 | Verified |
| 14 | Chap28 | Max Contiguous Subsum | 0 | 5 | 3 | 32 | 0 | Blocked |
| 15 | Chap35 | Order Stat Select | 0 | 4 | 0 | 32 | 0 | Ported |
| 16 | Chap36 | QuickSort | 0 | 2 | 1 | 12 | 0 | Blocked |
| 17 | Chap37 | BSTs (Plain, AVL, RB, Splay, BBAlpha) | 0 | 8 | 11 | 230 | 0 | Blocked |
| 18 | Chap38 | Parallel BST | 0 | 2 | 0 | 21 | 0 | Ported |
| 19 | Chap39 | Treap | 0 | 4 | 0 | 99 | 0 | Ported |
| 20 | Chap40 | BST Augmented (KeyValue, Size, Reduced) | 0 | 3 | 0 | 33 | 0 | Ported |
| 21 | Chap41 | Sets (Array, AVLTree) | 0 | 6 | 1 | 113 | 0 | Blocked |
| 22 | Chap42 | Tables | 0 | 4 | 0 | 57 | 0 | Ported |
| 23 | Chap43 | Ordered Tables/Sets | 0 | 11 | 0 | 273 | 0 | Ported |
| 24 | Chap44 | Document Index | 0 | 2 | 0 | 40 | 0 | Ported |
| 25 | Chap45 | Priority Queues | 0 | 7 | 0 | 210 | 0 | Ported |
| 26 | Chap47 | Hash Tables | 0 | 9 | 0 | 90 | 0 | Ported |
| 27 | Chap49 | DP: Edit Distance, Subset Sum | 0 | 8 | 0 | 136 | 0 | Ported |
| 28 | Chap50 | DP: Matrix Chain, OBST | 0 | 9 | 0 | 169 | 0 | Ported |
| 29 | Chap51 | DP: Top-down, Bottom-up | 0 | 8 | 0 | 115 | 0 | Ported |
| 30 | Chap52 | Graph Representations | 0 | 12 | 2 | 97 | 0 | Blocked |
| 31 | Chap53 | Graph Search | 0 | 4 | 1 | 38 | 0 | Blocked |
| 32 | Chap54 | BFS | 0 | 4 | 0 | 28 | 0 | Ported |
| 33 | Chap55 | DFS, SCC, TopoSort | 0 | 8 | 0 | 61 | 0 | Ported |
| 34 | Chap56 | Shortest Path Results | 0 | 0 | 12 | — | 0 | Blocked |
| 35 | Chap57 | Dijkstra | 0 | 0 | 3 | — | 0 | Blocked |
| 36 | Chap58 | Bellman-Ford | 0 | 0 | 2 | — | 0 | Blocked |
| 37 | Chap59 | Johnson | 0 | 0 | 4 | — | 0 | Blocked |
| 38 | Chap61 | Edge Contraction | 0 | 4 | 0 | 14 | 0 | Ported |
| 39 | Chap62 | Star Contraction | 0 | 4 | 0 | 17 | 0 | Ported |
| 40 | Chap63 | Connectivity | 0 | 2 | 0 | 13 | 0 | Ported |
| 41 | Chap64 | Spanning Trees, TSP Approx | 0 | 0 | 3 | — | 0 | Blocked |
| 42 | Chap65 | Kruskal, Prim, Union-Find | 0 | 0 | 3 | — | 0 | Blocked |
| 43 | Chap66 | Boruvka MST | 0 | 0 | 2 | — | 0 | Blocked |

## Summary

| State | Chapters | Modules | Verified |
|-------|----------|---------|----------|
| Verified | 13 | 81 | 1476 |
| Partial | 1 | 3 (+1 gated) | 126 |
| Ported | 16 | 115 | 0 |
| Blocked | 13 | 34 cargo + 48 gated | 0 |
| **Total** | **43** | **282** | **1476** |

## Blocked Module Details

### OrderedFloat/View orphan rule (7 chapters, 28 modules)

Chapters 56–59, 64–66 use `OrderedFloat<f64>` which cannot implement `View` (from vstd)
due to Rust's orphan rule. All modules in these chapters are behind `#[cfg(feature = "all_chapters")]`.

### ParaPair! lifetime issues (11 modules)

Chap37 MtEph/MtPer BST variants hit E0521/E0310 lifetime errors in the `ParaPair!` macro.
Dependents in Chap41 (AVLTreeSetMtPer), Chap52 (AdjTableGraphMtPer, EdgeSetGraphMtPer),
and Chap53 (GraphSearchMtPer) are also blocked.

### Missing scan() (3 modules)

Chap28 MtEph variants require `scan()` on `ArraySeqMtEphBaseTrait`, which is not yet implemented.

### ArraySeqMtEphSlice (2 modules)

Chap19 `ArraySeqMtEphSlice` and Chap36 `QuickSortMtEphSlice` depend on it.
