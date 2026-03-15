# Review Against Prose: Chapter 65 — Sequential MST Algorithms (Prim, Kruskal, UnionFind)

Date: 2026-03-15
Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

Source: `src/Chap65/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Exec Fns (trait) | Exec Fns (impl) | Spec Fns | Holes |
|---|------|------|------------------|-----------------|----------|-------|
| 1 | 65 | KruskalStEph.rs | 3 | 0 | 1 | 0 |
| 2 | 65 | PrimStEph.rs | 2 (+1 helper) | 2 (Ord, PartialOrd) | 1 | 0 |
| 3 | 65 | UnionFindStEph.rs | 6 | 6 | 1 | 0 |

Total: 11 trait-level exec fns, 6 trait impls, 3 helper/derive fns, 3 wf spec fns, 0 holes.

## Phase 2: Prose Inventory

Source: `prompts/Chap65.txt`

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 65 | Alg 65.1: Prim's Algorithm | Algorithm |
| 2 | 65 | Alg 65.2: Kruskal's Algorithm (Union-Find) | Algorithm |
| 3 | 65 | Union-Find ADT (insert, union, find, equals) | Data structure |
| 4 | 65 | Ex 65.1: Prove correctness of Prim's | Exercise (text) |
| 5 | 65 | Ex 65.2: Prove correctness of Kruskal's | Exercise (text) |
| 6 | 65 | Cost of Prim's: O(m lg n) | Theorem |
| 7 | 65 | Cost of Kruskal's: O(m lg n) | Theorem |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions already have APAS + Claude-Opus-4.6 cost annotations.

| # | Chap | File | Function | APAS Cost | Claude Assessment |
|---|------|------|----------|-----------|-------------------|
| 1 | 65 | PrimStEph.rs | prim_mst | W O(m lg n), S O(m lg n) | W O(m^2 lg n), S O(m^2 lg n) |
| 2 | 65 | PrimStEph.rs | mst_weight | (no cost stated) | W O(MST), S O(MST) |
| 3 | 65 | PrimStEph.rs | pq_entry_new | N/A (scaffolding) | W O(1), S O(1) |
| 4 | 65 | KruskalStEph.rs | kruskal_mst | W O(m lg n), S O(m lg n) | W O(m lg m), S O(m lg m) |
| 5 | 65 | KruskalStEph.rs | mst_weight | (no cost stated) | W O(MST), S O(MST) |
| 6 | 65 | KruskalStEph.rs | verify_mst_size | (no cost stated) | W O(1), S O(1) |
| 7 | 65 | UnionFindStEph.rs | new | W O(1), S O(1) | Agrees |
| 8 | 65 | UnionFindStEph.rs | insert | W O(1), S O(1) | Agrees |
| 9 | 65 | UnionFindStEph.rs | find | W O(alpha(n)), S O(alpha(n)) | Agrees |
| 10 | 65 | UnionFindStEph.rs | union | W O(alpha(n)), S O(alpha(n)) | Agrees |
| 11 | 65 | UnionFindStEph.rs | equals | W O(alpha(n)), S O(alpha(n)) | Agrees |
| 12 | 65 | UnionFindStEph.rs | num_sets | W O(n alpha(n)), S O(n alpha(n)) | Agrees |

### 3b. Implementation Fidelity

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 65 | PrimStEph.rs | prim_mst | Faithful to Alg 65.1; uses BinaryHeapPQ. ng() returns flat edge scan (O(m) not O(degree)) |
| 2 | 65 | KruskalStEph.rs | kruskal_mst | Faithful to Alg 65.2; sorts edges, iterates with union-find |
| 3 | 65 | UnionFindStEph.rs | find | Two-pass: chase to root, then path compression. Faithful to prose |
| 4 | 65 | UnionFindStEph.rs | union | Union by rank. Faithful to prose |
| 5 | 65 | UnionFindStEph.rs | equals | Uses find + feq comparison. Faithful |

### 3c. Spec Fidelity

**UnionFindStEph.rs** has the strongest specs in these chapters:
- `new()`: ensures empty parent, rank, elements, roots maps
- `insert()`: ensures parent/rank/roots updated correctly for new element
- `find()`: ensures result is canonical root (`root@ == old(self)@.roots[v@]`), path compression preserves logical partition
- `union()`: ensures merged elements share new root, others unchanged
- `equals()`: ensures result equals (`old(self)@.roots[u@] == old(self)@.roots[v@]`)
- `num_sets()`: ensures wf preserved, roots unchanged
- `spec_unionfindsteph_wf`: comprehensive invariant (domain consistency, root fixed-point, parent domain closure, rank ordering, element coverage, no duplicates)

Spec strength for UnionFindStEph: **strong** -- full functional postconditions on all operations.

**KruskalStEph.rs** and **PrimStEph.rs**: `requires spec_wf(graph)` only. No postcondition on MST validity (edge count, spanning, minimality). The prose proves correctness via light-edge property (Lemma 64.3), but this is a text proof not encoded as specs.

Spec strength for Kruskal/Prim: **weak** -- wf preconditions only.

## Phase 4: Parallelism Review

No Mt variants exist for Chap65. This is correct per the prose: Chapter 65 is titled "Sequential MST Algorithms." Prim's and Kruskal's are inherently sequential (Prim's is priority-first search; Kruskal's processes sorted edges sequentially). The prose explicitly notes: "we first note that there is no parallelism, so the span equals the work."

## Phase 5: Runtime Test Review

| # | Chap | Test File | Coverage |
|---|------|-----------|----------|
| 1 | 65 | TestKruskalStEph.rs | kruskal_mst, mst_weight, verify_mst_size |
| 2 | 65 | TestPrimStEph.rs | prim_mst, mst_weight |
| 3 | 65 | TestUnionFindStEph.rs | new, insert, find, union, equals, num_sets |

All 3 source files have corresponding RTTs. Coverage is complete.

## Phase 6: PTT Review

No PTTs exist for Chap65. None are needed (no iterators). The UnionFindStEph specs are strong enough that PTTs could be useful for validating callability, but the current RTT coverage validates runtime correctness.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 65 | No Kruskal/Prim MST postconditions | Medium | No ensures on MST validity, edge count, or minimality |
| 2 | 65 | Prim O(m^2) due to flat edge scan | Low | ng() and get_edge_label() each O(m); prose assumes adjacency list |
| 3 | 65 | UnionFindStEph: requires_true warning on coin_flip | N/A | coin_flip is in Chap66, not Chap65 |
| 4 | 65 | mst_weight has NoSpec (no ensures) | Low | Utility function; ensures on weight sum would be nice |

## Phase 8: TOC Review

**UnionFindStEph.rs** is exemplary: clean TOC with sections 2 (imports), 4 (type definitions), 5 (view impls), 6 (spec fns), 8 (traits), 9 (impls), all inside verus!. Ghost field `roots` enables clean spec without recursive spec functions. The View type uses a ghost struct `UnionFindStEphV`.

**KruskalStEph.rs** and **PrimStEph.rs** follow the standard pattern with trait inside verus! and implementations outside (cfg(not(verus_keep_ghost))). PrimStEph has PQEntry type and View impl inside verus!; Ord/PartialOrd/Display/Debug outside. `pub type T<V>` outside verus! for each file.

## Summary

Chapter 65 implements both sequential MST algorithms (Alg 65.1 Prim's, Alg 65.2 Kruskal's) and the Union-Find data structure. All 3 modules are **clean** (0 holes). No Mt variants exist, matching the prose's "Sequential MST Algorithms" title. The standout is **UnionFindStEph.rs**, which has strong specs with full functional postconditions on all 6 operations and a comprehensive well-formedness invariant with path compression and union-by-rank correctness. This is the best-specified module in chapters 61-66. Kruskal and Prim have weak specs (wf-only preconditions). Cost annotations are present on all exec functions. RTT coverage is complete.
