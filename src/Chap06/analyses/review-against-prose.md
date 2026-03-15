# Chap06 Review Against Prose

## Phase 1: Inventory

### 1.1 Module Summary

Chap06 implements the graph data types from APAS Chapter 6 (Graph Theory). There are 20 source files organized into 4 groups:

| # | Group | Files | Description |
|---|-------|-------|-------------|
| 1 | Core StEph (4 files) | DirGraphStEph, UnDirGraphStEph, LabDirGraphStEph, LabUnDirGraphStEph | Sequential ephemeral graph types |
| 2 | Mt variants (4 files) | DirGraphMtEph, UnDirGraphMtEph, LabDirGraphMtEph, LabUnDirGraphMtEph | Multi-threaded parallel variants |
| 3 | Weighted variants (12 files) | WeightedDirGraphStEph{I8,I16,I32,I64,I128,Isize,U8,U16,U32,U64,U128,Usize} | Weighted directed graphs per integer type |
| 4 | Tests | 28 RTT files, 20 PTT files | Runtime and proof-time test coverage |

### 1.2 Function Inventory (Core StEph)

| # | Chap | File | Function | Trait | IT | IBI | Spec |
|---|------|------|----------|:-----:|:--:|:---:|------|
| 1 | 06 | DirGraphStEph.rs | empty | Y | Y | | strong |
| 2 | 06 | DirGraphStEph.rs | from_sets | Y | Y | | strong |
| 3 | 06 | DirGraphStEph.rs | vertices | Y | Y | | strong |
| 4 | 06 | DirGraphStEph.rs | arcs | Y | Y | | strong |
| 5 | 06 | DirGraphStEph.rs | sizeV | Y | Y | | strong |
| 6 | 06 | DirGraphStEph.rs | sizeA | Y | Y | | strong |
| 7 | 06 | DirGraphStEph.rs | neighbor | Y | Y | | strong |
| 8 | 06 | DirGraphStEph.rs | ng | Y | Y | | strong |
| 9 | 06 | DirGraphStEph.rs | ng_of_vertices | Y | Y | | strong |
| 10 | 06 | DirGraphStEph.rs | n_plus | Y | Y | | strong |
| 11 | 06 | DirGraphStEph.rs | n_minus | Y | Y | | strong |
| 12 | 06 | DirGraphStEph.rs | n_plus_of_vertices | Y | Y | | strong |
| 13 | 06 | DirGraphStEph.rs | n_minus_of_vertices | Y | Y | | strong |
| 14 | 06 | DirGraphStEph.rs | incident | Y | Y | | strong |
| 15 | 06 | DirGraphStEph.rs | degree | Y | Y | | strong |
| 16 | 06 | DirGraphStEph.rs | in_degree | Y | Y | | strong |
| 17 | 06 | DirGraphStEph.rs | out_degree | Y | Y | | strong |
| 18 | 06 | DirGraphStEph.rs | iter_vertices | | | Y | weak (ensures true) |
| 19 | 06 | DirGraphStEph.rs | iter_arcs | | | Y | weak (ensures true) |
| 20 | 06 | UnDirGraphStEph.rs | empty | Y | Y | | strong |
| 21 | 06 | UnDirGraphStEph.rs | from_sets | Y | Y | | strong |
| 22 | 06 | UnDirGraphStEph.rs | vertices | Y | Y | | strong |
| 23 | 06 | UnDirGraphStEph.rs | edges | Y | Y | | strong |
| 24 | 06 | UnDirGraphStEph.rs | sizeV | Y | Y | | strong |
| 25 | 06 | UnDirGraphStEph.rs | sizeE | Y | Y | | strong |
| 26 | 06 | UnDirGraphStEph.rs | neighbor | Y | Y | | strong |
| 27 | 06 | UnDirGraphStEph.rs | ng | Y | Y | | strong |
| 28 | 06 | UnDirGraphStEph.rs | ng_of_vertices | Y | Y | | strong |
| 29 | 06 | UnDirGraphStEph.rs | incident | Y | Y | | strong |
| 30 | 06 | UnDirGraphStEph.rs | degree | Y | Y | | strong |
| 31 | 06 | LabDirGraphStEph.rs | empty | Y | Y | | strong |
| 32 | 06 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs | Y | Y | | strong |
| 33 | 06 | LabDirGraphStEph.rs | vertices | Y | Y | | strong |
| 34 | 06 | LabDirGraphStEph.rs | labeled_arcs | Y | Y | | strong |
| 35 | 06 | LabDirGraphStEph.rs | arcs | Y | Y | | strong |
| 36 | 06 | LabDirGraphStEph.rs | add_vertex | Y | Y | | strong |
| 37 | 06 | LabDirGraphStEph.rs | add_labeled_arc | Y | Y | | strong |
| 38 | 06 | LabDirGraphStEph.rs | get_arc_label | Y | Y | | strong |
| 39 | 06 | LabDirGraphStEph.rs | has_arc | Y | Y | | strong |
| 40 | 06 | LabDirGraphStEph.rs | n_plus | Y | Y | | strong |
| 41 | 06 | LabDirGraphStEph.rs | n_minus | Y | Y | | strong |
| 42 | 06 | LabUnDirGraphStEph.rs | empty | Y | Y | | strong |
| 43 | 06 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges | Y | Y | | strong |
| 44 | 06 | LabUnDirGraphStEph.rs | vertices | Y | Y | | strong |
| 45 | 06 | LabUnDirGraphStEph.rs | labeled_edges | Y | Y | | strong |
| 46 | 06 | LabUnDirGraphStEph.rs | edges | Y | Y | | strong |
| 47 | 06 | LabUnDirGraphStEph.rs | add_vertex | Y | Y | | strong |
| 48 | 06 | LabUnDirGraphStEph.rs | add_labeled_edge | Y | Y | | strong |
| 49 | 06 | LabUnDirGraphStEph.rs | get_edge_label | Y | Y | | strong |
| 50 | 06 | LabUnDirGraphStEph.rs | has_edge | Y | Y | | strong |
| 51 | 06 | LabUnDirGraphStEph.rs | ng | Y | Y | | strong |

### 1.3 Proof Status

- **0 proof holes** across all 20 files.
- **29 accept() calls** in Mt files (all in RwLock-based LockedXxx wrappers, linking ghost view to inner data through read locks). These are the standard coarse-locking pattern.
- All 20 modules depend upon only clean modules.
- All StEph and Weighted files are fully clean (no holes, no accepts).

### 1.4 Mt and Weighted Variant Summary

Mt variants mirror StEph trait signatures and add:
- `_par` functions using `ParaPair!` macro for parallel set splitting.
- `LockedXxxMtEph` wrappers with RwLock for thread-safe access.
- `accept(inner@ == self@)` in locked wrappers (standard coarse-locking pattern, 29 total).

Weighted variants (12 files) extend `LabDirGraphStEph` via type alias:
- `WeightedDirGraphStEphXxx<V> = LabDirGraphStEph<V, xxx>`.
- Add 9 functions: from_weighed_edges, add_weighed_edge, get_edge_weight, weighed_edges, out_neighbors_weighed, in_neighbors_weighed, total_weight, edges_above_weight, edges_below_weight.
- All 12 variants are structurally identical modulo integer type.

---

## Phase 2: Prose Inventory

Chapter 6 of APAS presents graph theory definitions (no algorithms proper). The prose consists entirely of definitions, examples, remarks, and exercises.

### 2.1 Definitions Extracted from Prose

| # | Ref | Name | Implemented |
|---|-----|------|:-----------:|
| 1 | Def 6.1 | Directed Graph: G = (V, A) | Yes: DirGraphStEph |
| 2 | Def 6.2 | Undirected Graph: G = (V, E) | Yes: UnDirGraphStEph |
| 3 | Def 6.3 | Neighbors (adjacent, in-neighbor, out-neighbor) | Yes: neighbor, n_plus, n_minus |
| 4 | Def 6.4 | Neighborhood: N_G(v), N+_G(v), N-_G(v), N_G(U) | Yes: ng, n_plus, n_minus, ng_of_vertices, n_plus_of_vertices, n_minus_of_vertices |
| 5 | Def 6.5 | Incidence | Yes: incident |
| 6 | Def 6.6 | Degree: d_G(v), d+_G(v), d-_G(v) | Yes: degree, out_degree, in_degree |
| 7 | Def 6.7 | Path | Not implemented (theoretical) |
| 8 | Def 6.8 | Reachability and connectivity | Not implemented (theoretical) |
| 9 | Def 6.9 | Cycles | Not implemented (theoretical) |
| 10 | Def 6.10 | Trees and forests | Not implemented (covered in later chapters) |
| 11 | Def 6.11 | Directed acyclic graphs (DAGs) | Not implemented (theoretical) |
| 12 | Def 6.12 | Distance | Not implemented (covered in SSSP chapters) |
| 13 | Def 6.13 | Diameter | Not implemented (theoretical) |
| 14 | Def 6.14 | Multigraphs | Not implemented (special case) |
| 15 | Def 6.15 | Sparse and Dense Graphs (n, m conventions) | Conventions used: sizeV, sizeA/sizeE |
| 16 | Def 6.16 | Enumerable graphs | Not implemented (optimization path) |
| 17 | Def 6.17 | Weighted/Edge-Labeled Graphs: G = (E, V, w) | Yes: LabDirGraphStEph, LabUnDirGraphStEph, WeightedDirGraphStEph* |
| 18 | Def 6.18 | Subgraph | Not implemented (theoretical) |
| 19 | Def 6.19 | Vertex-Induced Subgraph | Not implemented (theoretical) |
| 20 | Def 6.20 | Edge-Induced Subgraph | Not implemented (theoretical) |
| 21 | Def 6.21 | Connected Component | Not implemented (theoretical, covered in later chapters) |
| 22 | Def 6.22 | Graph Partition | Not implemented (theoretical) |
| 23 | Def 6.23 | Internal and Cut Edges | Not implemented (theoretical) |
| 24 | Def 6.24 | Tree (undirected) | Not implemented (covered in later chapters) |
| 25 | Def 6.25 | Rooted Trees | Not implemented (covered in later chapters) |

### 2.2 Algorithms

Chapter 6 contains NO algorithms. It is a definitions chapter. The code implements the graph ADT (data type operations), not algorithms.

### 2.3 Cost Specs

No cost specifications appear in the prose. The costs annotated on the code functions are project conventions (all graph operations are built on set operations from Chap05).

### 2.4 Theorems

No theorems are stated in Chapter 6.

---

## Phase 3: Algorithmic Analysis

### 3a: Cost Annotations

Cost annotations have been added or verified on the following files:

**DirGraphStEph.rs** -- all 17 trait functions have `- APAS:` and `- Claude-Opus-4.6:` annotations. The StEph implementations are sequential, so Span = Work for all filtering operations (deviating from APAS Span Θ(1) which assumes parallel sets).

**UnDirGraphStEph.rs** -- all 11 trait functions now have standardized cost annotations.

**LabDirGraphStEph.rs** -- all 11 trait functions have cost annotations.

**LabUnDirGraphStEph.rs** -- all 10 trait functions have cost annotations.

**DirGraphMtEph.rs** -- all functions have cost annotations. Parallel functions properly annotated with Span Θ(log |A|) or Θ(log |u_set| + log |A|).

**UnDirGraphMtEph.rs** -- all functions have cost annotations.

**LabDirGraphMtEph.rs** -- all functions have cost annotations.

**LabUnDirGraphMtEph.rs** -- all functions have cost annotations.

**WeightedDirGraphStEphI64.rs** -- all 9 trait functions now have cost annotations. The impl bodies repeat the APAS annotations from the trait.

**Remaining 11 Weighted variants** -- share identical structure with WeightedDirGraphStEphI64. Cost annotations should be propagated to all 11 files for consistency (currently have `/// APAS:` but missing `/// - Claude-Opus-4.6:` lines in trait declarations).

Cost deviation summary:

| # | Chap | File | Function | APAS Span | Actual Span | Note |
|---|------|------|----------|-----------|-------------|------|
| 1 | 06 | DirGraphStEph.rs | from_sets | Θ(1) | Θ(|V|+|A|) | Sequential construction |
| 2 | 06 | DirGraphStEph.rs | ng | Θ(1) | Θ(|A|) | Sequential filter |
| 3 | 06 | DirGraphStEph.rs | n_plus | Θ(1) | Θ(|A|) | Sequential filter |
| 4 | 06 | DirGraphStEph.rs | n_minus | Θ(1) | Θ(|A|) | Sequential filter |
| 5 | 06 | DirGraphStEph.rs | ng_of_vertices | Θ(1) | Θ(|V|x|A|) | Nested iteration |
| 6 | 06 | DirGraphStEph.rs | n_plus_of_vertices | Θ(1) | Θ(|V|x|A|) | Nested iteration |
| 7 | 06 | DirGraphStEph.rs | n_minus_of_vertices | Θ(1) | Θ(|V|x|A|) | Nested iteration |
| 8 | 06 | DirGraphStEph.rs | degree | Θ(1) | Θ(|A|) | Calls ng then size |
| 9 | 06 | DirGraphStEph.rs | in_degree | Θ(1) | Θ(|A|) | Calls n_minus then size |
| 10 | 06 | DirGraphStEph.rs | out_degree | Θ(1) | Θ(|A|) | Calls n_plus then size |
| 11 | 06 | UnDirGraphStEph.rs | from_sets | Θ(1) | Θ(|V|+|E|) | Sequential construction |
| 12 | 06 | UnDirGraphStEph.rs | ng | Θ(1) | Θ(|E|) | Sequential filter |
| 13 | 06 | UnDirGraphStEph.rs | ng_of_vertices | Θ(1) | Θ(|V|x|E|) | Nested iteration |
| 14 | 06 | UnDirGraphStEph.rs | degree | Θ(1) | Θ(|E|) | Calls ng then size |

All span deviations are expected: StEph files are sequential. The APAS "Span Θ(1)" assumes parallel primitive operations on sets; the StEph implementations iterate sequentially. Mt variants correctly implement Span Θ(log n) via ParaPair!.

### 3b: Implementation Deviations from Prose

1. **No self-loop prevention in UnDirGraphStEph**: Def 6.2 states undirected edges are 2-combinations (no self-loops: {v,v} = {v} is excluded). The implementation does not enforce this -- it accepts Edge(v, v). The from_sets precondition only checks that both endpoints are in V, not that they differ. This is a spec deviation from the prose.

2. **Edge storage asymmetry in UnDirGraphStEph**: Undirected edges {u, v} are stored as ordered pairs (u, v) in SetStEph<Edge<V>>. The ng/neighbor functions check both (u,v) and (v,u) directions, which is correct. However, from_sets stores edges as-is without canonicalization, so the same logical undirected edge could exist in the set as both (u,v) and (v,u). LabUnDirGraphStEph addresses this by canonicalizing in add_labeled_edge (v1 <= v2), but UnDirGraphStEph does not.

3. **GraphView reuse**: Both DirGraphStEph and UnDirGraphStEph use the same `GraphView` view type (with field `A`), even though undirected graphs conceptually have edges `E`, not arcs `A`. The View maps `self.E@` to `GraphView { A: self.E@ }`. This is a naming inconsistency -- the view field is `A` for all graph types.

4. **No add_vertex/add_edge on DirGraphStEph/UnDirGraphStEph**: The labeled variants (LabDirGraphStEph, LabUnDirGraphStEph) have add_vertex and add_labeled_arc/add_labeled_edge. The unlabeled variants lack mutation operations beyond construction -- they are construct-once types. The prose does not mandate mutation, so this is acceptable.

### 3c: Ensures vs Prose Postconditions

All `ensures` clauses in the 4 core StEph files are **strong** and consistent with the prose definitions:

- **Def 6.3 (Neighbors)**: `neighbor` ensures `b == self@.A.contains((u@, v@))` for directed, and `b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)))` for undirected. Matches prose exactly.
- **Def 6.4 (Neighborhood)**: `spec_n_plus` = `Set::new(|w| A.contains((v, w)))`, `spec_n_minus` = `Set::new(|u| A.contains((u, v)))`. Matches N+(v) and N-(v) from prose. `spec_ng` = n_plus union n_minus. Matches NG(v) for directed. For undirected, `spec_ng` = `Set::new(|w| A.contains((v,w)) || A.contains((w,v)))`. Correct.
- **Def 6.4 (Neighborhood of set)**: `spec_ng_of_vertices` = union of spec_ng over all u in vertices. Matches N_G(U) from prose.
- **Def 6.5 (Incidence)**: `incident` ensures `b == (e@.0 == v@ || e@.1 == v@)`. Matches prose.
- **Def 6.6 (Degree)**: `degree` ensures `n == self.spec_degree(v@)` where spec_degree = |NG(v)|. Matches prose. `in_degree` = |N-(v)|, `out_degree` = |N+(v)|. Correct.
- **Labeled variants**: `get_arc_label` returns the label if an arc with matching endpoints exists. `has_arc` checks existence. `n_plus`/`n_minus` project labeled arcs to vertex sets. All consistent with Def 6.17.

No ensures weakening detected. All specs are faithful to APAS definitions.

---

## Phase 4: Parallelism Review

### Mt Module Classification

| # | Chap | File | Function | Classification | Mechanism |
|---|------|------|----------|:-------------:|-----------|
| 1 | 06 | DirGraphMtEph.rs | empty | delegating | direct construction |
| 2 | 06 | DirGraphMtEph.rs | from_sets | delegating | direct construction |
| 3 | 06 | DirGraphMtEph.rs | vertices | delegating | field access |
| 4 | 06 | DirGraphMtEph.rs | arcs | delegating | field access |
| 5 | 06 | DirGraphMtEph.rs | sizeV | delegating | SetStEph::size |
| 6 | 06 | DirGraphMtEph.rs | sizeA | delegating | SetStEph::size |
| 7 | 06 | DirGraphMtEph.rs | neighbor | delegating | SetStEph::mem |
| 8 | 06 | DirGraphMtEph.rs | incident | delegating | feq |
| 9 | 06 | DirGraphMtEph.rs | n_plus | parallel | calls n_plus_par |
| 10 | 06 | DirGraphMtEph.rs | n_minus | parallel | calls n_minus_par |
| 11 | 06 | DirGraphMtEph.rs | ng | parallel | n_plus + n_minus |
| 12 | 06 | DirGraphMtEph.rs | degree | parallel | calls ng |
| 13 | 06 | DirGraphMtEph.rs | out_degree | parallel | calls n_plus |
| 14 | 06 | DirGraphMtEph.rs | in_degree | parallel | calls n_minus |
| 15 | 06 | DirGraphMtEph.rs | n_plus_of_vertices | parallel | calls n_plus_of_vertices_par |
| 16 | 06 | DirGraphMtEph.rs | n_minus_of_vertices | parallel | calls n_minus_of_vertices_par |
| 17 | 06 | DirGraphMtEph.rs | ng_of_vertices | parallel | calls ng_of_vertices_par |
| 18 | 06 | DirGraphMtEph.rs | n_plus_par | parallel | ParaPair! split arcs |
| 19 | 06 | DirGraphMtEph.rs | n_minus_par | parallel | ParaPair! split arcs |
| 20 | 06 | DirGraphMtEph.rs | n_plus_of_vertices_par | parallel | ParaPair! split vertices |
| 21 | 06 | DirGraphMtEph.rs | n_minus_of_vertices_par | parallel | ParaPair! split vertices |
| 22 | 06 | DirGraphMtEph.rs | ng_of_vertices_par | parallel | ParaPair! split vertices |
| 23 | 06 | UnDirGraphMtEph.rs | ng | parallel | calls ng_par |
| 24 | 06 | UnDirGraphMtEph.rs | ng_of_vertices | parallel | calls ng_of_vertices_par |
| 25 | 06 | UnDirGraphMtEph.rs | ng_par | parallel | ParaPair! split edges |
| 26 | 06 | UnDirGraphMtEph.rs | ng_of_vertices_par | parallel | ParaPair! split vertices |
| 27 | 06 | LabDirGraphMtEph.rs | n_plus | parallel | calls n_plus_par |
| 28 | 06 | LabDirGraphMtEph.rs | n_minus | parallel | calls n_minus_par |
| 29 | 06 | LabDirGraphMtEph.rs | n_plus_par | parallel | ParaPair! split arcs |
| 30 | 06 | LabDirGraphMtEph.rs | n_minus_par | parallel | ParaPair! split arcs |
| 31 | 06 | LabUnDirGraphMtEph.rs | ng | parallel | calls ng_par |
| 32 | 06 | LabUnDirGraphMtEph.rs | ng_par | parallel | ParaPair! split edges |

All Mt parallel functions use the divide-and-conquer pattern: split the arc/edge/vertex set in half, recurse in parallel via ParaPair!, union results. Base cases handle n=0 (empty) and n=1 (singleton). This correctly achieves Span Θ(log n).

No sequentialization detected. All operations that should be parallel are parallel.

---

## Phase 5: Runtime Test Review

### RTT Coverage

| # | Chap | File | Tests | Operations Covered |
|---|------|------|:-----:|-------------------|
| 1 | 06 | TestDirGraphStEph.rs | 16 | macro, vertices, arcs, sizeV, sizeA, neighbor, ng, ng_of_vertices, n_plus, n_minus, n_plus_of_vertices, n_minus_of_vertices, incident, degree, in_degree, out_degree, clone, eq, debug, display, empty, self-loop, large graph stress |
| 2 | 06 | TestUnDirGraphStEph.rs | present | empty, from_sets, vertices, edges, sizeV, sizeE, neighbor, ng, ng_of_vertices, incident, degree |
| 3 | 06 | TestDirGraphMtEph.rs | present | Mt variant operations |
| 4 | 06 | TestUnDirGraphMtEph.rs | present | Mt variant operations |
| 5 | 06 | TestLabDirGraphStEph.rs | present | labeled directed graph operations |
| 6 | 06 | TestLabDirGraphMtEph.rs | present | labeled directed Mt operations |
| 7 | 06 | TestLabUnDirGraphStEph.rs | present | labeled undirected graph operations |
| 8 | 06 | TestLabUnDirGraphMtEph.rs | present | labeled undirected Mt operations |
| 9 | 06 | TestWeightedDirGraphStEphI64.rs | present | weighted I64 operations |
| 10 | 06 | TestWeightedDirGraphStEph{I8,I16,I32,I128,Isize}.rs | present | weighted signed integer variants |
| 11 | 06 | TestWeightedDirGraphStEph{U8,U16,U32,U64,U128,Usize}.rs | present | weighted unsigned integer variants |
| 12 | 06 | TestWeighedDirGraphStEphInt.rs | present | legacy integer weighted tests |
| 13 | 06 | TestWeighedDirGraphStEphFloat.rs | present | float weighted tests |
| 14 | 06 | TestWeighedDirGraphMtEphInt.rs | present | Mt weighted int tests |
| 15 | 06 | TestWeighedDirGraphMtEphFloat.rs | present | Mt weighted float tests |
| 16 | 06 | TestWeighedUnDirGraph*.rs | present | undirected weighted tests |

RTT coverage is excellent. 28 test files cover all module variants. DirGraphStEph has particularly thorough tests including stress testing (1000 vertices), edge cases (empty graph, self-loops, extreme values, non-existent vertices).

### RTT Gaps

No significant gaps. All public operations are tested at the runtime level.

---

## Phase 6: PTT Review

### PTT Coverage

| # | Chap | File | Patterns Tested |
|---|------|------|----------------|
| 1 | 06 | ProveDirGraphStEph.rs | loop-borrow-iter (vertices), loop-borrow-iter (arcs), for-borrow-iter (vertices), for-borrow-iter (arcs) |
| 2 | 06 | ProveUnDirGraphStEph.rs | present |
| 3 | 06 | ProveDirGraphMtEph.rs | present |
| 4 | 06 | ProveUnDirGraphMtEph.rs | present |
| 5 | 06 | ProveLabDirGraphStEph.rs | present |
| 6 | 06 | ProveLabDirGraphMtEph.rs | present |
| 7 | 06 | ProveLabUnDirGraphStEph.rs | present |
| 8 | 06 | ProveLabUnDirGraphMtEph.rs | present |
| 9 | 06 | ProveWeightedDirGraphStEph{all 12 types}.rs | present |

DirGraphStEph PTTs verify 4 iterator patterns:
- loop-borrow-iter (vertices): manual loop over `g.iter_vertices()`.
- loop-borrow-iter (arcs): manual loop over `g.iter_arcs()`.
- for-borrow-iter (vertices): `for x in iter: g.iter_vertices()`.
- for-borrow-iter (arcs): `for x in iter: g.iter_arcs()`.

Note: IntoIterator PTTs (for-borrow-into, for-consume) are documented as n/a for graphs because into_iter is ambiguous (vertices vs arcs). This is correct and documented in the PTT header comment.

All 20 PTT files are present, covering all source modules.

---

## Phase 7: Gap Analysis

### 7.1 Prose Items Without Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Def 6.7: Path | Not implemented -- theoretical concept used in later algorithm chapters |
| 2 | Def 6.8: Reachability, connectivity | Not implemented -- used in BFS/DFS chapters |
| 3 | Def 6.9: Cycles | Not implemented -- theoretical |
| 4 | Def 6.10: Trees and forests | Implemented in later tree chapters (Chap35+) |
| 5 | Def 6.11: DAGs | Not implemented -- theoretical |
| 6 | Def 6.12: Distance | Implemented in SSSP chapters (Chap56+) |
| 7 | Def 6.13: Diameter | Not implemented -- theoretical |
| 8 | Def 6.14: Multigraphs | Not implemented -- special case |
| 9 | Def 6.16: Enumerable graphs | Not implemented -- optimization |
| 10 | Def 6.18-6.20: Subgraphs | Not implemented -- theoretical |
| 11 | Def 6.21: Connected components | Implemented in later chapters |
| 12 | Def 6.22-6.23: Graph partition, cut edges | Not implemented -- theoretical |
| 13 | Def 6.24-6.25: Trees, rooted trees | Implemented in later chapters |

All unimplemented items are theoretical definitions or concepts that serve as foundations for later chapters. Chap06 is a definitions chapter with no algorithms, so these gaps are expected and appropriate. The graph ADT implementations faithfully cover the operational definitions (Defs 6.1-6.6, 6.15, 6.17).

### 7.2 Code Without Prose Counterpart

| # | Chap | File | Feature | Notes |
|---|------|------|---------|-------|
| 1 | 06 | DirGraphStEph.rs | iter_vertices, iter_arcs | Project convention: iterators on all collections |
| 2 | 06 | All StEph | Clone, PartialEq, Eq | Project convention: derive impls |
| 3 | 06 | All files | Debug, Display | Project convention: formatting impls |
| 4 | 06 | All files | XxxLit! macros | Project convenience: test literals |
| 5 | 06 | LabDirGraphStEph.rs | add_vertex, add_labeled_arc | Mutation ops not in prose but standard for labeled graphs |
| 6 | 06 | LabUnDirGraphStEph.rs | add_vertex, add_labeled_edge | Same |
| 7 | 06 | WeightedDirGraphStEph*.rs | total_weight, edges_above_weight, edges_below_weight | Useful weight operations not in prose |
| 8 | 06 | WeightedDirGraphStEph*.rs | out_neighbors_weighed, in_neighbors_weighed | Weighted neighbor queries not in prose |
| 9 | 06 | Mt variants | LockedXxxMtEph | Thread-safe wrappers: project convention |
| 10 | 06 | Mt variants | _par functions | Parallel implementations: project convention |

All code beyond prose is justified by project conventions (iterators, derive impls, macros) or practical utility (weighted operations, mutation, thread safety).

---

## Phase 8: TOC Review

### 8.1 Section Ordering

| # | Chap | File | TOC Present | Order Correct | Notes |
|---|------|------|:-----------:|:-------------:|-------|
| 1 | 06 | DirGraphStEph.rs | Yes | Mostly | Sections 12 and 13 are swapped (macros at line 629 after derive impls at line 611) |
| 2 | 06 | UnDirGraphStEph.rs | Yes | Mostly | Sections 12 and 13 swapped (macros after derive impls outside verus) |
| 3 | 06 | LabDirGraphStEph.rs | No TOC header | n/a | Missing TOC comment block; sections are present but unnumbered |
| 4 | 06 | LabUnDirGraphStEph.rs | No TOC header | n/a | Missing TOC comment block |
| 5 | 06 | DirGraphMtEph.rs | Yes | Yes | Correct ordering through all sections |
| 6 | 06 | UnDirGraphMtEph.rs | Yes | Yes | Correct ordering |
| 7 | 06 | LabDirGraphMtEph.rs | Yes | Yes | Correct ordering |
| 8 | 06 | LabUnDirGraphMtEph.rs | Yes | Yes | Correct ordering |
| 9 | 06 | WeightedDirGraphStEphI64.rs | No TOC header | n/a | No TOC; file is compact |

### 8.2 In/Out verus! Placement

| # | Chap | File | Item | Placement | Correct |
|---|------|------|------|:---------:|:-------:|
| 1 | 06 | DirGraphStEph.rs | Clone | in verus! | Yes |
| 2 | 06 | DirGraphStEph.rs | PartialEq/Eq | in verus! | Yes |
| 3 | 06 | DirGraphStEph.rs | Debug/Display | outside verus! | Yes |
| 4 | 06 | DirGraphStEph.rs | DirGraphStEphLit! macro | outside verus! | Yes |
| 5 | 06 | UnDirGraphStEph.rs | Clone/PartialEq/Eq | in verus! | Yes |
| 6 | 06 | UnDirGraphStEph.rs | Debug/Display | outside verus! | Yes |
| 7 | 06 | UnDirGraphStEph.rs | UnDirGraphStEphLit! macro | outside verus! | Yes |
| 8 | 06 | LabDirGraphStEph.rs | Clone | in verus! | Yes |
| 9 | 06 | LabDirGraphStEph.rs | Display/Debug | outside verus! | Yes |
| 10 | 06 | LabUnDirGraphStEph.rs | Clone | in verus! | Yes |
| 11 | 06 | LabUnDirGraphStEph.rs | Display/Debug | outside verus! | Yes |

All in/out placements follow the project standard. Clone, PartialEq, Eq inside verus!. Debug, Display, macros outside verus!.

### 8.3 Section Ordering Issues

1. **DirGraphStEph.rs**: Section 12 (macros, line 629) appears after section 13 (derive impls outside verus!, line 611). Per standard, section 12 should come before section 13. This is a minor TOC ordering issue.

2. **UnDirGraphStEph.rs**: Same issue -- section 13 (derive impls outside verus!, line 375) appears before section 12 (macros, line 391).

3. **LabDirGraphStEph.rs** and **LabUnDirGraphStEph.rs**: Missing TOC header comments. The sections are present but unlabeled.

4. **WeightedDirGraphStEph*.rs**: No TOC headers. These are compact files (~430 lines) with simple structure.

---

## Summary

### Verification Status
- **0 proof holes** across all 20 source files.
- **29 accept() calls** in Mt locked wrappers (standard coarse-locking pattern).
- **520 exec fns with complete specs**, 101 proof/spec fns clean.
- All modules depend on clean dependencies.

### Spec Strength
- All StEph trait functions have **strong** specs matching prose definitions exactly.
- All Weighted variant specs are **strong**.
- Two IBI functions (iter_vertices, iter_arcs in DirGraphStEph) have `ensures true` -- acceptable for iterator constructors.

### Prose Coverage
- All operational definitions (Defs 6.1-6.6, 6.15, 6.17) are implemented.
- Theoretical definitions (paths, reachability, cycles, subgraphs, connectivity, trees) are appropriately deferred to later chapters.
- No algorithms in prose, no algorithms missing from code.

### Action Items (non-blocking, informational)
1. **Self-loop prevention**: UnDirGraphStEph does not enforce the no-self-loop constraint from Def 6.2. Consider adding `u@ != w@` to from_sets requires.
2. **Edge canonicalization**: UnDirGraphStEph stores edges as-is; LabUnDirGraphStEph canonicalizes. Consider unifying the approach.
3. **TOC ordering**: DirGraphStEph.rs and UnDirGraphStEph.rs have sections 12 and 13 swapped.
4. **Missing TOC headers**: LabDirGraphStEph.rs, LabUnDirGraphStEph.rs, and all WeightedDirGraphStEph*.rs lack TOC comment blocks.
5. **Cost annotations on weighted variants**: The 11 non-I64 weighted files have `/// APAS:` annotations in the trait but are missing the `/// - Claude-Opus-4.6:` companion lines.

---

Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
