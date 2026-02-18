<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Review Against Prose Summary: Chapters 36–51

**Date**: 2026-02-17  
**Reviewer**: Claude-Opus-4.6

## Aggregate Summary

| Metric | Value |
|--------|-------|
| Total chapters reviewed | 14 |
| Total source files | 96 |
| Total proof holes | 16 (15 in Chap37 + 1 in Chap47) |
| Total review TODOs | 157 |
| Chapters with Verus verification | **2** (Chap36, Chap37) |
| Chapters without any Verus code | **12** |
| Critical bugs found | 2 (Chap47 hash_index, Chap51 BU/TD metric mismatch) |

## Chapter Overview

| # | Chapter | Topic | Files | Proof Holes | Review TODOs | Verus? | Top Priority Issue |
|---|---------|-------|:-----:|:-----------:|:------------:|:------:|---|
| 1 | 36 | QuickSort | 3 | 0 | 8 | Yes | MtEph is entirely sequential; sort variants are misleading |
| 2 | 37 | BST variants | 19 | 15 | 12 | Yes | AVL/BB MtEph use std::sync::RwLock (13 avoidable holes) |
| 3 | 38 | Parametric BSTs | 2 | 0 | 8 | No | Union missing `(_, Leaf)` base case |
| 4 | 39 | Treaps | 4 | 0 | 12 | No | No Verus specs on any function |
| 5 | 40 | Augmenting BSTs | 3 | 0 | 16 | No | No Verus; range_reduce is O(n) not O(log n) |
| 6 | 41 | Sets ADT | 7 | 0 | 15 | No | AVLTreeSet uses linear scans, not AVL ops |
| 7 | 42 | Tables | 4 | 0 | 15 | No | No Verus; sorted-array gives O(n) insert |
| 8 | 43 | Ordered Tables | 11 | 0 | 14 | No | Doc claims O(lg n) on O(n) operations |
| 9 | 44 | Document Indexing | 2 | 0 | 8 | No | make_index is O(n²) not O(n log n) |
| 10 | 45 | Priority Queues | 7 | 0 | 9 | No | Quadratic rebuilds in 3 of 5 PQ types |
| 11 | 47 | Hash Tables | 9 | 1 | 9 | No | **BUG**: hash_index always returns 0 |
| 12 | 49 | DP: Subset Sum/MED | 8 | 0 | 10 | No | Weak MtEph test assertions |
| 13 | 50 | DP: Matrix Chain/OBST | 9 | 0 | 14 | No | Mt span is O(n²), not claimed O(n log n) |
| 14 | 51 | DP: Implementation | 8 | 0 | 7 | No | **BUG**: Top-down computes Levenshtein, bottom-up computes APAS MED |

## Proof Holes Detail

| # | File | Count | Types |
|---|------|:-----:|-------|
| 1 | `Chap37/BSTAVLMtEph.rs` | 8 | 1 external_type_spec, 1 assume_spec, 6 external_body |
| 2 | `Chap37/BSTBBAlphaMtEph.rs` | 5 | 5 external_body |
| 3 | `Chap37/BSTPlainMtEph.rs` | 2 | 2 assume (usize overflow in size/height) |
| 4 | `Chap47/DoubleHashFlatHashTableStEph.rs` | 1 | 1 unsafe block (FNV-1a byte read) |

**Note**: 13 of the 15 Chap37 holes are avoidable — switching BSTAVLMtEph and BSTBBAlphaMtEph from `std::sync::RwLock` to `vstd::rwlock::RwLock` (as already done for BSTPlainMtEph) would eliminate them.

## Critical Bugs

| # | Chapter | Bug | Severity | Impact |
|---|---------|-----|----------|--------|
| 1 | 47 | `hash_index` always returns 0 in all 3 chained hash table implementations | Critical | All entries go to bucket 0; tables degenerate to single linked list |
| 2 | 51 | Top-down MED includes substitution (Levenshtein distance); bottom-up does not (APAS MED) | Critical | The two variants compute different metrics; existing tests don't catch it because "tcat"/"atc" coincidentally gives 3 for both |

## Systematic Issues Across Chapters

### 1. Verification desert (12 of 14 chapters)
Only Chap36 and Chap37 have any `verus!` blocks. The remaining 12 chapters are entirely plain Rust with no specifications, no proofs, and no verification path. Proof holes report "0" vacuously.

### 2. Cost inflation
Multiple chapters have implementations whose asymptotic costs exceed what APAS specifies:

| # | Chapter | Operation | APAS Cost | Actual Cost | Root Cause |
|---|---------|-----------|-----------|-------------|------------|
| 1 | 40 | range_reduce | O(log n) | O(log n + k) | Visits all nodes in range |
| 2 | 41 | AVLTreeSet.find | O(log n) | O(n) | Linear scan through in-order traversal |
| 3 | 42 | Table.insert | O(log n) | O(n) | Sorted Vec backing structure |
| 4 | 43 | OrderedTable ordering ops | O(log n) | O(n) | Linearization through to_seq/collect |
| 5 | 44 | make_index | O(n log n) | O(n²) | Nested scan instead of Table.collect |
| 6 | 45 | BinaryHeapPQ ops | O(log n) | O(n) | Vec rebuild on every operation |
| 7 | 50 | Mt span | O(n log n) | O(n²) | Sequential subproblem evaluation |

### 3. Pseudo-parallelism in Mt modules
Several Mt modules claim parallel span but execute sequentially:

| # | Chapter | Module | Claimed Span | Actual | Issue |
|---|---------|--------|-------------|--------|-------|
| 1 | 36 | QuickSortMtEph | O(n) | O(n log n) | Entirely sequential |
| 2 | 41 | ArraySetEnumMtEph.filter | O(n/p) | O(n) | Sequential spawn-join loop |
| 3 | 42 | TableMtEph intersect/union/diff | O(lg²n) | O(n) | Sequential loops |
| 4 | 50 | All Mt *_rec | O(n log n) | O(n²) | Sequential at subproblem level |

## Review TODOs by Priority

### Critical (fix immediately)
| # | Chapter | TODO |
|---|---------|------|
| 1 | 47 | Fix `hash_index` returning 0 in all chained hash tables |
| 2 | 51 | Fix substitution deviation between top-down and bottom-up MED |
| 3 | 51 | Add cross-variant consistency test |

### High (should fix soon)
| # | Chapter | TODO |
|---|---------|------|
| 4 | 37 | Migrate BSTAVLMtEph and BSTBBAlphaMtEph to vstd::rwlock (eliminates 13 holes) |
| 5 | 36 | Fix StEph/MtEph RTTs for new verusified API |
| 6 | 38 | Fix union missing `(_, Leaf)` base case |
| 7 | 41 | Fix AVLTreeSet to use actual AVL tree operations |
| 8 | 43 | Fix doc comments claiming O(lg n) when actually O(n) |
| 9 | 43 | Fix split_key to return 3-tuple per prose |
| 10 | 47 | Fix num_elements never updated in chained tables |

### Medium (improve when working in area)
| # | Chapter | TODO |
|---|---------|------|
| 11 | 36 | Implement actual pivot strategies in verusified code |
| 12 | 37 | Add delete/split/join to verusified BST variants |
| 13 | 38 | Unify ParamBSTTrait across StEph and MtEph |
| 14 | 39 | Verusify BSTParaTreapMtEph (flagship module) |
| 15 | 40 | Verusify with BST ordering/heap invariants |
| 16 | 42 | Implement Algorithm 42.3 (collect) |
| 17 | 44 | Fix make_index to use Table.collect |
| 18 | 45 | Verusify LeftistHeapPQ |
| 19 | 49 | Strengthen MtEph MED test assertions |
| 20 | 50 | Parallelize subproblem evaluation in Mt variants |

## Individual Review Locations

| # | Chapter | Review File |
|---|---------|-------------|
| 1 | 36 | `src/Chap36/analyses/review-against-prose.md` |
| 2 | 37 | `src/Chap37/analyses/review-against-prose.md` |
| 3 | 38 | `src/Chap38/analyses/review-against-prose.md` |
| 4 | 39 | `src/Chap39/analyses/review-against-prose.md` |
| 5 | 40 | `src/Chap40/analyses/review-against-prose.md` |
| 6 | 41 | `src/Chap41/analyses/review-against-prose.md` |
| 7 | 42 | `src/Chap42/analyses/review-against-prose.md` |
| 8 | 43 | `src/Chap43/analyses/review-against-prose.md` |
| 9 | 44 | `src/Chap44/analyses/review-against-prose.md` |
| 10 | 45 | `src/Chap45/analyses/review-against-prose.md` |
| 11 | 47 | `src/Chap47/analyses/review-against-prose.md` |
| 12 | 49 | `src/Chap49/analyses/review-against-prose.md` |
| 13 | 50 | `src/Chap50/analyses/review-against-prose.md` |
| 14 | 51 | `src/Chap51/analyses/review-against-prose.md` |
