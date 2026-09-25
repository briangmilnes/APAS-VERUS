# r224 report: Chap64 spanning tree on the two-vertex graph

Plan: `plans/r224-spantree-two-vertices.md`. Branch `r224/spantree`, base
`c22859345`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Root cause

`star_contract_mt_fuel` in `src/Chap62/StarContractionMtEph.rs` bounded the
recursion by `fuel = |V|` and, when the fuel reached 0, called `base` on a
graph that still had edges, as if the graph had none. A round of the
randomized star partition removes no vertex when no tails vertex has a heads
neighbor, and each such round still spent one unit of fuel. For
V = {0, 1}, E = {(0, 1)}, seed 42, the hash coins are (false, false) under
seed 42 and (true, true) under seed 43, so both rounds contract nothing, the
fuel runs out, and `base` returns the empty edge set for a graph with one
edge. The expand closures then map an empty quotient tree back to no edges.
The spanning tree ensured only `spec_setsteph_wf`, so verification could not
detect the wrong answer.

The same framework serves `count_components_hof` and `connected_components_hof`
in Chap63 `ConnectivityMtEph.rs`, which had the same exposure (a
fuel-exhausted round reports every remaining vertex as its own component).
`star_contract_fuel` in `StarContractionStEph.rs` has the same fuel design; its
greedy sequential partition removes a vertex whenever a non-loop edge exists,
so its fuel never ran out with edges left, but that fact was an unstated
argument, not a proof.

## Fix

The recursion now terminates on |V| instead of on fuel, and a graph with a
remaining edge is never handed to `base`:

- After the partition, the round checks `centers.size() < graph.sizeV()`.
- If the partition made no progress, `find_non_loop_edge` scans for an edge
  (u, v) with u != v, and `single_edge_partition` contracts it: v joins u's
  star, every other vertex is its own center. Its ensures prove the partition
  valid and `|centers| < |V|`.
- If every edge is a self-loop, the graph has no edge joining two vertices,
  and `base` is correct for it.
- `build_quotient_graph(_parallel)` now ensures `quotient@.V == centers@`,
  which gives the `decreases graph@.V.len()` measure.

No special case for two vertices. The random partition is unchanged and stays
parallel. A no-progress round adds O(n + m) work (a sequential edge scan and a
one-pass partition), the same order as a round, and still removes a vertex,
so the expected bounds are unchanged. The same change is applied to the StEph
framework, so neither framework relies on fuel.

## Files changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarContractionMtEph.rs | fuel -> `decreases |V|`; progress check; 2 helpers; ensures |
| 2 | 62 | StarContractionStEph.rs | same as row 1 for the sequential framework |
| 3 | 64 | SpanTreeMtEph.rs | closure specs; stronger ensures; Part 2 via `to_seq` |
| 4 | 64 | SpanTreeStEph.rs | same as row 3 |
| 5 | 64 | TestSpanTreeMtEph.rs | seed sweep test |
| 6 | 64 | TestSpanTreeStEph.rs | graph sweep test |
| 7 | 63 | TestConnectivityMtEph.rs | `count_components_hof` seed sweep |

Row 1 also adds a proof block in `route_edges_parallel` (unchanged exec code):
it states `start <= mid < end` and the right-half range fact once, which the
precondition of the second recursive call needed under Z3 seeds 5-8.

## Spec changes (all strengthened, none weakened)

| # | Chap | File | Function | New ensures |
|---|------|------|----------|-------------|
| 1 | 62 | StarContractionMtEph.rs | star_contract_mt | result from base on V, or from expand on (V, E, valid partition) |
| 2 | 62 | StarContractionStEph.rs | star_contract | same as row 1 |
| 3 | 62 | both | build_quotient_graph* | `quotient@.V == centers@` |
| 4 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt | every tree edge is a graph edge |
| 5 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction | every tree edge is a graph edge |

Rows 4-5 state `tree@.contains((u, w)) ==> A.contains((u, w)) || A.contains((w, u))`
(the edge set of an undirected graph stores one orientation). The trait
declarations carry the same ensures. The proof uses row 1's disjunction: the
base closure returns the empty set, and the expand closure ensures its output
is contained in `original_edges` up to orientation.

How the expand closure proves containment: Part 2 (quotient tree edges mapped
back) now iterates `original_edges.to_seq()`, so each inserted edge is proved
to be an original edge. Part 1 (satellite-to-center edges) inserts the edge
only when `original_edges.mem` finds it in either orientation. The test never
fails for a partition built from graph edges; it states the fact here because
neither partition function ensures "a satellite is adjacent to its center".
Proving that in `parallel_star_partition` means carrying edge membership
through `build_th_edges_mt`, `build_satellite_map_mt` and
`build_p_vec_with_inject_mt`; that would remove the test and is left for a
later round.

Not attempted: acyclicity and |tree| = |V| - components as ensures. They need
the expand closure's result related to the recursive result across levels,
which the fixed `r_inv` predicate of the framework cannot express.

## Tests

| # | Chap | File | Test | Result |
|---|------|------|------|--------|
| 1 | 64 | TestSpanTreeMtEph.rs | test_spanning_tree_mt_two_vertices (unchanged) | pass |
| 2 | 64 | TestSpanTreeMtEph.rs | test_spanning_tree_mt_seed_sweep | pass |
| 3 | 64 | TestSpanTreeStEph.rs | test_spanning_tree_graph_sweep | pass |
| 4 | 63 | TestConnectivityMtEph.rs | test_count_components_hof_mt_seed_sweep | pass |

The sweep graphs, for n = 2..8: path, star, cycle (n >= 3), complete, two
disjoint paths (plus an isolated vertex for odd n), no edges, and a path with a
self-loop at every vertex. MtEph runs seeds 0..200 on each (48 graphs x 200
seeds). StEph takes no seed, so each graph runs once. Each result is checked
as a spanning forest: every tree edge is a graph edge, no tree edge closes a
cycle, the tree connects exactly the graph's components, and
|tree| = |V| - (number of components); connected graphs also pass
`verify_spanning_tree`. Test 4 checks `count_components_hof` against the
known component count for paths and split paths, seeds 0..200.

RTT runs:
- `scripts/rtt.sh spanning_tree`: 38 tests run, 38 passed.
- Chap62/63/64 test binaries (`cargo nextest run --release -j 6
  -E 'binary(/TestStar|TestSpanTree|TestConnectivity/)'`, the name filter of
  `rtt.sh` does not match binary names): 118 tests run, 118 passed
  (`logs/rtt.20260925-102319.log`).

## Verification

- `scripts/validate.sh isolate Chap62`: 1264 verified, 0 errors, no warnings,
  no trigger notes.
- `scripts/validate.sh isolate Chap64`: 1279 verified, 0 errors, no warnings,
  no trigger notes (`logs/validate.20260925-103035.log`).
- Chap63 (`ConnectivityMtEph`, `ConnectivityStEph`) calls the changed
  frameworks but was not validated here: this round's constraints allowed
  only isolate Chap62 and Chap64. Its call sites only gain ensures; the main
  session's full validate covers it.

## Seeds 1-8

`VERUS_EXTRA_ARGS="--verify-only-module <m> --smt-option smt.random_seed=N"`.

| # | Chap | Module | Seeds passing | Note |
|---|------|--------|---------------|------|
| 1 | 62 | StarContractionMtEph | 8/8 (9 verified each) | after the route_edges hint |
| 2 | 62 | StarContractionStEph | 8/8 (8 verified each) | |
| 3 | 64 | SpanTreeMtEph | 8/8 (2 verified each) | |
| 4 | 64 | SpanTreeStEph | 8/8 (2 verified each) | |

Before the `route_edges_parallel` hint, row 1 failed seeds 5-8 on the
precondition of the second recursive call (`logs/validate.20260925-102441.log`
through `-102501.log`).

## Holes

`scripts/holes.sh src/Chap62/` and `src/Chap64/`: 0 holes in each, as before.
No assume, admit, accept, external_body, `#![auto]`, or `veracity:
no_requires` added; no rlimit raised.

## Full checks (main session, after the round)

| # | Chap | Measure | Value |
|---|---|---|---|
| 1 | — | Full `scripts/validate.sh` (covers Chap63) | 5825 verified, 0 errors |
| 2 | — | Full `scripts/rtt.sh` | 4371 of 4371 pass |
| 3 | — | `scripts/ptt.sh` | 328 of 328 pass |
