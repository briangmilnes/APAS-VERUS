# Agent 2 Round 83 Report

## Objective

Remove `external_body` from 6 functions across 4 Chap55 files:
- CycleDetectStEph `has_cycle`
- CycleDetectStPer `has_cycle`
- TopoSortStEph `topological_sort_opt`, `topo_sort`
- TopoSortStPer `topological_sort_opt`, `topo_sort`

## Result

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 55 | CycleDetectStEph.rs | 1 | 1 | 0 |
| 2 | 55 | CycleDetectStPer.rs | 1 | 1 | 0 |
| 3 | 55 | TopoSortStEph.rs | 2 | 2 | 0 |
| 4 | 55 | TopoSortStPer.rs | 2 | 2 | 0 |

**Holes closed: 0.** All 6 external_body remain — the semantic postconditions
require DFS correctness proofs that are beyond Z3's reliable capacity.

## Verification

- Isolate Chap55: 2082 verified, 0 errors
- Full validate: not run (OOM risk per prompt instructions)

## Structural Improvements Made

Although no holes were closed, significant proof infrastructure was added:

### TopoSortStPer `dfs_finish_order`
- Added `visited@[vertex as int]` ensures (was missing vs StEph)
- Added `forall|k| finish_order@[k] as int < graph@.len()` requires/ensures
- Added `finish_order@.len() >= old(finish_order)@.len()` ensures
- Added to inner loop invariant accordingly

### TopoSortStPer `topo_sort` (inside external_body)
- Push-loop: added `forall|k| !visited@[k]` invariant for all-false initialization
- DFS loop: strengthened invariant from `<= n` to `== n` (sum preservation)
- DFS loop: added `forall|j| 0 <= j < start ==> visited@[j]` with explicit
  ghost monotonicity proof after each dfs_finish_order call
- Added `lemma_all_true_num_false_zero` with proper precondition proof

### TopoSortStEph `topo_sort` + `topological_sort_opt` (inside external_body)
- Named closures (`f_false`) with explicit ensures for tabulate initialization
- Added all-visited loop invariant with ghost monotonicity proof
- Both function bodies structurally ready for semantic proof

## Analysis: Why Semantic Proofs Failed

### CycleDetect `has_cycle`
**Ensures:** `has_cycle == !spec_is_dag(graph)` (biconditional)

The diagnostic showed:
- **Completeness** (`!spec_is_dag ==> has_cycle`): Z3 marks ✔ (vacuous at each exit)
- **Soundness** (`has_cycle ==> !spec_is_dag`): Z3 marks ✘

**Root cause:** `dfs_check_cycle` returns true when `ancestors[vertex]` is on the DFS
stack, but its ensures don't connect this to `!spec_is_dag(graph)`. Z3 cannot construct
the cycle witness (a path from vertex through the DFS stack back to vertex).

**What's needed:** A ghost parameter `Ghost<Seq<int>>` carrying the DFS path through
the recursive calls. When `ancestors[vertex]` is true, the cycle witness is
`dfs_path.subrange(vertex_pos, end).push(vertex)`. I implemented ~80% of this approach
(ghost parameter, base case proof with cycle construction, extended path for recursive
calls) but ran out of steps before:
- Proving the ancestors <==> path invariant before the inner loop
- Proving ancestors restoration on false return
- Resolving Z3 flakiness on the recursive call precondition

Also need `!has_cycle ==> ancestors@ =~= old(ancestors)@` in ensures (ancestors fully
restored when no cycle found) for loop invariant maintenance.

### TopoSort `topological_sort_opt` and `topo_sort`
**Ensures:** `spec_is_dag <==> topo_order.is_some()` and `spec_is_topo_order(graph, order@)`

**Z3 flakiness:** These postconditions verified in some runs but not others. In one
validate run, all 4 TopoSort functions passed. In the next (with identical code),
they failed. This is SMT solver nondeterminism.

`spec_is_topo_order` requires proving:
1. `order.no_duplicates()` — DFS visits each vertex exactly once
2. `order[k] < graph@.len()` — trackable through invariants (done)
3. `spec_has_edge(order[i], order[j]) ==> i < j` — DFS finish-order correctness

**What's needed:** A ghost finish-time tracking scheme that proves:
- Each vertex is pushed to finish_order exactly once (no_duplicates)
- For any edge u→v in a DAG, u finishes after v (finish[u] > finish[v])
- Reversing finish order gives topological order

This requires a DFS timestamp invariant carried through the recursive calls,
similar to the ghost path approach for CycleDetect.

## Techniques Attempted

1. Removed external_body from all 6 functions simultaneously
2. Named closures with explicit ensures for tabulate initialization proofs
3. Strengthened `dfs_finish_order` (StPer) with missing ensures
4. Added all-visited loop invariants with explicit ghost monotonicity proofs
5. Attempted ghost DFS path approach for CycleDetect soundness proof
6. Multiple validate runs to characterize Z3 flakiness

## Recommendation for Future Rounds

1. **CycleDetect ghost path**: Complete the ghost `Seq<int>` DFS path approach.
   ~80% implemented. Needs: ancestors <==> path invariant proof before inner loop,
   ancestors restoration ensures, and Z3 flakiness resolution. Estimated: 8-12 more
   iterations for StEph, then replicate for StPer.

2. **TopoSort DFS correctness**: Needs ghost finish-time scheme or ghost ordering
   invariant. More complex than CycleDetect because it requires proving both
   no_duplicates and edge ordering properties. Estimated: 15-20 iterations per file.

3. **Z3 stability**: The semantic proofs are on the boundary of Z3's capacity.
   Even with explicit proofs, flakiness may persist. Consider rlimit increases
   or proof decomposition into smaller lemmas.
