# R43 Agent 1: Chap61 + Chap62 Graph Algorithms (17 holes)

## Baseline

- 4362 verified, 0 errors, 139 holes
- 34 clean chapters
- Your branch: `agent1/ready`, worktree at `.claude/worktrees/agent1/` (or equivalent)

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken `ensures`.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert `assume()` to `accept()`.**
**DO NOT add `external_body` to solve a failing proof. Ask the user first.**
**DO NOT add `assume(...)` outside the four permitted patterns. Ask the user first.**

Read `/home/milnes/projects/APAS-VERUS/CLAUDE.md` before starting.
Read `/home/milnes/projects/APAS-VERUS/src/standards/mod_standard.rs` before starting.

## Assignment

Prove 17 external_body holes across Chap61 and Chap62 graph algorithm files. All 17
are `#[verifier::external_body] #[cfg(not(verus_keep_ghost))]` functions that already
have correct specs in their trait. Your job is to remove `external_body` and write real
proof-carrying bodies with ghost state tracking.

## File Inventory

| # | Chap | File | Holes | Functions |
|---|------|------|-------|-----------|
| 1 | 61 | EdgeContractionStEph.rs | 2 | `edge_contract`, `contract_round` |
| 2 | 61 | VertexMatchingStEph.rs | 2 | `greedy_matching`, `parallel_matching_st` |
| 3 | 61 | EdgeContractionMtEph.rs | 2 | `edge_contract_mt`, `contract_round_mt` |
| 4 | 61 | VertexMatchingMtEph.rs | 2 | `parallel_matching_mt`, `select_edges_parallel` |
| 5 | 62 | StarPartitionStEph.rs | 1 | `sequential_star_partition` |
| 6 | 62 | StarContractionStEph.rs | 3 | `star_contract`, `build_quotient_graph`, `contract_to_vertices` |
| 7 | 62 | StarPartitionMtEph.rs | 1 | `parallel_star_partition` |
| 8 | 62 | StarContractionMtEph.rs | 4 | `star_contract_mt`, `build_quotient_graph_parallel`, `route_edges_parallel`, `contract_to_vertices_mt` |

Full paths:
- `/home/milnes/projects/APAS-VERUS/src/Chap61/EdgeContractionStEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap61/VertexMatchingStEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap61/EdgeContractionMtEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap61/VertexMatchingMtEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap62/StarPartitionStEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap62/StarContractionStEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap62/StarPartitionMtEph.rs`
- `/home/milnes/projects/APAS-VERUS/src/Chap62/StarContractionMtEph.rs`

## What the Functions Do

### Chap61: Vertex Matching

**VertexMatchingStEph.rs** implements greedy and randomized vertex matching on undirected
graphs. `greedy_matching` iterates edges and greedily selects each edge if neither endpoint
is already matched. `parallel_matching_st` is the sequential baseline of the parallel
version: flip random coins per edge, then select edges where the coin is heads and all
adjacent edges are tails.

**VertexMatchingMtEph.rs** is the parallel version. `parallel_matching_mt` flips coins
sequentially (RNG cannot be parallelized), then calls `select_edges_parallel` which uses
divide-and-conquer via `ParaPair!` to select matching edges in parallel. The internal
helpers `flip_coins_parallel`, `select_edges_parallel`, `select_edges_recursive`, and
`should_select_edge` are scaffolding — `select_edges_recursive` is counted as a separate
hole by veracity (via OPAQUE_EXTERNAL).

### Chap61: Edge Contraction

**EdgeContractionStEph.rs** contracts edges in a matching by merging their endpoints.
`edge_contract` builds a vertex-to-block map from the matching, then re-routes all
graph edges through block representatives, dropping self-loops. `contract_round` calls
`greedy_matching` then `edge_contract`.

**EdgeContractionMtEph.rs** is the parallel version. `edge_contract_mt` does Phase 1
(build vertex-to-block map, sequential) and Phase 2 (build new vertices, sequential),
then delegates Phase 3 (build new edges) to `build_edges_parallel` which uses `ParaPair!`
divide-and-conquer. `contract_round_mt` calls `parallel_matching_mt` then `edge_contract_mt`.

### Chap62: Star Partition

**StarPartitionStEph.rs**: `sequential_star_partition` greedily assigns each unprocessed
vertex as a center, then claims its unprocessed neighbors into its star.

**StarPartitionMtEph.rs**: `parallel_star_partition` flips coins per vertex, then for
each tails-heads edge records a pointer update, applies those updates to build centers
and a partition map.

### Chap62: Star Contraction

**StarContractionStEph.rs**: `star_contract` is a higher-order recursive function.
Base case: no edges, call `base(vertices)`. Recursive case: partition the graph into
stars, build a quotient graph over centers, recurse on the quotient graph, then call
`expand` with the original graph info and the recursive result. `build_quotient_graph`
routes edges through the partition map and drops self-loops. `contract_to_vertices` is
a convenience wrapper with identity base/expand closures.

**StarContractionMtEph.rs**: Same structure as St version but uses `parallel_star_partition`
and `build_quotient_graph_parallel`. The parallel quotient-building uses `route_edges_parallel`
(divide-and-conquer via `ParaPair!`). `contract_to_vertices_mt` is the convenience wrapper.

## Proof Strategy

### Step 1: Read First

Before writing any code, read all 8 files. They are each under 200 lines. Then read:
- `/home/milnes/projects/APAS-VERUS/src/Chap05/SetStEph.rs` — to understand `SetStEph`
  operations and their specs (particularly `insert`, `mem`, `iter`, `clone`).
- `/home/milnes/projects/APAS-VERUS/src/Chap06/UnDirGraphStEph.rs` — to understand
  `vertices()`, `edges()`, `ng()`, `incident()`, `from_sets()`, `sizeE()`.
- `/home/milnes/projects/APAS-VERUS/src/Chap06/UnDirGraphMtEph.rs` — same for Mt variant.

### Step 2: Start with St Files

Prove all St (sequential) files first — they are simpler (no parallelism, no `Arc`). Then
use those proofs as templates for Mt. The execution logic is the same in most cases.

Order:
1. `VertexMatchingStEph.rs` — `greedy_matching` is the simplest: a single loop with two sets.
2. `StarPartitionStEph.rs` — similar pattern: two-level nested loop, three sets.
3. `EdgeContractionStEph.rs` — two phases, HashMap + two SetStEph accumulators.
4. `StarContractionStEph.rs` — recursive, needs `decreases graph.sizeV()` or similar.
5. Then Mt files using St proofs as templates.

### Step 3: Ghost State Pattern for St Functions

The key challenge in these proofs is connecting the imperative loop body to the
mathematical spec. The standard pattern:

```rust
pub fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> (matching: SetStEph<Edge<V>>)
    requires Self::spec_vertexmatchingsteph_wf(graph)
    ensures /* whatever spec says */
{
    let mut matching: SetStEph<Edge<V>> = SetLit![];
    let mut matched_vertices: SetStEph<V> = SetLit![];
    let ghost orig_graph = graph@;

    for edge in graph.edges().iter()
        invariant
            // What is true about matching and matched_vertices at each iteration?
            // matching@ is a valid subset of edges in orig_graph
            // matched_vertices@ = union of endpoints of edges in matching@
            // no two edges in matching@ share an endpoint
    {
        let Edge(u, v) = edge;
        if !matched_vertices.mem(u) && !matched_vertices.mem(v) {
            let _ = matching.insert(edge.clone());
            let _ = matched_vertices.insert(u.clone());
            let _ = matched_vertices.insert(v.clone());
        }
    }
    matching
}
```

Write the `invariant` clause that captures exactly what the loop maintains. Use
`ghost` variables to track the mathematical view (`@`) of sets as you build them.

### Step 4: Specs on the Functions

The trait specs are already written. When you lift a function out of `external_body`,
copy its trait `requires`/`ensures` onto the free function signature so Verus can
verify the body against them. The trait methods delegate to the free functions in all
these files, so the free function must carry matching postconditions.

If the trait has no `ensures` on a method (just `requires`), focus on proving the
body verifies (i.e., no internal verification errors) rather than a full functional
postcondition. But do NOT weaken any existing `ensures`.

### Step 5: Parallel Mt Functions

For Mt files:
- The sequential phases (building `vertex_to_block`, `coin_flips`, etc.) are identical
  to St — use the same ghost state tracking.
- For functions using `ParaPair!` (divide-and-conquer helpers like `build_edges_parallel`,
  `route_edges_parallel`, `select_edges_recursive`): these are already written correctly.
  The only change is removing `external_body` and writing the invariant for the
  `if size == 1` base case plus the merge step.
- Keep `ParaPair!` intact. Do NOT replace parallel code with sequential loops.
- `contract_round_mt` and `parallel_matching_mt` are thin wrappers — once the inner
  functions verify, these should follow immediately.

### Step 6: HashMap Operations

These files use `std::collections::HashMap`. In Verus, HashMap is an external type.
You can use `HashMap::new()`, `.insert()`, `.get()`, `.contains_key()`, and `.values()`
in exec code, but you cannot directly reason about their ghost state without using
a ghost `Map` tracked in parallel. Pattern:

```rust
let mut vertex_to_block = HashMap::<V, V>::new();
let ghost mut vertex_to_block_view: Map<V@, V@> = Map::empty();

// In loop body:
vertex_to_block.insert(u.clone(), u.clone());
proof { vertex_to_block_view = vertex_to_block_view.insert(u@, u@); }

// Invariant:
// vertex_to_block.get(&k) == Some(&v) <==> vertex_to_block_view.contains_key(k@) && vertex_to_block_view[k@] == v@
```

If Verus cannot bridge HashMap exec behavior to ghost state (common), you may need to
use a ghost abstraction that only tracks what you need for the postcondition. Keep it
minimal.

### Step 7: Recursive Functions (StarContraction)

`star_contract` and `star_contract_mt` are recursive. You need a `decreases` clause.
The graph's edge count decreases at each recursive call because star contraction strictly
reduces the graph. Use `decreases graph.sizeE()` or `decreases graph.sizeV()`.

For the higher-order closure arguments `base: &F` and `expand: &G`, read
`/home/milnes/projects/APAS-VERUS/src/standards/using_closures_standard.rs` before
writing any closure-related proof code. Closure requires/ensures propagation is specific
in Verus and has a standard pattern.

### Step 8: Validate Incrementally

Run `scripts/validate.sh` after each file or pair of files. Do NOT batch all 8 files
before validating — you want early error feedback.

Full path: `/home/milnes/projects/APAS-VERUS/scripts/validate.sh`

Show the full validation output in your report. Never pipe, grep, or filter validate output.

## Priority Order

Work in this order. Stop and move to the next file if you hit a proof blocker you
cannot resolve in 3-4 attempts.

1. `VertexMatchingStEph.rs` — `greedy_matching` (simple greedy loop)
2. `VertexMatchingStEph.rs` — `parallel_matching_st` (coin flip + selection loop)
3. `StarPartitionStEph.rs` — `sequential_star_partition` (nested loop, ghost map tracking)
4. `EdgeContractionStEph.rs` — `edge_contract` (two-phase HashMap + SetStEph)
5. `EdgeContractionStEph.rs` — `contract_round` (thin wrapper)
6. `StarContractionStEph.rs` — `build_quotient_graph` (loop, simpler than star_contract)
7. `StarContractionStEph.rs` — `star_contract` (recursive, needs decreases + closure spec)
8. `StarContractionStEph.rs` — `contract_to_vertices` (wrapper)
9. `VertexMatchingMtEph.rs` — `parallel_matching_mt`, `flip_coins_parallel`, `select_edges_parallel`
10. `VertexMatchingMtEph.rs` — `select_edges_recursive`, `should_select_edge`
11. `EdgeContractionMtEph.rs` — `build_edges_parallel` (parallel divide-and-conquer)
12. `EdgeContractionMtEph.rs` — `edge_contract_mt`, `contract_round_mt`
13. `StarPartitionMtEph.rs` — `parallel_star_partition`
14. `StarContractionMtEph.rs` — `route_edges_parallel`
15. `StarContractionMtEph.rs` — `build_quotient_graph_parallel`, `star_contract_mt`, `contract_to_vertices_mt`

## Key Points

- All functions already live inside `verus! { }`. Do not move anything.
- The `#[cfg(not(verus_keep_ghost))]` gates are correct — leave them in place.
- The `use` imports guarded by `#[cfg(not(verus_keep_ghost))]` (HashMap, Arc, etc.)
  are needed by the exec body — do not remove them.
- `SetLit![]` constructs an empty `SetStEph`. It is in scope via `use crate::SetLit;`.
- `ParaPair!(f1, f2)` spawns two closures and returns `(f1(), f2())`. It is in scope
  via `use crate::{ParaPair, SetLit};`.
- `N` is a type alias for `usize` (from `crate::Types::Types::*`).
- `B` is a type alias for `bool`.
- `HashOrd` is a combined trait bound (Hash + Ord + Clone + ...). Check its definition
  if needed: `/home/milnes/projects/APAS-VERUS/src/Types/Types.rs`.
- `Edge<V>` is a tuple struct `Edge(V, V)`. Pattern-match as `let Edge(u, v) = edge;`.

## What to Do if Stuck

If a proof attempt fails after 3-4 iterations:
1. Leave the `external_body` in place (do not remove it).
2. Document what you tried and where Verus reported the failure.
3. Move to the next function in the priority list.
4. Do NOT add `assume`, `accept`, or a weaker `ensures` to paper over the failure.

## Validation

After all work is done:
```
scripts/validate.sh    # must be 0 errors
scripts/rtt.sh         # runtime tests must pass
```

Run these sequentially, not in parallel.

## Report

Write your report to `/home/milnes/projects/APAS-VERUS/plans/agent1-r43-report.md`.

Include:

1. A table of holes before/after per file:

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|------------|

2. Chapters closed (if any).

3. Verification count before/after (e.g., 4362 → N verified).

4. For each function: whether you proved it, left it, or hit a blocker.

5. For each blocker: what you tried, what Verus reported, why it remains.

6. Full validate output (final run).
