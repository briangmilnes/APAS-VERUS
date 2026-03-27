# Agent 2 Round 84 Report

## Objective
Remove `external_body` from `has_cycle` in `CycleDetectStEph.rs` and `CycleDetectStPer.rs`.

## Result: 2 holes closed, 0 remaining in CycleDetect files

Both `has_cycle` functions are now fully proved — no `external_body`, no `assume`, no `accept`.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|-------------|
| 1 | 55 | CycleDetectStEph.rs | 1 | 0 |
| 2 | 55 | CycleDetectStPer.rs | 1 | 0 |

## Verification Count

2128 verified (isolate Chap55), 0 errors, 0 warnings.

## Technique: Ghost DFS Finish-Time Ordering

The completeness direction (`has_cycle` returns false → graph is a DAG) required proving
that DFS explores the entire graph and that the absence of back edges implies acyclicity.

### Proof Architecture

1. **Ghost ordering**: Each vertex gets a monotonically increasing "finish time" when DFS
   completes processing it. This is threaded through `dfs_check_cycle` via ghost parameters
   `Ghost(ord): Ghost<Map<int, nat>>` and `Ghost(next_time): Ghost<nat>`.

2. **Acyclic ordering property** (`spec_acyclic_ord`): For every edge u→v where both u and v
   are "finished" (in the ordering), finish(u) > finish(v). Also includes edge closure: if u
   is finished and has edge to v, then v is finished.

3. **Vertex finish**: After processing all neighbors, vertex gets finish time `cur_next` and
   `cur_next` increments. Key sub-proofs:
   - No self-loop: if vertex→vertex existed, DFS(vertex) would detect back edge.
   - No incoming from finished: if finished vertex u→vertex existed, edge closure on cur_ord
     would require vertex in cur_ord, but vertex is an ancestor (not finished). Contradiction.
   - All neighbors visited and not ancestors: proved using DFS ensures + ancestors biconditional.

4. **DAG lemma** (`lemma_acyclic_ord_implies_dag`): If an acyclic ordering covers all vertices,
   no cycle exists. Proof: any cycle path would create a strictly decreasing sequence of nat
   values that wraps around (first vertex = last vertex), contradicting strict decrease.

### Key Technical Challenges

- **Choose trigger issues**: Verus `choose` expressions with nested quantifiers (exists inside
  choose) fail trigger inference. Solution: wrapper spec function `spec_is_valid_ord` and a
  `lemma_extract_ord` proof function that extracts existential witnesses.

- **Seq update axiom firing**: Z3 struggled with `s.update(k, v)[j] == s[j]` for `k != j`
  after DFS calls. Solution: avoided the seq update axiom entirely by using the
  ancestors↔DFS-path biconditional to derive `!old(ancestors)@[neighbor]` through
  `!spec_in_path(dfs_path, neighbor)`.

- **Visited monotonicity across calls**: Z3 lost quantified loop invariants after DFS calls.
  Solution: ghost `visited_pre_call` snapshot captured before each DFS call, then used as
  trigger term for monotonicity.

## Files Modified

- `src/Chap55/CycleDetectStEph.rs` — removed external_body, added 6 spec/proof fns + ghost logic
- `src/Chap55/CycleDetectStPer.rs` — removed external_body, added 6 spec/proof fns + ghost logic
