# R59 Agent 2 — Chap53 Capacity Cascade (12 holes)

## Assignment

Close 12 capacity assumes in Chap53 graph search and priority queue files.
All 12 are the same pattern: `assume(x.len() + y < usize::MAX as nat)` before
a `.union()` call. Replace each with a proven assertion by adding capacity
`requires` to the enclosing function and propagating to callers.

## Pattern

This is the same pattern Cursor used in Chap41/Chap43:
1. Add `requires` bounding input sizes (e.g., `visited@.len() + frontier@.len() < usize::MAX as nat`)
2. Add loop invariant maintaining the bound through iterations
3. Replace `assume(...)` with `assert(...)` — the bound flows from requires + invariant
4. Propagate new `requires` to callers (check what calls each function)

## Files and Holes

### GraphSearchStEph.rs (2 holes)

Function: `graph_search_explore` (line 117)
- Line 144: `assume(visited@.len() + frontier@.len() < usize::MAX as nat)` — before `visited.union(&frontier)`
- Line 163: `assume(new_neighbors@.len() + neighbors@.len() < usize::MAX as nat)` — before `new_neighbors.union(&neighbors)`

### GraphSearchStPer.rs (2 holes)

Function: `graph_search_explore` (line 125)
- Line 152: same pattern as StEph
- Line 170: same pattern as StEph

### PQMinStEph.rs (4 holes)

Function: `pq_explore` (line 128)
- Line 171: `assume(visited@.len() + 1 < usize::MAX as nat)` — before `visited.union(&singleton)`
- Line 193: `assume(frontier_updated@.len() + 1 < usize::MAX as nat)` — before `frontier_updated.union(&singleton)`
- Line 221: `assume(priorities@.len() + 1 < usize::MAX as nat)` — before `priorities.union(&singleton)`

Function: `pq_min_multi` (line 229)
- Line 265: `assume(initial_frontier@.len() + 1 < usize::MAX as nat)` — before `initial_frontier.union(&singleton)`

### PQMinStPer.rs (4 holes)

Function: `pq_explore` (line 125)
- Line 162, 183, 210: same pattern as PQMinStEph

Function: `pq_min_multi` (line 218)
- Line 253: same pattern as PQMinStEph

## Important Notes

- The graph functions take closures (`G: Fn(&V) -> AVLTreeSetStEph<V>`) with no
  size constraints. You will need to add a capacity bound to the outer function's
  `requires`, not to the closure contract.
- For `pq_explore`: the `+1` assumes are inside a while loop. The loop invariant
  must carry the bound. Since sets don't grow larger than the graph's vertex set,
  you can bound by the graph's vertex count if one is available, or add an explicit
  `max_vertices: Ghost<nat>` parameter.
- For `pq_min_multi`: the loop iterates over sources, adding one entry per source.
  Bound by `sources@.len()`.
- **Do not add tautological requires.** Every requires must be a real constraint.
- Check `src/Chap53/GraphSearchMtPer.rs` — if it has the same holes, fix those too.
  (It was listed as clean, so it likely doesn't.)

## Cascade Check

After adding `requires` to `graph_search_explore` and `pq_explore`, check their
callers for precondition failures:
- `graph_search`, `dfs`, `bfs` in the same files
- `pq_min`, `pq_min_multi` call `pq_explore`
- `dijkstra`, `bellman_ford` in Chap57–59 may call Chap53 functions

Run `scripts/validate.sh` after each file pair (StEph+StPer) to catch cascades early.

## Validation

Run `scripts/validate.sh` after each file pair. Show full output. Fix cascades
before moving on. Do NOT add `assume`, `accept`, or `external_body`.

## Report

Write `plans/agent2-round59-report.md` with:
- Holes before/after per file (table with # and Chap columns)
- Cascade impacts (which callers needed new requires)
- Verification count
