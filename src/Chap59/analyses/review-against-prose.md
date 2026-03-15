# Review Against Prose: Chapter 59 -- Johnson's Algorithm

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap59.txt` (APAS Chapter 59: Johnson's Algorithm)

## Phase 1: Inventory

From `veracity-review-module-fn-impls.md`:

| # | Chap | File | Fns | Tr | ML | V! | -V! | Holes | NoSpec |
|---|------|------|-----|----|----|----|----|----|----|
| 1 | 59 | JohnsonStEphI64.rs | 4 | 1 | 4 | 1 | 3 | 0 | 4 |
| 2 | 59 | JohnsonStEphF64.rs | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 3 | 59 | JohnsonMtEphI64.rs | 5 | 1 | 5 | 1 | 4 | 0 | 5 |
| 4 | 59 | JohnsonMtEphF64.rs | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Total: 9 exec functions (I64 only), 0 holes, 4 clean modules.

Note: Like Bellman-Ford, the I64 implementations are **entirely outside verus!**
(`#[cfg(not(verus_keep_ghost))]`). Only the trait declarations are inside `verus!`. The
algorithmic code is NOT verified.

## Phase 2: Prose Inventory

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 59 | Algorithm 59.1 (Johnson's APSP) | algorithm |
| 2 | 59 | Lemma 59.1 (Path Potentials) | lemma |
| 3 | 59 | Lemma 59.2 (Non-Negative Weights) | lemma |
| 4 | 59 | Example 59.1 (Johnson run) | example |

Cost analysis from prose:
- Phase 1 (Bellman-Ford): Work O(nm), Span O(n log n).
- Phase 2 (Reweighting): Work O(m), Span O(m).
- Phase 3 (n Dijkstras in parallel): Work O(mn log n), Span O(m log n).
- Total: Work O(mn log n), Span O(m log n).
- Parallelism: Theta(n) from parallel Dijkstra runs.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Chap | File | Function | Annotation |
|---|------|------|----------|------------|
| 1 | 59 | JohnsonStEphI64.rs | johnson_apsp | APAS: W O(mn log n), S O(m log n). Claude: W O(mn log n), S O(mn log n) -- sequential Dijkstra loop. |
| 2 | 59 | JohnsonStEphI64.rs | add_dummy_source | APAS: N/A. Claude: W O(n+m), S O(n+m). |
| 3 | 59 | JohnsonStEphI64.rs | reweight_graph | APAS: W O(m), S O(m). Claude: W O(n+m), S O(n+m). |
| 4 | 59 | JohnsonStEphI64.rs | create_negative_cycle_result | APAS: N/A. Claude: W O(n^2), S O(n^2). |
| 5 | 59 | JohnsonMtEphI64.rs | johnson_apsp | APAS: W O(mn log n), S O(m log n), Parallelism Theta(n). Claude agrees. |
| 6 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | APAS: N/A. Claude: W O(k * m log n), S O(m log n). |
| 7 | 59 | JohnsonMtEphI64.rs | add_dummy_source | Same as StEph version. |
| 8 | 59 | JohnsonMtEphI64.rs | reweight_graph | Same as StEph version. |
| 9 | 59 | JohnsonMtEphI64.rs | create_negative_cycle_result | Same as StEph version. |

### 3b. Implementation Fidelity

**JohnsonStEphI64.rs (`johnson_apsp`)** vs Algorithm 59.1:

| # | Chap | Prose Step | Code | Match? |
|---|------|-----------|------|--------|
| 1 | 59 | G+ = add dummy s, zero edges | `add_dummy_source(graph, n)` | Yes |
| 2 | 59 | D = BellmanFord(G+, s) | `bellman_ford(&graph_with_dummy, dummy_idx)` | Yes |
| 3 | 59 | Handle neg cycle (return None) | `Err(_) => create_negative_cycle_result(n)` | Yes (returns UNREACHABLE matrix) |
| 4 | 59 | w'(u,v) = w(u,v) + D[u] - D[v] | `reweight_graph(graph, &potentials, n)` | Yes |
| 5 | 59 | G' = (V, E, w') | Result of reweight_graph | Yes |
| 6 | 59 | For each u: run Dijkstra on G' | `for u in 0..n { dijkstra(&reweighted_graph, u) }` | Yes (sequential) |
| 7 | 59 | Readjust: d - D[u] + D[v] | `d_prime - p_u + p_v` | Yes |

**Deviations**:
- The prose presents the union of all Dijkstra results. The code builds row-by-row
  distance and predecessor matrices using `ArraySeqStEphS::append`.
- The StEph version runs Dijkstra **sequentially** in a `for u in 0..n` loop. The prose
  says "runs Dijkstra's algorithm for each vertex u" without specifying sequential or
  parallel. The cost table shows parallel span O(m log n), implying parallelism.

**JohnsonMtEphI64.rs (`johnson_apsp`)** -- Parallel variant:

The Mt variant uses `parallel_dijkstra_all` which implements recursive divide-and-conquer
via `ParaPair!`:
- Base case (range_size == 1): run single Dijkstra, readjust distances.
- Recursive case: split range at midpoint, run left and right halves in parallel via
  `ParaPair!`, concatenate results.

This achieves Theta(n) parallelism as described in the prose. The implementation is correct:
binary splitting gives O(log n) depth, each leaf runs a full Dijkstra in O(m log n).

**add_dummy_source**: Creates vertex n as dummy source with zero-weight edges to all
original vertices. Matches the prose exactly.

**reweight_graph**: Computes w'(u,v) = w(u,v) + p(u) - p(v) for all edges. Uses i128
arithmetic to avoid overflow. Matches Lemma 59.1 formula.

**F64 variants** (JohnsonStEphF64.rs, JohnsonMtEphF64.rs): Empty stubs with placeholder
verus! blocks. Blocked by missing f64 graph types and BellmanFord/Dijkstra f64 variants.

### 3c. Spec Fidelity

**JohnsonStEphI64.rs (`johnson_apsp`)**:
- Trait declaration: `fn johnson_apsp(...) -> AllPairsResultStEphI64` with **NoSpec**.
- The implementation is outside verus! -- no verification.
- **Assessment**: No spec. Completely unverified.

**JohnsonMtEphI64.rs (`johnson_apsp`)**:
- Same as StEph: trait with NoSpec inside verus!, implementation outside.
- **Assessment**: No spec. Completely unverified.

All helper functions (`add_dummy_source`, `reweight_graph`, `create_negative_cycle_result`,
`parallel_dijkstra_all`) are outside verus! with no specs.

## Phase 4: Parallelism Review

**JohnsonMtEphI64.rs** is the Mt module. Parallelism assessment:

| # | Chap | Aspect | Expected | Actual |
|---|------|--------|----------|--------|
| 1 | 59 | Phase 3 parallelism | Theta(n) via n independent Dijkstras | ParaPair! divide-and-conquer |
| 2 | 59 | Parallelism mechanism | Parallel map/tabulate over vertices | Recursive binary split with ParaPair! |
| 3 | 59 | Span | O(m log n) | O(m log n) -- correct |
| 4 | 59 | Phase 1 (BF) | Sequential | Sequential -- correct |
| 5 | 59 | Phase 2 (reweight) | Sequential | Sequential -- correct |

The parallelism is correctly implemented. The `ParaPair!` macro provides fork-join
parallelism matching the prose's description of running n Dijkstras in parallel. The
binary splitting achieves O(log n) depth for scheduling, with each leaf running Dijkstra
in O(m log n), giving overall Span O(m log n + log n) = O(m log n).

**Concern**: The code clones the graph and potentials for each recursive call
(`graph.clone()`, `potentials.clone()`). This adds O(n+m) work per recursive call. With
O(n) recursive calls, total cloning overhead is O(n(n+m)). For dense graphs (m = O(n^2)),
this is O(n^3) which could dominate the O(mn log n) Dijkstra work. For sparse graphs
(m = O(n)), cloning overhead is O(n^2) vs O(n^2 log n) Dijkstra work, so it is subdominant.
This is an implementation cost, not an algorithmic deviation.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Covers |
|---|------|-----------|--------|
| 1 | 59 | TestJohnsonStEphI64.rs | JohnsonStEphI64 |
| 2 | 59 | TestJohnsonStEphF64.rs | JohnsonStEphF64 |
| 3 | 59 | TestJohnsonMtEphI64.rs | JohnsonMtEphI64 |
| 4 | 59 | TestJohnsonMtEphF64.rs | JohnsonMtEphF64 |

All 4 modules have test files. F64 tests are likely trivial since those modules are stubs.

## Phase 6: PTT Review

No proof-time tests for Chapter 59. Not applicable since the algorithmic code is outside
verus!.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 59 | johnson_apsp is entirely outside verus! (StEph) | Critical | Core algorithm has no Verus verification. |
| 2 | 59 | johnson_apsp is entirely outside verus! (MtEph) | Critical | Parallel version also unverified. |
| 3 | 59 | No correctness spec | High | All-pairs distances should equal delta_G(u,v). Not formalized. |
| 4 | 59 | F64 variants are empty stubs | Medium | Blocked by missing f64 graph infrastructure. |
| 5 | 59 | Lemma 59.1 (Path Potentials) not formalized | Medium | Key correctness property: reweighting preserves shortest paths. No proof. |
| 6 | 59 | Lemma 59.2 (Non-Negative Weights) not formalized | Medium | Reweighted edges are non-negative. Critical for Dijkstra applicability. No proof. |
| 7 | 59 | ParaPair! parallelism is correct but unverified | Medium | Mt variant uses ParaPair! correctly but since code is outside verus!, the parallelism is not formally verified. |
| 8 | 59 | Graph cloning overhead | Low | Each recursive call clones full graph. Dominated by Dijkstra for sparse graphs but could matter for dense. |

**Verusification path**: Johnson depends on Bellman-Ford and Dijkstra. Verusification order:
1. Verusify Bellman-Ford (Chap 58) -- must move inside verus!, replace HashMap.
2. Strengthen Dijkstra ensures (Chap 57) -- add shortest-path correctness.
3. Then Johnson can be verusified with specs referencing BF and Dijkstra postconditions.
4. Mt variant requires closure specs for ParaPair! -- standard parallel closure pattern.

**Overall assessment**: Chapter 59 is technically **clean** (0 holes) because all code is
outside verus!. However, the algorithmic content is **completely unverified** for both St
and Mt variants. The Mt variant correctly implements parallel divide-and-conquer via
ParaPair!. The implementation faithfully follows Algorithm 59.1 including the reweighting
trick. F64 variants are blocked stubs.

## Phase 8: TOC Review

- JohnsonStEphI64.rs: Trait inside verus!, implementation outside with cfg gates.
  Minimal TOC appropriate for current unverified state.
- JohnsonMtEphI64.rs: Same structure. Uses ParaPair! outside verus!.
- F64 stubs: Minimal, acceptable.

No ordering violations in any file.
