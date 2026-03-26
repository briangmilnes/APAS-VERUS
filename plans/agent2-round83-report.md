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

**Holes closed: 0.** All 6 `external_body` on the target functions remain.
However, significant proof infrastructure was built and verified.

## Verification

- Isolate Chap55: 2084 verified, 0 errors
- Full validate: not run (OOM risk per prompt instructions)

## Major Accomplishment: Ghost DFS Path Proofs

Both `dfs_check_cycle` functions (StEph and StPer) now carry a ghost DFS path
parameter and have two new proved ensures:

1. **Soundness**: `has_cycle ==> !spec_is_dag(graph)` — if the DFS finds a back
   edge, a cycle provably exists. Proved via ghost cycle witness construction
   using `dfs_path.subrange(i, end).push(vertex)`.

2. **Ancestors restoration**: `!has_cycle ==> ancestors@ =~= old(ancestors)@` — if
   no cycle is found, the ancestors array is fully restored to its pre-call state.

### Ghost Path Approach

- `Ghost<Seq<int>>` parameter tracks the DFS call stack
- Requires: path vertices are valid, consecutive edges exist, last element connects
  to current vertex, ancestors biconditional with path membership
- Base case (ancestors[vertex] true): `spec_in_path(dfs_path, vertex)` → invoke
  `lemma_cycle_not_dag` to construct cycle witness
- Recursive case: extends path with current vertex, propagates `!spec_is_dag`
- Loop invariant: `ext_path =~= dfs_path.push(vertex)`, ancestors <==> ext_path
- StEph: needed pre-set/post-set spec_index bridging for ArraySeqStEphS
- StPer: simpler bridging with Vec<bool> direct `@.update()` semantics

## Structural Improvements

### TopoSortStPer `dfs_finish_order`
- Added `visited@[vertex as int]` ensures
- Added `forall|k| finish_order@[k] as int < graph@.len()` requires/ensures
- Added `finish_order@.len() >= old(finish_order)@.len()` ensures

### TopoSort initialization and loop invariants (both files, inside external_body)
- Named closures for tabulate initialization proofs
- All-visited DFS loop invariants with explicit ghost monotonicity proofs
- Sum invariant strengthened to equality

## What Remains

### CycleDetect `has_cycle` (both files) — 1 hole each
**Soundness (proved)**: `return true ==> !spec_is_dag` — via ghost path in `dfs_check_cycle`.
**Completeness (not proved)**: `return false ==> spec_is_dag` — needs DFS completeness:
"if all vertices explored without finding a back edge, the graph is a DAG."

Approaches for future rounds:
- Add visited-set tracking to `dfs_check_cycle` ensures (e.g., "all vertices reachable
  from vertex through unvisited vertices are now visited")
- Prove that complete DFS exploration with no back edges implies no cycles

### TopoSort (both files) — 2 holes each
**Semantic postconditions**: `spec_is_dag <==> topo_order.is_some()`,
`spec_is_topo_order(graph, order@)`.

Z3 proved these in some validation runs but not others (nondeterminism).
Real proofs need:
- DFS finish-order correctness (no_duplicates + edge ordering in reverse finish order)
- Ghost finish-time tracking through recursive calls

## Techniques Used

1. Ghost `Seq<int>` DFS path parameter for cycle witness construction
2. `spec_in_path` helper and `lemma_cycle_not_dag` proof lemma
3. Ancestors <==> path biconditional with `choose` existential extraction
4. Pre-set/post-set spec_index bridging for ArraySeqStEphS
5. Vec<bool> Seq::update chain for StPer ancestors restoration
6. Named closures with explicit ensures for tabulate initialization
7. Ghost monotonicity proofs for visited array tracking
