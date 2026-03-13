# Agent 3 — Round 6 Work Plan

## Current State

| # | Chap | File | Holes | Types |
|---|------|------|-------|-------|
| 1 | 65 | UnionFindStEph.rs | 1 | 1 assume (eq bridge) |
| 2 | 52 | EdgeSetGraphMtPer.rs | 1 | 1 external_body (filter closure) |
| 3 | 53 | GraphSearchStEph.rs | 2 | 1 assume + 1 ext_body |
| 4 | 53 | PQMinStEph.rs | 3 | 2 assume + 1 ext_body |
| 5 | 53 | GraphSearchMtPer.rs | 2 | 1 assume + 1 ext_body |
| 6 | 50 | MatrixChainMtEph.rs | 7 | 5 assume + 2 ext_body |
| 7 | 50 | OptBinSearchTreeMtEph.rs | 4 | 2 assume + 2 ext_body |
| 8 | 50 | MatrixChainMtPer.rs | 2 | 2 ext_body |
| 9 | 50 | OptBinSearchTreeMtPer.rs | 2 | 2 ext_body |
| 10 | 50 | OptBinSearchTreeStEph.rs | 1 | 1 ext_body |
| 11 | 50 | OptBinSearchTreeStPer.rs | 1 | 1 ext_body |

Plus ~12 fn_missing_spec across Chap53 files.

---

## Phase 1: Close Near-Clean Chapters (+2 clean, -2 holes)

### 1.1 Chap65/UnionFindStEph.rs — Eq Bridge (-1 hole)

**Hole**: Line 337 `assume((root_u == root_v) == (root_u@ == root_v@))`

**Strategy**: Replace raw `==` with `feq::feq(&root_u, &root_v)` which has
`ensures eq == (x@ == y@)`. The wf already requires `obeys_feq_full::<V>()`, satisfying
feq's precondition. This moves the bridge from algorithmic code to the feq utility
(where it already exists).

**Result**: UnionFindStEph → 0 holes. Chap65 clean (+1).

### 1.2 Chap52/EdgeSetGraphMtPer.rs — Filter Closure (-1 hole, uncertain)

**Hole**: Line 135 `external_body` on `out_neighbors`.

**Root cause**: `self.edges.filter(move |edge| edge.0 == u_clone)` — filter returns a
subset but Verus can't connect the runtime predicate to the spec-level set comprehension
`Set::new(|v| self.edges@.contains((u@, v)))`.

**Strategy**: Rewrite without filter. Iterate edges via `to_seq()`, manually check
`edge.0 == u` using feq, build neighbor set. The parallel split/join on indices is
preserved. If the manual iteration can establish the ensures, remove external_body.

**Risk**: High. The ensures requires exact set equality with a comprehension. Even
manual iteration may not bridge this gap without additional set-membership lemmas.
If blocked, document and move on.

---

## Phase 2: Chap53 Graph Search (-5 to -8 holes)

### 2.1 Convert graph_search_explore to Iterative (-2 ext_body)

**Files**: GraphSearchStEph.rs (line 98), GraphSearchMtPer.rs (line 104)

**Current**: Recursive function with `external_body`. No decreases clause.

**Strategy**: Convert to iterative while loop with `exec_allows_no_decreases_clause`
(same pattern as PQMinStEph.pq_explore). Add invariants:
- `visited.spec_avltreesetsteph_wf()` / `spec_avltreesetmtper_wf()`
- `initial_visited@ ⊆ visited@`
- `forall|v: &V| graph.requires((v,))`

The loop body: select → union visited → gather neighbors → compute new frontier → repeat.
This is verified code (no external_body), just without a decreasing measure.

### 2.2 Remove external_body from pq_min_multi (-1 ext_body)

**File**: PQMinStEph.rs (line 197)

**Current**: Setup loop (sources → frontier entries) then call to pq_explore.

**Strategy**: Add loop invariants to the setup while loop (i <= len, frontier wf,
sources subset tracking). pq_explore is already verified (exec_allows_no_decreases_clause).
The ensures `sources@.subset_of(search.visited@)` needs an intermediate assertion that
all source vertices enter the frontier and thus get visited.

### 2.3 Add Missing Requires Clauses

**Files**: GraphSearchStEph.rs, GraphSearchMtPer.rs, PQMinStEph.rs

All graph_search, graph_search_multi, reachable, pq_min functions need:
```rust
requires forall|v: &V| #[trigger] graph.requires((v,))
```
And PQ functions additionally need:
```rust
requires forall|v: &V| #[trigger] priority_fn.requires((v,))
```
Cascading: trait methods → free functions → internal helpers.

### 2.4 Prove Clone Assumes via feq (-2 to -4 assumes)

**Holes**:
- GraphSearchStEph.rs:73 `assume(first@ == first_ref@)`
- GraphSearchMtPer.rs:79 `assume(first@ == first_ref@)`
- PQMinStEph.rs:101 `assume(v@ == entry_ref.1@)`

**Strategy**: After `clone()`, vstd ensures `strictly_cloned(*self, result)`. With
`obeys_feq_clone::<V>()`, this implies `self.eq_spec(&result)`. With `obeys_feq_view`,
this gives `self@ == result@`.

**Requires change**: Add `obeys_feq_clone::<V>()` (or `obeys_feq_full::<V>()`) to:
- `SelectionStrategy::select` requires
- `graph_search_explore` requires
- `graph_search` / `graph_search_multi` / `reachable` requires

This cascading change is reasonable — it says "V must have well-behaved equality."

**Verification**: Test with one file first (GraphSearchStEph), then replicate.

### 2.5 Prove Neighbors wf Assume (-1 assume)

**Hole**: PQMinStEph.rs:146 `assume(neighbors.spec_avltreesetsteph_wf())`

**Strategy**: Add to graph closure spec:
```rust
requires forall|v: &V| graph.ensures((v,), result) ==> result.spec_avltreesetsteph_wf()
```
Or more practically, add `result.spec_wf()` as a known postcondition of AVLTreeSet
operations (which it should be — all AVLTreeSet constructors ensure wf).

If graph() returns an AVLTreeSetStEph built from AVLTreeSet operations, wf should hold.
The issue is that for a generic `G: Fn(&V) -> AVLTreeSetStEph<V>`, we can't assume
anything about the result without adding it to the closure spec.

**Decision**: Add to graph_search requires that graph results are wf, OR strengthen
the closure ensures. Either way, callers must provide this guarantee.

---

## Phase 3: Chap50 Lock-Boundary (target: -3 to -5 holes)

### 3.1 MatrixChainMtEph Lock-Boundary Assumes (-3 to -5 assumes)

**Holes**:
- Line 238: `assume(i < dims@.len() && k < dims@.len() && j < dims@.len())`
- Line 340: `assume(index < dims@.len())`
- Line 357: `assume(index < dims@.len())`
- Line 373: `assume(n == self@.dimensions.len())`
- Line 242: arithmetic overflow assume (harder)

**Strategy**: The RwLock invariant should capture `dims@.len()`. After `acquire_read()`,
the invariant gives us facts about the locked data. If the invariant says
`dims@.len() == self@.dimensions.len()`, then `index < self@.dimensions.len()` (from
requires) proves `index < dims@.len()`.

**Steps**:
1. Read the RwLockPredicate (MatrixChainMtEphInv) and understand what invariant it carries.
2. Strengthen invariant to include dims length if missing.
3. After acquire_read, assert the invariant fact, then prove the index bound.
4. For the arithmetic assume (line 242), check if bounds can come from the wf invariant.

### 3.2 external_body on parallel_min_reduction and matrix_chain_rec

These are genuine thread-spawn boundaries. Likely must stay as external_body.
Skip unless there's a clear path to verification.

---

## Phase 4: Skip

Chap45/BinaryHeapPQ.rs (9 holes) — multiset proofs, lowest priority. Not attempted.

---

## Execution Order

1. Phase 1.1 — Chap65 feq bridge (quick win, +1 clean chapter)
2. Phase 2.1 — graph_search_explore iterative conversion (highest impact)
3. Phase 2.3 — Add missing requires (enables 2.4)
4. Phase 2.4 — Clone assumes via feq
5. Phase 2.2 — pq_min_multi external_body removal
6. Phase 2.5 — Neighbors wf assume
7. Phase 3.1 — MatrixChainMtEph lock-boundary
8. Phase 1.2 — EdgeSetGraphMtPer (if time, high risk)

## Projected Results

| Metric | Before | After (conservative) | After (optimistic) |
|--------|--------|---------------------|-------------------|
| Holes (my files) | ~26 | 16 | 11 |
| Holes reduced | — | -10 | -15 |
| Clean chapters gained | 0 | +1 (Chap65) | +2 (Chap65, Chap52) |
