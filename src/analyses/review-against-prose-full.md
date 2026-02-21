<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Full Review Against Prose — All Chapters

**Date**: 2026-02-21  
**Reviewer**: Claude-Opus-4.6  
**Scope**: All 43 chapters with reviews (Chapters 02–66)

## Executive Summary

| Metric | Value |
|--------|-------|
| Chapters with reviews | 43 |
| Chapters stale (inputs changed) | 43 |
| Chapters up to date | 0 |
| Total modules (proof-holes) | 399 |
| Clean modules (0 holes) | 278 |
| Holed modules | 121 |
| Total proof holes | 757 |
| Verus verification | 2584 verified, 0 errors |

## Stale Status

All 43 chapter reviews are **stale** — at least one input (source file, prose prompt, proof-holes log, or test file) is newer than the review. Regenerate per chapter when working in that area.

## Chapter Overview

| # | Chapter | Clean | Holed | Stale | Prose | Top Priority |
|---|---------|:-----:|:-----:|:-----:|:-----:|---|
| 1 | 02 | 1 | 1 | ✓ | ✓ | HFScheduler external_body |
| 2 | 03 | 1 | 0 | ✓ | ✓ | InsertionSort verified |
| 3 | 05 | 2 | 3 | ✓ | ✓ | SetStEph/MappingStEph holes |
| 4 | 06 | 20 | 0 | ✓ | ✓ | All verified |
| 5 | 11 | 2 | 3 | ✓ | ✓ | FibonacciMtEph holes |
| 6 | 12 | 1 | 2 | ✓ | ✓ | Exercise12_1/12_5 holes |
| 7 | 17 | 0 | 1 | ✓ | ✓ | MathSeq holes |
| 8 | 18 | 0 | 7 | ✓ | ✓ | ArraySeq/LinkedList deps |
| 9 | 19 | 1 | 3 | ✓ | ✓ | ArraySeqMtEph holes |
| 10 | 21 | 12 | 0 | ✓ | ✓ | All verified |
| 11 | 23 | 0 | 2 | ✓ | ✓ | BalBinTreeStEph holes |
| 12 | 26 | 6 | 2 | ✓ | ✓ | ETSPMtEph/StEph holes |
| 13 | 27 | 4 | 0 | ✓ | ✓ | All verified |
| 14 | 28 | 11 | 0 | ✓ | ✓ | All verified |
| 15 | 35 | 2 | 2 | ✓ | ✓ | OrderStatMt holes |
| 16 | 36 | 2 | 1 | ✓ | ✓ | QuickSortMtEphSlice hole |
| 17 | 37 | 0 | 1 | ✓ | ✓ | AVLTreeSeq holed chain |
| 18 | 38 | 0 | 2 | ✓ | ✓ | BSTParaStEph holes |
| 19 | 39 | 0 | 3 | ✓ | ✓ | Treap holed (rand) |
| 20 | 40 | 0 | 3 | ✓ | ✓ | BSTKeyValue holes |
| 21 | 41 | 0 | 2 | ✓ | ✓ | AVLTreeSet holed |
| 22 | 42 | 1 | 3 | ✓ | ✓ | Table holed |
| 23 | 43 | 1 | 4 | ✓ | ✓ | OrderedTable holed |
| 24 | 44 | 0 | 2 | ✓ | ✓ | DocumentIndex holes |
| 25 | 45 | 1 | 6 | ✓ | ✓ | PQ holed |
| 26 | 47 | 4 | 5 | ✓ | ✓ | **BUG** hash_index |
| 27 | 49 | 0 | 5 | ✓ | ✓ | DP MED/SubsetSum holes |
| 28 | 50 | 0 | 9 | ✓ | ✓ | MatrixChain/OBST holes |
| 29 | 51 | 2 | 6 | ✓ | ✓ | **BUG** MED metric |
| 30 | 52 | 12 | 2 | ✓ | ✓ | AdjTable/EdgeSet holes |
| 31 | 53 | 0 | 5 | ✓ | ✓ | GraphSearch holed |
| 32 | 54 | 4 | 0 | ✓ | ✓ | All verified |
| 33 | 55 | 8 | 0 | ✓ | ✓ | All verified |
| 34 | 56 | 8 | 4 | ✓ | ✓ | PathWeightUtils holes |
| 35 | 57 | 1 | 2 | ✓ | ✓ | Dijkstra holes |
| 36 | 58 | 2 | 0 | ✓ | ✓ | All verified |
| 37 | 59 | 4 | 0 | ✓ | ✓ | All verified |
| 38 | 61 | 4 | 0 | ✓ | ✓ | All verified |
| 39 | 62 | 4 | 0 | ✓ | ✓ | All verified |
| 40 | 63 | 2 | 0 | ✓ | ✓ | All verified |
| 41 | 64 | 2 | 1 | ✓ | ✓ | SpanTreeMtEph hole |
| 42 | 65 | 1 | 2 | ✓ | ✓ | Prim/UnionFind holes |
| 43 | 66 | 0 | 2 | ✓ | ✓ | Boruvka holed (rand) |

## Fully Verified Chapters (0 Holed Modules)

| # | Chapter | Modules |
|---|---------|---------|
| 1 | 03 | InsertionSortStEph |
| 2 | 06 | 20 graph modules |
| 3 | 21 | 12 Algorithm/Exercise/Problem |
| 4 | 27 | 4 Reduce/Scan contract |
| 5 | 28 | 11 MaxContigSubSum |
| 6 | 54 | 4 BFS |
| 7 | 55 | 8 Cycle/DFS/SCC/Topo |
| 8 | 58 | 2 BellmanFord |
| 9 | 59 | 4 Johnson |
| 10 | 61 | 4 EdgeContraction/VertexMatching |
| 11 | 62 | 4 StarContraction/Partition |
| 12 | 63 | 2 Connectivity |

## Critical Bugs (from Chap36–51 Summary)

| # | Chapter | Bug | Severity |
|---|---------|-----|----------|
| 1 | 47 | `hash_index` always returns 0 in chained hash tables | Critical |
| 2 | 51 | Top-down MED (Levenshtein) ≠ bottom-up (APAS MED) | Critical |

## Cost Inflation (APAS vs Actual)

| # | Chapter | Operation | APAS | Actual | Cause |
|---|---------|-----------|------|--------|-------|
| 1 | 40 | range_reduce | O(log n) | O(log n + k) | Visits all nodes in range |
| 2 | 41 | AVLTreeSet.find | O(log n) | O(n) | Linear scan |
| 3 | 42 | Table.insert | O(log n) | O(n) | Sorted Vec backing |
| 4 | 43 | OrderedTable ops | O(log n) | O(n) | Linearization |
| 5 | 44 | make_index | O(n log n) | O(n²) | Nested scan |
| 6 | 45 | BinaryHeapPQ ops | O(log n) | O(n) | Vec rebuild |
| 7 | 50 | Mt span | O(n log n) | O(n²) | Sequential subproblems |

## Pseudo-Parallelism (Mt Span ≠ Claimed)

| # | Chapter | Module | Claimed Span | Actual |
|---|---------|--------|-------------|--------|
| 1 | 36 | QuickSortMtEph | O(n) | O(n log n) |
| 2 | 41 | ArraySetEnumMtEph.filter | O(n/p) | O(n) |
| 3 | 42 | TableMtEph intersect/union/diff | O(lg²n) | O(n) |
| 4 | 50 | All Mt *_rec | O(n log n) | O(n²) |

## Proof Holes by Type

| Type | Count |
|------|------|
| external_body | 459 |
| assume() | 209 |
| admit() | 48 |
| assume_specification | 10 |
| external_type_specification | 10 |
| assume(false) | 8 |
| unsafe {} | 4 |
| unsafe impl | 2 |
| external_trait_specification | 2 |
| external_trait_extension | 1 |
| external | 3 |
| Tracked::assume_new() | 1 |

## Individual Review Locations

| # | Chapter | Review File |
|---|---------|-------------|
| 1 | 02 | `src/Chap02/analyses/review-against-prose.md` |
| 2 | 03 | `src/Chap03/analyses/review-against-prose.md` |
| 3 | 05 | `src/Chap05/analyses/review-against-prose.md` |
| 4 | 06 | `src/Chap06/analyses/review-against-prose.md` |
| 5 | 11 | `src/Chap11/analyses/review-against-prose.md` |
| 6 | 12 | `src/Chap12/analyses/review-against-prose.md` |
| 7 | 17 | `src/Chap17/analyses/review-against-prose.md` |
| 8 | 18 | `src/Chap18/analyses/review-against-prose.md` |
| 9 | 19 | `src/Chap19/analyses/review-against-prose.md` |
| 10 | 21 | `src/Chap21/analyses/review-against-prose.md` |
| 11 | 23 | `src/Chap23/analyses/review-against-prose.md` |
| 12 | 26 | `src/Chap26/analyses/review-against-prose.md` |
| 13 | 27 | `src/Chap27/analyses/review-against-prose.md` |
| 14 | 28 | `src/Chap28/analyses/review-against-prose.md` |
| 15 | 35 | `src/Chap35/analyses/review-against-prose.md` |
| 16 | 36 | `src/Chap36/analyses/review-against-prose.md` |
| 17 | 37 | `src/Chap37/analyses/review-against-prose.md` |
| 18 | 38 | `src/Chap38/analyses/review-against-prose.md` |
| 19 | 39 | `src/Chap39/analyses/review-against-prose.md` |
| 20 | 40 | `src/Chap40/analyses/review-against-prose.md` |
| 21 | 41 | `src/Chap41/analyses/review-against-prose.md` |
| 22 | 42 | `src/Chap42/analyses/review-against-prose.md` |
| 23 | 43 | `src/Chap43/analyses/review-against-prose.md` |
| 24 | 44 | `src/Chap44/analyses/review-against-prose.md` |
| 25 | 45 | `src/Chap45/analyses/review-against-prose.md` |
| 26 | 47 | `src/Chap47/analyses/review-against-prose.md` |
| 27 | 49 | `src/Chap49/analyses/review-against-prose.md` |
| 28 | 50 | `src/Chap50/analyses/review-against-prose.md` |
| 29 | 51 | `src/Chap51/analyses/review-against-prose.md` |
| 30 | 52 | `src/Chap52/analyses/review-against-prose.md` |
| 31 | 53 | `src/Chap53/analyses/review-against-prose.md` |
| 32 | 54 | `src/Chap54/analyses/review-against-prose.md` |
| 33 | 55 | `src/Chap55/analyses/review-against-prose.md` |
| 34 | 56 | `src/Chap56/analyses/review-against-prose.md` |
| 35 | 57 | `src/Chap57/analyses/review-against-prose.md` |
| 36 | 58 | `src/Chap58/analyses/review-against-prose.md` |
| 37 | 59 | `src/Chap59/analyses/review-against-prose.md` |
| 38 | 61 | `src/Chap61/analyses/review-against-prose.md` |
| 39 | 62 | `src/Chap62/analyses/review-against-prose.md` |
| 40 | 63 | `src/Chap63/analyses/review-against-prose.md` |
| 41 | 64 | `src/Chap64/analyses/review-against-prose.md` |
| 42 | 65 | `src/Chap65/analyses/review-against-prose.md` |
| 43 | 66 | `src/Chap66/analyses/review-against-prose.md` |

## Regeneration Procedure

To regenerate a chapter's review when stale:

```bash
# 1. Regenerate proof holes
~/projects/veracity/target/release/veracity-review-proof-holes -d src/ChapNN/

# 2. Regenerate function inventory
~/projects/veracity/target/release/veracity-review-module-fn-impls -d src/ChapNN

# 3. Classify spec strengths (optional per classify-spec-strengths rule)

# 4. Update src/ChapNN/analyses/review-against-prose.md per Phase 1–8
```

## Recent Changes (Chap52)

- **AdjTableGraphMtPer::delete_vertex** and **EdgeSetGraphMtPer::out_neighbors** now use HFScheduler `join()` instead of raw `thread::spawn` — preserves APAS parallelism and cost specs. Update Chap52 review: these two functions now have true thread-based parallelism via HFScheduler.
- **OrderedFloat → F64Dist** migration complete in Chap64 (TSPApproxStEph), Chap65 (Prim, Kruskal), Chap66 (BoruvkaStEph, BoruvkaMtEph). Boruvka remains commented out (uses rand).
