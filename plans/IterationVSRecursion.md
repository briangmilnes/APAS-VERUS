# Iteration vs Recursion Audit

APAS presents most algorithms recursively. Agents often convert recursion to
iteration (while loops). This audit identifies where that happened.

## Status Key

| Code | Meaning |
|------|---------|
| APASRecCodeIt | APAS recursive, code iterative (MISMATCH — main concern) |
| APASRecCodeRec | APAS recursive, code recursive (correct match) |
| APASItCodeIt | APAS iterative, code iterative (correct match) |
| APASNA | APAS has no pseudocode for this function |
| APASUnspecified | APAS doesn't clearly specify rec vs it |

## Propose Key

| Code | Meaning |
|------|---------|
| GoRec | Convert to recursion to match APAS |
| StayIt | Keep iteration (justified deviation) |
| StayRec | Keep recursion (already correct) |
| Review | Needs human review against textbook |

## Findings

### MISMATCHES: APAS Recursive, Code Iterative

| # | Chap | File | Function | Lines | Status | Propose | Notes |
|---|------|------|----------|-------|--------|---------|-------|
| 1 | 11 | FibonacciStEph.rs | fib | 106 | APASRecCodeIt | StayIt | Has fib_recursive too; iterative is perf variant |
| 2 | 18 | ArraySeqStEph.rs | reduce | ~var | APASRecCodeIt | Review | Tree reduce; sequential uses while |
| 3 | 18 | ArraySeqStPer.rs | reduce | ~var | APASRecCodeIt | Review | Same pattern, persistent |
| 4 | 18 | ArraySeqStEph.rs | flatten | ~var | APASRecCodeIt | Review | Tree flatten via while loops |
| 5 | 18 | ArraySeqStPer.rs | flatten | ~var | APASRecCodeIt | Review | Same pattern |
| 6 | 21 | Algorithm21_1.rs | flatten_inner | 103 | APASRecCodeIt | GoRec | Tree traversal, 3 nested while loops |
| 7 | 26 | DivConReduceStPer.rs | max_element | 135 | APASRecCodeIt | StayIt | Sequential fold; D&C in Mt variant |
| 8 | 41 | AVLTreeSetStEph.rs | union | 327,341 | APASRecCodeIt | GoRec | APAS: recursive tree merge |
| 9 | 41 | AVLTreeSetStEph.rs | intersection | 273 | APASRecCodeIt | GoRec | APAS: recursive tree operation |
| 10 | 41 | AVLTreeSetStEph.rs | difference | 300 | APASRecCodeIt | GoRec | APAS: recursive tree operation |
| 11 | 41 | AVLTreeSetStPer.rs | union | ~var | APASRecCodeIt | GoRec | Same as StEph |
| 12 | 41 | AVLTreeSetStPer.rs | intersection | ~var | APASRecCodeIt | GoRec | Same as StEph |
| 13 | 41 | AVLTreeSetStPer.rs | difference | ~var | APASRecCodeIt | GoRec | Same as StEph |
| 14 | 55 | DFSStPer.rs | dfs_helper | 82 | APASRecCodeIt | Review | Iterative DFS with stack |
| 15 | 55 | DFSStEph.rs | dfs_helper | 79 | APASRecCodeIt | Review | Iterative DFS with stack |
| 16 | 55 | TopoSortStPer.rs | dfs_helper | 145 | APASRecCodeIt | Review | Iterative DFS with stack |
| 17 | 55 | CycleDetectStEph.rs | has_cycle_helper | 83 | APASRecCodeIt | Review | Iterative DFS with stack |
| 18 | 55 | SCCStPer.rs | dfs_helper | 89 | APASRecCodeIt | Review | Iterative DFS with stack |
| 19 | 55 | SCCStEph.rs | dfs_helper | ~var | APASRecCodeIt | Review | Iterative DFS with stack |

### CORRECT: APAS Recursive, Code Recursive

| # | Chap | File | Function | Status | Notes |
|---|------|------|----------|--------|-------|
| 1 | 26 | MergeSortStPer.rs | merge_sort | APASRecCodeRec | Recursive D&C |
| 2 | 26 | ScanDCStPer.rs | scan_dc | APASRecCodeRec | Recursive D&C |
| 3 | 35 | OrderStatSelectStEph.rs | select_inner | APASRecCodeRec | Recursive quickselect |
| 4 | 36 | QuickSortStEph.rs | quick_sort_* | APASRecCodeRec | Recursive quicksort |
| 5 | 37 | BSTPlainStEph.rs | insert/find/delete | APASRecCodeRec | All BST ops recursive |
| 6 | 37 | BSTAVLStEph.rs | insert/find/delete | APASRecCodeRec | All BST ops recursive |
| 7 | 37 | BSTRBStEph.rs | insert/find/delete | APASRecCodeRec | All BST ops recursive |
| 8 | 37 | BSTSplayStEph.rs | insert/find/delete | APASRecCodeRec | All BST ops recursive |
| 9 | 37 | BSTBBAlphaStEph.rs | insert/find/delete | APASRecCodeRec | All BST ops recursive |
| 10 | 62 | StarContractionStEph.rs | star_contract | APASRecCodeRec | Recursive contraction |
| 11 | 63 | ConnectivityStEph.rs | count_components | APASRecCodeRec | Recursive on quotient |
| 12 | 63 | ConnectivityStEph.rs | connected_components | APASRecCodeRec | Recursive on quotient |
| 13 | 64 | SpanTreeStEph.rs | spanning_tree_* | APASRecCodeRec | Recursive via contraction |

### CORRECT: Inherently Iterative

| # | Chap | File | Function | Status | Notes |
|---|------|------|----------|--------|-------|
| 1 | 3 | InsertionSortStEph.rs | insertion_sort | APASItCodeIt | Nested loops by design |
| 2 | 47 | LinProb*.rs | find_slot | APASItCodeIt | Hash probing is iterative |
| 3 | 47 | QuadProb*.rs | find_slot | APASItCodeIt | Hash probing is iterative |
| 4 | 49 | SubsetSumStEph.rs | subset_sum | APASItCodeIt | Bottom-up DP table fill |
| 5 | 54 | BFSStEph.rs | bfs | APASItCodeIt | BFS is queue-based |
| 6 | 57 | DijkstraStEphI64.rs | dijkstra | APASItCodeIt | PQ extraction loop |
| 7 | 58 | BellmanFordStEphI64.rs | bellman_ford | APASItCodeIt | Relaxation loop |
| 8 | 65 | UnionFindStEph.rs | find | APASItCodeIt | Path compression |
| 9 | 65 | PrimStEph.rs | prim_mst | APASItCodeIt | PQ extraction loop |

### Infrastructure While Loops (Not Algorithmic)

These while loops are array manipulation between recursive calls (partitioning,
copying, concatenation). They are not algorithmic mismatches.

| # | Chap | File | Function | Notes |
|---|------|------|----------|-------|
| 1 | 26 | MergeSortStPer.rs | merge | Array merge between recursive calls |
| 2 | 36 | QuickSortStEph.rs | concat_three | Concatenate partition results |
| 3 | 40 | BSTSizeStEph.rs | find_min_priority_idx | Helper scan |
| 4 | 40 | BSTKeyValueStEph.rs | filter_by_key_kvp | Helper scan |

## Summary

| Category | Count | Action |
|----------|-------|--------|
| APASRecCodeIt (mismatch) | 19 | 5 GoRec, 2 StayIt, 12 Review |
| APASRecCodeRec (correct) | 13 | None needed |
| APASItCodeIt (correct) | 9 | None needed |
| Infrastructure | 4 | None needed |

### Priority Tiers

**Tier 1 — GoRec (clear mismatches, convert to recursion):**
- Chap41 set ops: union, intersection, difference (6 functions across StEph/StPer)
- Chap21 Algorithm21_1 flatten_inner

**Tier 2 — Review (needs textbook check):**
- Chap55 DFS family (6 functions) — iterative DFS with stack is a valid alternative
  but APAS may present recursive. Human decision needed.
- Chap18 reduce/flatten (4 functions) — sequential versions of parallel algorithms.
  APAS may present only the parallel (recursive) version.

**Tier 3 — StayIt (justified):**
- Chap11 fib (has recursive variant too)
- Chap26 max_element (sequential fold)

## Effort Estimate (4 agents)

### Tier 1: GoRec conversions (7 functions)

Converting iteration to recursion in verified code is delicate — loop invariants
become recursive specs with decreases clauses. Each conversion:
1. Rewrite function body from while to recursive calls
2. Add decreases clause
3. Adjust spec (loop invariant becomes recursive postcondition)
4. Fix all callers if signature changes
5. Validate

Estimated per function: 20-40 min (simple tree ops) to 60-90 min (complex merge).

| Agent | Files | Functions | Est Time |
|-------|-------|-----------|----------|
| 1 | Chap41/AVLTreeSetStEph.rs | union, intersection, difference | 2-3 hours |
| 2 | Chap41/AVLTreeSetStPer.rs | union, intersection, difference | 2-3 hours |
| 3 | Chap21/Algorithm21_1.rs | flatten_inner | 1-2 hours |
| 4 | (available for Tier 2 review) | DFS family or reduce/flatten | 2-3 hours |

Total for Tier 1: ~8-12 hours across 4 agents, ~3 hours wall clock.

### Tier 2: Review decisions (10 functions)

Requires human to check APAS textbook for each function. Once decided:
- Chap55 DFS (6 functions): if GoRec, ~4-6 hours total (recursive DFS is simpler
  but needs different spec structure)
- Chap18 reduce/flatten (4 functions): if GoRec, ~3-4 hours total

### Full plan (Tier 1 + Tier 2 if all GoRec): ~15-20 agent-hours, ~5 hours wall clock with 4 agents.

## Agent Prompts

See plans/iter-vs-rec-agent{1-4}-prompt.md
