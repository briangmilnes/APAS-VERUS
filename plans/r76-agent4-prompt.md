# R76 Agent 4 — Chap37 BST Mt holes + Chap64 + Chap65 UnionFind (17 holes)

## Objective

Eliminate holes across 5 files in 3 chapters:
- Chap37: BSTRBMtEph (3 holes), BSTSplayMtEph (5 holes)
- Chap64: SpanTreeStEph (2 holes), TSPApproxStEph (2 holes)
- Chap65: UnionFindStEph (5 holes)

## Baseline

- 4794 verified, 0 errors, 0 warnings
- All dependencies clean for every target file

## Holes by file

### BSTRBMtEph.rs (Chap37, 3 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 796 | filter_parallel | external_body | Recursive fork-join with Arc closure |
| 2 | 821 | reduce_parallel | external_body | Recursive fork-join with Arc closure |
| 3 | 1052 | height (Mt wrapper) | assume | `link_height < usize::MAX` — needs height bound from wf |

**Strategy**: filter_parallel and reduce_parallel are genuine fork-join with `Arc<F>` closures
and thread spawning. These may stay external_body (thread spawn boundary). Focus on the
height assume (#3): the wf invariant should bound tree height. Check if
`spec_bstrbmteph_wf()` or the RwLock invariant already constrains `link_height`. If so,
the assume can be replaced with a proof step extracting the bound.

### BSTSplayMtEph.rs (Chap37, 5 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 1453 | build_balanced | external_body | Blocked by clone |
| 2 | 1479 | filter_parallel | external_body | Blocked by clone |
| 3 | 1512 | reduce_parallel | external_body | Blocked by clone |
| 4 | 1729 | height (Mt wrapper) | assume | Same as RB — height bound |
| 5 | 1801 | clone | external_body | ROOT — recursive Clone on Node<T> |

**Strategy**: The root cause is `clone` (#5). Verus has trouble with recursive `Clone` on
tree nodes with `Box<Node<T>>`. Agent 4 tried this in R75 but the recursive clone cycle
errors persist. Check the current clone body — if it's a manual recursive clone, see if
adding `decreases` helps. If the Verus limitation is real, document what you tried.

For height (#4): same approach as BSTRBMtEph — extract bound from wf.

### SpanTreeStEph.rs (Chap64, 2 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 53 | spanning_tree_star_contraction | external_body | Closure interface for star_contract |
| 2 | 149 | verify_spanning_tree | external_body | Verification helper |

**Strategy**: Read the function bodies. `spanning_tree_star_contraction` likely uses a
closure passed to `star_contract` from `BoruvkaStEph`. Check if the closure can be given
explicit `requires`/`ensures` following the closures standard. `verify_spanning_tree` is a
verification helper — should be straightforward to prove if the specs are right.

### TSPApproxStEph.rs (Chap64, 2 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 120 | euler_tour_dfs | external_body | ROOT — mutable visited set |
| 2 | 96 | euler_tour | external_body | Blocked by euler_tour_dfs |

**Strategy**: `euler_tour_dfs` uses a mutable `HashSet` for visited tracking. Check if it
can be replaced with a verified `SetStEph` or a `Vec<bool>` visited array. The DFS itself
is standard graph traversal — the proof needs a decreasing measure (unvisited vertices).

### UnionFindStEph.rs (Chap65, 5 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 219 | insert | external_body | ROOT |
| 2 | 231 | find | external_body | Blocked by insert |
| 3 | 256 | union | external_body | Blocked by find |
| 4 | 277 | equals | external_body | Blocked by find |
| 5 | 285 | num_sets | external_body | Blocked by find |

**Strategy**: `insert` is the root cause (4 downstream). Read the function body — it likely
uses `HashMap` or similar unverified type internally. Check if replacing with
`HashMapWithViewPlus` or the project's verified map type enables verification.

Agent 1 tried ClonePreservesView approach in R75 and failed — do NOT retry that approach.
Instead focus on the function bodies themselves: what unverifiable types or operations do
they use? Replace those with verified alternatives.

## Priority order

1. **UnionFindStEph** (5 holes, 1 root cause) — highest ROI if insert is fixable
2. **BSTRBMtEph height assume** — quick win if wf bounds height
3. **BSTSplayMtEph height assume** — same pattern
4. **SpanTreeStEph** (2 holes) — may be provable with closure standard
5. **TSPApproxStEph** (2 holes) — DFS rewrite
6. **BSTSplayMtEph clone** — hardest, Verus limitation
7. **filter_parallel/reduce_parallel** — likely stay external_body

## Key resources

- `src/standards/using_closures_standard.rs` — For closure patterns
- `src/standards/partial_eq_eq_clone_standard.rs` — For clone patterns
- `src/Chap65/UnionFindStEph.rs` — Read the full file
- `src/Chap64/SpanTreeStEph.rs` — Read the full file
- `src/Chap64/TSPApproxStEph.rs` — Read the full file
- `plans/agent4-round75-report.md` — Previous work on BSTRBMtEph/BSTSplayMtEph

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round76-report.md` with holes before/after (table with Chap column).
