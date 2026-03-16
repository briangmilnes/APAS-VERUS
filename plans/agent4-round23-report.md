# Agent 4 — Round 23 Report: Verusify Chap59 Johnson + Chap41 ArraySet filter

## Mission

1. Move Johnson's APSP algorithm (Chap59) from outside `verus!` to fully verified inside `verus!`.
2. Prove ArraySetStEph::filter (Chap41) — remove 1 external_body.
3. Bonus: Enable and fix DijkstraStEphI64 (Chap57) — required dependency for Johnson.

## Results

| # | Chap | File | Holes Before | Holes After | Change |
|---|------|------|-------------|------------|--------|
| 1 | 59 | JohnsonStEphI64.rs | 0 (invisible) | 1 | +1 (net real gain) |
| 2 | 57 | DijkstraStEphI64.rs | 0 (invisible) | 5 | +5 (net real gain) |
| 3 | 41 | ArraySetStEph.rs | 1 | 0 | -1 |

Before: Johnson was behind `#[cfg(not(verus_keep_ghost))]` — Verus never saw it.
Dijkstra was commented out in lib.rs entirely. Both reported 0 holes because they
were invisible. ArraySetStEph filter had 1 external_body.

After: Johnson fully inside `verus!` with 10 verified functions. Dijkstra enabled and
inside `verus!` with specs and loop invariants. ArraySetStEph filter proved — 0 holes.

## What Changed

### JohnsonStEphI64.rs (Chap59) — Complete Rewrite

1. **Replaced closures/tabulate with loops**: Original used `Fn`-based iteration and
   `tabulate`. Rewritten with explicit while loops and loop invariants for Verus.

2. **Moved everything inside `verus!`**: Removed `#[cfg(not(verus_keep_ghost))]` gate.
   All algorithmic code now verified.

3. **Trait with specs**: `johnson_apsp` with requires for `graph@.V.len() > 0`,
   `spec_labgraphview_wf`, `valid_key_type_WeightedEdge`, and vertex indexability
   (`forall|v| graph@.V.contains(v) <==> v < graph@.V.len()`). Ensures result
   dimensions match graph.

4. **7 helper functions with specs**:
   - `neg_cycle_error_string()`: String creation (external_body, standard pattern)
   - `adjust_distance(d_prime, h_u, h_v)`: i128 overflow-safe readjustment
   - `reweight_edge(weight, h_u, h_v)`: i128 clamped reweighting
   - `build_vertex_set(max_val)`: cardinality-tracked vertex set construction
   - `add_dummy_source(graph, n)`: augmented graph with dummy vertex n
   - `reweight_graph(graph, potentials, n)`: reweighted graph construction
   - `create_negative_cycle_result(n)`: APSP result for negative cycles

5. **Loop invariants**: 7 loops with complete invariants (vertex set build, edge copy
   outer+inner, add-dummy edges, reweight outer+inner, main Dijkstra phase outer+inner).

6. **Broadcast uses**: `group_hash_set_with_view_plus_axioms`, `group_Pair_axioms`,
   `group_WeightedEdge_axioms` — required for SetStEph iteration and WeightedEdge
   valid_key_type propagation.

7. **`axiom_set_insert_len` broadcast**: Used in `build_vertex_set` to track
   `vertices@.len() == i as nat` through the loop.

### DijkstraStEphI64.rs (Chap57) — Enabled + Fixed

1. **Uncommented in lib.rs**: Was `// pub mod DijkstraStEphI64;`. Now active.

2. **Fixed API drift**: Updated from old iterator API (`it.elements`, `it.pos`) to
   current pattern (`it@.0`, `it@.1`, `it.next()` loop).

3. **Named return value**: `-> SSSPResultStEphI64` → `-> (sssp: SSSPResultStEphI64)`.

4. **Updated ensures**: `sssp.distances.spec_len()` → `sssp.spec_distances().len()`,
   `sssp.source` → `sssp.spec_source()`.

5. **Added TotalOrder impl for PQEntry**: Required by BinaryHeapPQ. Uses
   `(dist, vertex)` lexicographic order consistent with derived PartialEq.

6. **Ord/PartialOrd external_body**: vstd auto-postconditions on Ord::cmp conflict
   with custom ordering. Same pattern as Probability (Chap30).

7. **3 assumes**: `obeys_feq_clone::<PQEntry>()` (standard feq/clone workaround),
   PQ size bounds for delete_min and insert (BinaryHeapPQ internal array bounds).

### ArraySetStEph.rs (Chap41) — filter Proved

1. **Removed `#[verifier::external_body]`** from filter function.

2. **Added loop invariants**: spec_pred tracking per element (forward direction) and
   completeness direction (every qualifying element in self is in result).

3. **Connected `f.ensures` to `spec_pred`**: In both if-true and if-false branches,
   established that `f.ensures((&elem,), keep)` implies `keep == spec_pred(elem@)`.

4. **Final proof block**: Three assertions establishing wf, containment direction
   (`filtered@ ⊆ self@ ∩ spec_pred`), and completeness direction
   (`self@ ∩ spec_pred ⊆ filtered@`).

## Tests

- 3999 verified, 0 errors (was 3978 before this round)
- 2606 RTT pass (was 2600 — gained 6 Johnson + 7 Dijkstra - 1 already counted)
  - 6 Johnson tests: example_59_1, negative_cycle, negative_weights, simple_graph,
    single_vertex, disconnected_graph
  - 7 Dijkstra tests: example_57_1, example_57_3, larger_graph, single_vertex,
    unreachable_vertices, path_extraction, multiple_paths_same_weight

## Remaining Holes

| # | Chap | File | Line | Type | Reason |
|---|------|------|------|------|--------|
| 1 | 59 | JohnsonStEphI64.rs | 68 | external_body | `neg_cycle_error_string()` — String creation |
| 2 | 57 | DijkstraStEphI64.rs | 91 | external_body | `Ord::cmp` — vstd cmp_spec conflict |
| 3 | 57 | DijkstraStEphI64.rs | 108 | external_body | `PartialOrd::partial_cmp` — same |
| 4 | 57 | DijkstraStEphI64.rs | 156 | assume | `obeys_feq_clone::<PQEntry>()` — standard feq/clone |
| 5 | 57 | DijkstraStEphI64.rs | 173 | assume | PQ size bound for delete_min |
| 6 | 57 | DijkstraStEphI64.rs | 207 | assume | PQ size bound for insert |

Holes 1-3 are non-algorithmic (String creation, Ord/PartialOrd trait compat).
Hole 4 is the standard feq/clone workaround for concrete types.
Holes 5-6 are BinaryHeapPQ internal array bounds (PQ size < usize::MAX).

## Techniques

- **`build_vertex_set` with `axiom_set_insert_len`**: Constructs `{0..n}` as SetStEph
  while tracking `vertices@.len() == i as nat`. The vstd broadcast
  `axiom_set_insert_len` provides `s.insert(a).len() == s.len() + if s.contains(a) { 0 } else { 1 }`.

- **Vertex indexability precondition**: Johnson requires
  `forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len()` because APAS graphs
  use 0-indexed vertices but the graph spec doesn't guarantee this. Propagated through
  ensures of `add_dummy_source` and `reweight_graph`.

- **`reweight_edge` clamping**: Computing `weight + h(u) - h(v)` where weight is i128
  and h values are i64. First compute `diff = h(u) - h(v)` (safe in i128), then
  clamp `weight + diff` to avoid i128 overflow.

- **Runtime bounds checks**: Edge endpoints checked with `*v < n` rather than proving
  set membership from iterator specs. Pragmatic approach that avoids deep iterator
  specification reasoning.

- **Loop isolation(false)**: Outer loop variables (n, potentials, etc.) accessed in
  inner loops via `#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]`.

## Files Changed

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | 59 | JohnsonStEphI64.rs | Complete rewrite inside verus! |
| 2 | 57 | DijkstraStEphI64.rs | Enabled, fixed API drift, added TotalOrder |
| 3 | 41 | ArraySetStEph.rs | Proved filter (removed external_body) |
| 4 | — | lib.rs | Uncommented JohnsonStEphI64, DijkstraStEphI64 |
| 5 | — | Cargo.toml | Enabled TestJohnsonStEphI64, TestDijkstraStEphI64 |
