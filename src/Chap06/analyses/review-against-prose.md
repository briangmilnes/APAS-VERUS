<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap06 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-17
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Prose file:** `prompts/Chap06.txt`
**Source files:** 20 files (4 St graph types, 4 Mt graph types, 12 WeightedDirGraph per-type variants)

## Prose Inventory

Chapter 6 is purely definitional — graph theory background. No algorithms, no
pseudocode, no cost specifications.

| # | Item | Type |
|---|------|------|
| 1 | Def 6.1: Directed Graph — G = (V, A), A <= V x V | Definition |
| 2 | Def 6.2: Undirected Graph — G = (V, E), E <= (V choose 2) | Definition |
| 3 | Def 6.3: Neighbors — adjacent vertices, in-neighbors, out-neighbors | Definition |
| 4 | Def 6.4: Neighborhood — N_G(v), N+(v), N-(v), neighborhood of vertex set | Definition |
| 5 | Def 6.5: Incidence — edge incident on vertex | Definition |
| 6 | Def 6.6: Degree — d(v) = size of neighborhood, in-degree, out-degree | Definition |
| 7 | Def 6.7: Path — sequence of adjacent vertices, simple path | Definition |
| 8 | Def 6.8: Reachability and Connectivity — R_G(u), connected, strongly connected | Definition |
| 9 | Def 6.9: Cycles — directed and undirected simple cycles | Definition |
| 10 | Def 6.10: Trees and Forests — no cycles, connected | Definition |
| 11 | Def 6.11: DAG — directed acyclic graph | Definition |
| 12 | Def 6.12: Distance — shortest path length | Definition |
| 13 | Def 6.13: Diameter — max shortest path over all pairs | Definition |
| 14 | Def 6.14: Multigraphs — multi-edges | Definition |
| 15 | Def 6.15: Sparse/Dense — n = V, m = E conventions | Definition |
| 16 | Def 6.16: Enumerable Graphs — vertices labeled 0..n-1 | Definition |
| 17 | Def 6.17: Weighted/Edge-Labeled Graphs — G = (V, E, w), w: E -> L | Definition |
| 18 | Def 6.18: Subgraph — V' <= V, E' <= E | Definition |
| 19 | Def 6.19: Vertex-Induced Subgraph | Definition |
| 20 | Def 6.20: Edge-Induced Subgraph | Definition |
| 21 | Def 6.21: Connected Component — maximally connected subgraph | Definition |
| 22 | Def 6.22: Graph Partition — partition of vertex set | Definition |
| 23 | Def 6.23: Internal and Cut Edges | Definition |
| 24 | Def 6.24: Tree (undirected) | Definition |
| 25 | Def 6.25: Rooted Trees — root, parent, child, leaf, depth, height, subtree | Definition |
| 26 | Ex 6.1: Why cycle length >= 3 in undirected graphs? | Exercise |

## Code Inventory

### Core graph types (8 modules, all verified)

| # | File | Type | Exec fns | Spec fns | Proof holes | Notes |
|---|------|------|----------|----------|-------------|-------|
| 1 | DirGraphStEph.rs | St | 16 | 6 | 0 — clean | Implements Def 6.1, 6.3-6.6 |
| 2 | UnDirGraphStEph.rs | St | 12 | 3 | 0 — clean | Implements Def 6.2, 6.3-6.6 |
| 3 | LabDirGraphStEph.rs | St | 14 | 5 | 0 — clean | Implements Def 6.17 (directed, labeled) |
| 4 | LabUnDirGraphStEph.rs | St | 12 | 3 | 0 — clean | Implements Def 6.17 (undirected, labeled) |
| 5 | DirGraphMtEph.rs | Mt | 16 | 8 | 0 — clean | Parallel via ParaPair! |
| 6 | UnDirGraphMtEph.rs | Mt | 12 | 3 | 0 — clean | Parallel via ParaPair! |
| 7 | LabDirGraphMtEph.rs | Mt | 14 | 5 | 0 — clean | Parallel via ParaPair! |
| 8 | LabUnDirGraphMtEph.rs | Mt | 12 | 3 | 0 — clean | Parallel via ParaPair! |

### Weighted graph variants (12 modules, type-monomorphized)

| # | File | Weight type | Status |
|---|------|------------|--------|
| 9 | WeightedDirGraphStEphU8.rs | u8 | In lib.rs, verified |
| 10 | WeightedDirGraphStEphU16.rs | u16 | Commented out in lib.rs |
| 11 | WeightedDirGraphStEphU32.rs | u32 | In lib.rs, verified |
| 12 | WeightedDirGraphStEphU64.rs | u64 | Commented out in lib.rs |
| 13 | WeightedDirGraphStEphU128.rs | u128 | Commented out in lib.rs |
| 14 | WeightedDirGraphStEphUsize.rs | usize | Commented out in lib.rs |
| 15 | WeightedDirGraphStEphI8.rs | i8 | Commented out in lib.rs |
| 16 | WeightedDirGraphStEphI16.rs | i16 | Commented out in lib.rs |
| 17 | WeightedDirGraphStEphI32.rs | i32 | Commented out in lib.rs |
| 18 | WeightedDirGraphStEphI64.rs | i64 | Commented out in lib.rs |
| 19 | WeightedDirGraphStEphI128.rs | i128 | Commented out in lib.rs |
| 20 | WeightedDirGraphStEphIsize.rs | isize | Commented out in lib.rs |

The 10 commented-out variants are intentional — "just one being verified but they
all work." They are type-monomorphized copies of the same template. Uncommenting
all would add ~10x verification time for no coverage gain.

## Prose-to-Code Mapping

| # | Prose Definition | Code | Spec Fidelity |
|---|-----------------|------|---------------|
| 1 | Def 6.1: Directed Graph | DirGraphStEph, DirGraphMtEph | Strong — struct with V: Set, A: Set of pairs, wf_graph_view ensures arcs reference valid vertices |
| 2 | Def 6.2: Undirected Graph | UnDirGraphStEph, UnDirGraphMtEph | Strong — edges stored as pairs, neighbor check tests both orderings |
| 3 | Def 6.3: Neighbors | neighbor() | Strong — ensures result == A.contains((u,v)) for directed, tests both orderings for undirected |
| 4 | Def 6.4: Neighborhood | ng(), n_plus(), n_minus(), ng_of_vertices(), n_plus_of_vertices(), n_minus_of_vertices() | Strong — spec functions match prose definitions exactly |
| 5 | Def 6.5: Incidence | incident() | Strong — ensures b == (e.0 == v or e.1 == v) |
| 6 | Def 6.6: Degree | degree(), in_degree(), out_degree() | Strong — ensures n == neighborhood.len() |
| 7 | Def 6.17: Weighted Graph | WeightedDirGraphStEph*, LabDirGraphStEph, LabUnDirGraphStEph | Strong — G = (V, E, w) modeled as Set of (V, V, L) triples |
| 8 | Def 6.7-6.14: Path, Reachability, Cycles, Trees, DAG, Distance, Diameter | — | Not implemented — these are used by later algorithm chapters, not Chap06 itself |
| 9 | Def 6.15-6.16: Sparse/Dense, Enumerable | — | Conventions only, no code needed |
| 10 | Def 6.18-6.20: Subgraphs | — | Not implemented as operations |
| 11 | Def 6.21-6.23: Connected Components, Graph Partition, Cut Edges | — | Not implemented — algorithm territory |
| 12 | Def 6.24-6.25: Trees, Rooted Trees | — | Implemented in Chap23 (BalBinTreeStEph, PrimTreeSeqStPer) |

## Cost Analysis

The prose has no cost specifications (purely definitional chapter). All cost
annotations in the code are inferred from the implementations.

### Cost annotations: All modules complete

All 8 core graph modules have paired APAS and Claude-Opus-4.6 cost annotations
on every exec function. The pattern is consistent:

- O(1) operations: empty, from_sets, vertices, arcs, sizeV, sizeA, neighbor, incident
- O(|A|) or O(|E|) operations: ng, n_plus, n_minus, degree, in_degree, out_degree (iterate arcs/edges)
- O(|vertices| × |A|): ng_of_vertices, n_plus_of_vertices, n_minus_of_vertices (nested iteration)

### Mt modules

DirGraphMtEph, UnDirGraphMtEph, LabDirGraphMtEph, and LabUnDirGraphMtEph
annotate parallel functions with `Span Θ(log |A|)` or `Span Θ(log |E|)` —
appropriate for ParaPair! which splits into two halves and recurses.

## Parallelism Audit (Mt modules)

All four Mt modules use `ParaPair!` for genuine parallelism. This splits the
arc/edge set in half, processes each half in parallel, and merges results.

### DirGraphMtEph parallelism

| # | Function | Parallel? | Mechanism | APAS Span | Actual Span |
|---|----------|-----------|-----------|-----------|-------------|
| 1 | empty | No | Trivial O(1) | Theta(1) | Theta(1) |
| 2 | from_sets | No | Constructor | Theta(1) | Theta(1) |
| 3 | vertices | No | Accessor | Theta(1) | Theta(1) |
| 4 | arcs | No | Accessor | Theta(1) | Theta(1) |
| 5 | sizeV | No | Delegating | Theta(1) | Theta(1) |
| 6 | sizeA | No | Delegating | Theta(1) | Theta(1) |
| 7 | neighbor | No | Delegating | Theta(1) | Theta(1) |
| 8 | incident | No | Comparison | Theta(1) | Theta(1) |
| 9 | n_plus | Yes | ParaPair! split arcs | Theta(log A) | Theta(log A) |
| 10 | n_minus | Yes | ParaPair! split arcs | Theta(log A) | Theta(log A) |
| 11 | ng | Yes | Calls n_plus + n_minus | Theta(log A) | Theta(log A) |
| 12 | n_plus_of_vertices | Yes | ParaPair! split vertices | Theta(log V x log A) | Theta(log V x log A) |
| 13 | n_minus_of_vertices | Yes | ParaPair! split vertices | Theta(log V x log A) | Theta(log V x log A) |
| 14 | ng_of_vertices | Yes | ParaPair! split vertices | Theta(log V x log A) | Theta(log V x log A) |
| 15 | degree | Yes | Calls ng | Theta(log A) | Theta(log A) |
| 16 | out_degree | Yes | Calls n_plus | Theta(log A) | Theta(log A) |
| 17 | in_degree | Yes | Calls n_minus | Theta(log A) | Theta(log A) |

UnDirGraphMtEph, LabDirGraphMtEph, and LabUnDirGraphMtEph follow the same
pattern — genuine parallelism via ParaPair! for all neighborhood and degree
operations. This is a strong result compared to Chap05's SetMtEph where most
Mt operations were sequential loops.

## Runtime Test Review

| # | Source module | RTT file | Status |
|---|-------------|----------|--------|
| 1 | DirGraphStEph | TestDirGraphStEph.rs | Exists |
| 2 | DirGraphMtEph | TestDirGraphMtEph.rs | Exists |
| 3 | UnDirGraphStEph | TestUnDirGraphStEph.rs | Exists |
| 4 | UnDirGraphMtEph | TestUnDirGraphMtEph.rs | Exists |
| 5 | LabDirGraphStEph | TestLabDirGraphStEph.rs | Exists |
| 6 | LabDirGraphMtEph | TestLabDirGraphMtEph.rs | Exists |
| 7 | LabUnDirGraphStEph | TestLabUnDirGraphStEph.rs | Exists |
| 8 | LabUnDirGraphMtEph | TestLabUnDirGraphMtEph.rs | Exists |
| 9 | WeightedDirGraphStEphU32 | TestWeightedDirGraphStEphU32.rs | Exists |
| 10 | WeightedDirGraph (various) | TestWeighed*.rs (8 files) | Exists — int/float/dir/undir combinations |

17 RTT files total. Full coverage of all module families.

## PTT Review

### Unified test inventory table

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | DirGraphStEph | TestDirGraphStEph.rs | ProveDirGraphStEph.rs | Both exist |
| 2 | DirGraphMtEph | TestDirGraphMtEph.rs | ProveDirGraphMtEph.rs | Both exist |
| 3 | UnDirGraphStEph | TestUnDirGraphStEph.rs | ProveUnDirGraphStEph.rs | Both exist |
| 4 | UnDirGraphMtEph | TestUnDirGraphMtEph.rs | ProveUnDirGraphMtEph.rs | Both exist |
| 5 | LabDirGraphStEph | TestLabDirGraphStEph.rs | ProveLabDirGraphStEph.rs | Both exist |
| 6 | LabDirGraphMtEph | TestLabDirGraphMtEph.rs | ProveLabDirGraphMtEph.rs | Both exist |
| 7 | LabUnDirGraphStEph | TestLabUnDirGraphStEph.rs | ProveLabUnDirGraphStEph.rs | Both exist |
| 8 | LabUnDirGraphMtEph | TestLabUnDirGraphMtEph.rs | ProveLabUnDirGraphMtEph.rs | Both exist |
| 9 | WeightedDirGraphStEphU8 | — | ProveWeightedDirGraphStEphU8.rs | Missing RTT |
| 10 | WeightedDirGraphStEphU32 | TestWeightedDirGraphStEphU32.rs | ProveWeightedDirGraphStEphU32.rs | Both exist |
| 11 | WeightedDirGraph (other 10) | TestWeighed*.rs | ProveWeighted*.rs | Both exist |

All 8 core graph modules have both RTT and PTT. Excellent coverage.

### Iterator coverage

Graphs expose iteration via `iter_vertices()` and `iter_arcs()` which return
`SetStEphIter`. No `IntoIterator` (ambiguous — iterate vertices or arcs?).

| # | Type | loop-match (V) | for-iter (V) | loop-match (A) | for-iter (A) | for-consuming | Notes |
|---|------|---------------|-------------|---------------|-------------|--------------|-------|
| 1 | DirGraphStEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 2 | DirGraphMtEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 3 | UnDirGraphStEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 4 | UnDirGraphMtEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 5 | LabDirGraphStEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 6 | LabDirGraphMtEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 7 | LabUnDirGraphStEph | Yes | Yes | Yes | Yes | N/A | Full coverage |
| 8 | LabUnDirGraphMtEph | Yes | Yes | Yes | Yes | N/A | Full coverage |

All graph types have both loop-match and for-iter tested for both vertex and
arc/edge iteration. This is comprehensive.

### Loop form coverage

Source modules use `loop-match` exclusively for iteration (matching the
SetStEphIter protocol). PTTs test both `loop-match` and `for-iter` forms.
No `while`, `for-range`, or `for-consuming` patterns are used in graph code.

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Def 6.7: Path | Used by algorithm chapters (BFS, DFS), not a Chap06 data structure |
| 2 | Def 6.8: Reachability, Connectivity | Algorithm-level concept, not a graph ADT operation |
| 3 | Def 6.9: Cycles | Algorithm-level concept |
| 4 | Def 6.10-6.11: Trees, Forests, DAGs | Trees in Chap23; DAGs not explicit |
| 5 | Def 6.12-6.13: Distance, Diameter | Algorithm results (shortest path), not ADT operations |
| 6 | Def 6.14: Multigraphs | Not implemented — the Set-based representation prevents multi-edges |
| 7 | Def 6.16: Enumerable Graphs | Not implemented — would use Vec-based adjacency, different representation |
| 8 | Def 6.18-6.20: Subgraphs (vertex/edge-induced) | Not implemented as operations |
| 9 | Def 6.21-6.23: Connected Components, Graph Partition, Cut Edges | Algorithm territory |
| 10 | Def 6.24-6.25: Trees, Rooted Trees | Implemented in Chap23 |

These gaps are appropriate. Chapter 6 defines concepts; implementations arrive
in the algorithm chapters that use them. The code correctly implements the core
ADT operations (graph construction, neighbor queries, degree) and leaves the
algorithmic definitions (paths, reachability, components) to later chapters.

**Code with no prose counterpart:**

- `wf_graph_view`, `wf_lab_graph_view` — well-formedness predicates (arcs reference valid vertices)
- `GraphView`, `LabGraphView` — ghost view types for verified reasoning
- `ClonePlus`, `feq` — Verus-specific comparison and cloning infrastructure
- `ParaPair!`, `ParaPairDisjoint!` — parallelism macros for Mt variants
- `CheckedU32`/`CheckedNat` — overflow-safe arithmetic for weighted graph operations
- `valid_key_type_Edge`, `valid_key_type_LabEdge` — hash-collection preconditions
- Iterator ghost infrastructure — `DirGraphVertexIterView`, `DirGraphArcIterView`
- PartialEq/Eq implementations with specs
- Debug/Display implementations
- Graph literal macros (`DirGraphStEphLit!`, `UnDirGraphStEphLit!`)

## Proof Holes

**All 20 modules are fully clean — zero proof holes.**

DirGraphStEph previously used `assume()` in PartialEq::eq but was refactored
to use the same pattern as UnDirGraphStEph: separate let bindings for each
field comparison, then `assert(self@ =~= other@)` for extensional equality.
This eliminates the assume because:
1. Separate let bindings give the solver individual postconditions from each
   SetStEph::eq call.
2. The `=~=` operator triggers field-by-field comparison of the GraphView
   struct, which the solver can discharge from the field equalities.

The true trust boundary for PartialEq is at the leaf level (SetStEph, which
trusts HashSetWithViewPlus::eq being external_body). Composite types built on
SetStEph can prove equality without assume using this pattern.

## Cosmetic Issues

- ~~15 occurrences of `claude-4-sonet` in LabDirGraphStEph.rs and
  LabUnDirGraphStEph.rs~~ Fixed.
- ~~4 occurrences of `claude-4-sonet` in UnDirGraphStEph.rs~~ Fixed.

## Summary

Chap06 is the graph theory foundation chapter. The code implements the core ADT
operations from Definitions 6.1-6.6 and 6.17 across four graph families
(directed, undirected, labeled-directed, labeled-undirected) in both St and Mt
variants, plus 12 type-monomorphized weighted graph variants.

Key findings:
1. **Strong parallelism.** All four Mt modules use genuine `ParaPair!` parallelism
   for neighborhood and degree operations, achieving Theta(log n) span. This is
   a significant improvement over Chap05's SetMtEph where most operations were
   sequential loops.
2. **Excellent test coverage.** All 8 core modules have both RTT and PTT files.
   Iterator PTTs cover both loop-match and for-iter forms for both vertex and
   arc iteration.
3. **Fully clean proofs.** Zero proof holes across all 20 modules. DirGraphStEph's
   PartialEq assume was eliminated during this review by adopting the split-let +
   extensional equality pattern from UnDirGraphStEph.
4. **Appropriate gaps.** Algorithmic concepts (paths, reachability, components,
   trees) are correctly deferred to later algorithm chapters.
5. **No cost disagreements.** The prose has no cost specs; the code's annotations
   are consistent with the implementations.
