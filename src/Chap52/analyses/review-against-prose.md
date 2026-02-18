<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 52: Graphs and Their Representation — Review Against Prose

- **Date**: 2026-02-17
- **Reviewer**: Claude-Opus-4.6
- **Project**: APAS-VERUS-agent2
- **Prose Source**: `prompts/Chap52.txt` (Chapter 52 of APAS textbook)

## Phase 1: Summary

All 14 source files in `src/Chap52/` have been batch-verusified: wrapped in `verus!` blocks, all exec functions marked `#[verifier::external_body]`, `View` impls added.

| Metric | Value |
|---|---|
| Source files | 14 |
| Total `external_body` proof holes | 146 |
| Other proof holes (`assume`, `admit`) | 0 |
| Verified functions | 0 |
| Test files | 13 |

### Gating

| # | Module | cfg gating | Notes |
|---|---|---|---|
| 1 | AdjSeqGraph{StEph,StPer,MtEph,MtPer} | `not(feature = "experiments_only")` | Available in both Verus and cargo |
| 2 | AdjMatrixGraph{StEph,StPer,MtEph,MtPer} | `not(feature = "experiments_only")` | Available in both Verus and cargo |
| 3 | EdgeSetGraph{StEph,StPer} | `not(verus_keep_ghost)` | Cargo only — depends on Chap37/41 AVL trees |
| 4 | EdgeSetGraphMtPer | `not(verus_keep_ghost)` + `feature = "all_chapters"` | Cargo only — depends on Chap37/41 AVL trees |
| 5 | AdjTableGraph{StEph,StPer} | `not(verus_keep_ghost)` | Cargo only — depends on Chap37/41/43 |
| 6 | AdjTableGraphMtPer | `not(verus_keep_ghost)` + `feature = "all_chapters"` | Cargo only — depends on Chap37/41/43 |

EdgeSetGraph and AdjTableGraph modules are gated with `#[cfg(not(verus_keep_ghost))]` because they depend on unverified chapters: Chap37 (AVLTreeSeq), Chap41 (AVLTreeSet), and Chap43 (OrderedTable/HashTable). These modules are excluded from Verus verification but are available for `cargo test`.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Section | Description |
|---|---|---|---|
| 1 | Edge Set Representation | 3.1 | G = (V set, (V × V) set) |
| 2 | Adjacency Table Representation | 3.2 (Def 52.2) | G = (V × (V set)) table |
| 3 | Adjacency Sequence Representation | 3.3 (Def 52.4) | G = (int seq) seq for enumerable graphs |
| 4 | Adjacency Matrix Representation | 3.4 | G = (bool seq) seq |
| 5 | Weighted Graphs | 3.5 | Label tables, edge-weight pairs |

### Cost Specifications

| # | Cost Spec | Representation | Model |
|---|---|---|---|
| 1 | 52.1 | Edge Sets | Tree-based sets |
| 2 | 52.3 | Adjacency Tables | Tree-based tables + sets |
| 3 | 52.5 | Adjacency Sequences | (Persistent) array-sequences |
| 4 | 52.6 | Adjacency Matrix | Array-sequences |

### Exercises

| # | Exercise | Topic | Implemented? |
|---|---|---|---|
| 1 | 52.1 | Prove O(lg m) = O(lg n) | No (text proof) |
| 2 | 52.2 | Cost of deleting vertex with out-degree d | No (text answer) |
| 3 | 52.3 | Cost of finding out-neighbors (adj seq) | No (text answer) |
| 4 | 52.4 | Why mapping over edges requires Ω(n) work | No (text answer) |
| 5 | 52.5 | Constant-span edge deletion algorithm | No |
| 6 | 52.6 | Constant-span graph complement | Yes — `complement()` in AdjMatrix files |

## Phase 3: Cost Annotations Summary

All 146 `external_body` exec functions now carry two-line cost annotations:
```
/// - APAS: Work Θ(...), Span Θ(...)
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — [reason]
```

Additionally, 3 `Debug::fmt` functions (outside `verus!`) and 2 `Default::default` functions have annotations. The trait declarations retain old-format single-line `/// claude-4-sonet:` annotations from a previous session.

### Implementation Fidelity — Cost Spec 52.1 (Edge Sets, tree-based)

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|---|---|---|---|---|---|
| 1 | has_edge | Θ(lg n) | Θ(lg n) | Θ(lg m) | Θ(lg m) | ✓ (lg m = O(lg n)) |
| 2 | out_neighbors | Θ(m) | Θ(lg n) | Θ(m) | Θ(m) | Work ✓, Span ✗ (sequential) |
| 3 | out_degree | Θ(m) | Θ(lg n) | Θ(m) | Θ(m) | Work ✓, Span ✗ (sequential) |
| 4 | insert_vertex | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 5 | delete_vertex (isolated) | Θ(lg n) | Θ(lg n) | Θ(m) | Θ(m) | ✗ (code removes all edges, APAS assumes isolated) |
| 6 | insert_edge | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 7 | delete_edge | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |

### Implementation Fidelity — Cost Spec 52.3 (Adjacency Tables, tree-based)

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|---|---|---|---|---|---|
| 1 | has_edge | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 2 | out_neighbors | Θ(lg n + d(v)) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 3 | out_degree | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 4 | insert_vertex | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 5 | delete_vertex (isolated) | Θ(lg n) | Θ(lg n) | Θ(n lg n) | Θ(n lg n) | ✗ (code iterates all vertices) |
| 6 | insert_edge | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |
| 7 | delete_edge | Θ(lg n) | Θ(lg n) | Θ(lg n) | Θ(lg n) | ✓ |

### Implementation Fidelity — Cost Spec 52.5 (Adjacency Sequences, array-based)

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|---|---|---|---|---|---|
| 1 | Map over vertices | Θ(n) | Θ(1) | N/A | N/A | Not directly exposed |
| 2 | Map over edges | Θ(n + m) | Θ(1) | Θ(n + m) | Θ(n + m) | Work ✓, Span ✗ (sequential) |
| 3 | out_neighbors | Θ(d(v)) or Θ(1) | Θ(1) | Θ(1) | Θ(1) | ✓ |
| 4 | out_degree | Θ(1) | Θ(1) | Θ(1) | Θ(1) | ✓ |
| 5 | has_edge | Θ(d(u)) | Θ(lg d(u)) | Θ(d(u)) | Θ(d(u)) | Work ✓, Span ✗ (linear scan) |
| 6 | insert_edge | Θ(n) | Θ(1) | Θ(n + d(u)) | Θ(n + d(u)) | Work ≈, Span ✗ (sequential) |
| 7 | delete_edge | Θ(n) | Θ(1) | Θ(n + d(u)) | Θ(n + d(u)) | Work ≈, Span ✗ (sequential) |

### Implementation Fidelity — Cost Spec 52.6 (Adjacency Matrix, array-based)

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|---|---|---|---|---|---|
| 1 | Map over edges | Θ(n²) | Θ(1) | Θ(n²) | Θ(n²) | Work ✓, Span ✗ (sequential) |
| 2 | out_neighbors | Θ(n) | Θ(1) | Θ(n) | Θ(n) | Work ✓, Span ✗ (sequential) |
| 3 | out_degree | Θ(n) | Θ(lg n) | Θ(n) | Θ(n) | Work ✓, Span ✗ (sequential) |
| 4 | has_edge | Θ(1) | Θ(1) | Θ(1) | Θ(1) | ✓ |
| 5 | insert/delete edge | Θ(n) | Θ(1) | Θ(1) eph / Θ(n) per | Θ(1) eph / Θ(n) per | ✓ (ephemeral better) |
| 6 | complement | Θ(n²) | Θ(1) | Θ(n²) | Θ(n²) St / Θ(lg n) Mt | Work ✓, Span: St ✗, MtPer ✓ |

### Spec Fidelity Notes

1. **View types are minimal.** AdjSeq and AdjMatrix graph views map to `Seq<Seq<int>>` or `Seq<Seq<bool>>`, reflecting the mathematical structure. EdgeSet and AdjTable views map to `Self` (no abstraction), which is adequate for external_body but will need revision when verification begins.

2. **No `requires`/`ensures` on any function.** All functions are `external_body` with no pre/postcondition contracts. This is Phase 1 (batch verusify) status.

3. **`delete_vertex` implementations exceed APAS cost bounds** because the code removes all incident edges rather than assuming the vertex is isolated. The APAS cost specs assume isolated vertex deletion; the implementations handle non-isolated vertices. This is a deliberate implementation choice that is more robust but less efficient.

## Phase 4: Parallelism Review for Mt Modules

| # | Module | True Parallelism? | Operations with Threads | Notes |
|---|---|---|---|---|
| 1 | AdjSeqGraphMtEph | No | None | All loops sequential despite Mt backing type |
| 2 | AdjSeqGraphMtPer | No | None | All loops sequential; `map_vertices` (outside verus!) also sequential |
| 3 | AdjMatrixGraphMtEph | No | None | All loops sequential despite Mt backing type |
| 4 | AdjMatrixGraphMtPer | **Yes** | `num_edges`, `out_neighbors`, `out_degree`, `complement` | Uses `thread::spawn` with divide-and-conquer; 7 parallel helper functions |
| 5 | AdjTableGraphMtPer | No | None | `delete_vertex` has TODO for parallelization; `num_edges` sequential |
| 6 | EdgeSetGraphMtPer | Partial | None explicit | Uses MtPer sets which may parallelize internally (filter, etc.) |

### Parallelism Details — AdjMatrixGraphMtPer

This is the only module with TRUE thread-based parallelism. It implements divide-and-conquer using `thread::spawn`:

- `count_edges_parallel` — splits rows, recurses on halves in parallel
- `count_row_parallel` — splits columns, counts in parallel
- `collect_neighbors_parallel` — parallel filter + append for row neighbors
- `complement_matrix_parallel` / `complement_rows_parallel` / `complement_row_parallel` / `complement_columns_parallel` — full 2D parallel complement

All achieve Work Θ(n² or n), Span Θ(lg n), which matches APAS span bounds.

### Parallelism Gaps

- **AdjSeqGraphMtEph/MtPer**: Should use parallel tabulate/reduce for `num_edges`, parallel inject for `has_edge`. Currently all sequential.
- **AdjMatrixGraphMtEph**: Should mirror MtPer parallelism for read-only operations (`num_edges`, `out_neighbors`, `out_degree`, `complement`). Currently all sequential.
- **AdjTableGraphMtPer**: `delete_vertex` has an explicit TODO comment requesting parallelization. `num_edges` should use parallel reduction.
- **EdgeSetGraphMtPer**: `out_neighbors` uses sequential insert loop; filter operation on MtPer sets may internally parallelize but the insert loop is sequential.

## Phase 5: RTT Review

### Test File Inventory

| # | Test File | Source File | Status |
|---|---|---|---|
| 1 | `TestEdgeSetGraphStEph.rs` | `EdgeSetGraphStEph.rs` | Present |
| 2 | `TestEdgeSetGraphStPer.rs` | `EdgeSetGraphStPer.rs` | Present |
| 3 | `TestEdgeSetGraphMtPer.rs` | `EdgeSetGraphMtPer.rs` | Present |
| 4 | `TestEdgeSetGraphMtEph.rs` | **No source file** | Empty stub (5 lines, no tests) |
| 5 | `TestAdjTableGraphStEph.rs` | `AdjTableGraphStEph.rs` | Present |
| 6 | `TestAdjTableGraphStPer.rs` | `AdjTableGraphStPer.rs` | Present |
| 7 | `TestAdjTableGraphMtPer.rs` | `AdjTableGraphMtPer.rs` | Present |
| 8 | `TestAdjSeqGraphStEph.rs` | `AdjSeqGraphStEph.rs` | Present |
| 9 | `TestAdjSeqGraphStPer.rs` | `AdjSeqGraphStPer.rs` | Present |
| 10 | `TestAdjSeqGraphMtPer.rs` | `AdjSeqGraphMtPer.rs` | Present |
| 11 | `TestAdjMatrixGraphStEph.rs` | `AdjMatrixGraphStEph.rs` | Present |
| 12 | `TestAdjMatrixGraphStPer.rs` | `AdjMatrixGraphStPer.rs` | Present |
| 13 | `TestAdjMatrixGraphMtPer.rs` | `AdjMatrixGraphMtPer.rs` | Present |

### Missing Test Files

| # | Missing Test | Source Exists? | Notes |
|---|---|---|---|
| 1 | TestAdjSeqGraphMtEph | Yes (`AdjSeqGraphMtEph.rs`) | No test file for this module |
| 2 | TestAdjMatrixGraphMtEph | Yes (`AdjMatrixGraphMtEph.rs`) | No test file for this module |
| 3 | TestAdjTableGraphMtEph | No source file | No source or test |
| 4 | TestEdgeSetGraphMtEph | No source file | Test file exists but is an empty stub |

### Missing Source Files

| # | Missing Source | Notes |
|---|---|---|
| 1 | `EdgeSetGraphMtEph.rs` | No MtEph variant for Edge Set representation |
| 2 | `AdjTableGraphMtEph.rs` | No MtEph variant for Adjacency Table representation |

## Phase 6: PTT Review

No PTTs are needed at this stage. All functions are `external_body` with no verified loops, iterators, or proof obligations. PTTs become relevant when `external_body` annotations are removed and verification begins.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Prose Item | Section | Notes |
|---|---|---|---|
| 1 | Weighted Graphs | 3.5 | Label tables, edge-weight adjacency tables/sequences not implemented |
| 2 | Exercise 52.5 | 3.3 | Constant-span edge deletion using inject — not implemented |
| 3 | Map over vertices | All cost specs | No explicit `map_vertices` operation except in AdjSeqGraphMtPer (outside verus!) |
| 4 | Map over edges | All cost specs | No explicit `map_edges` operation exposed |
| 5 | Undirected graph variants | Section 3 preamble | Prose mentions keeping edges in both directions; no explicit undirected support |

### Code With No Prose Counterpart

| # | Function/Type | File(s) | Notes |
|---|---|---|---|
| 1 | `from_seq`, `from_matrix`, `from_vertices_and_edges`, `from_table` | All files | Constructors — Verus scaffolding |
| 2 | `set_neighbors` | AdjSeqGraphStEph | Mutation helper for ephemeral interface |
| 3 | `map_vertices` | AdjSeqGraphMtPer (outside verus!) | Vertex relabeling — useful but not in prose |
| 4 | `Default` impls | EdgeSetGraphMtPer, AdjTableGraphMtPer | Verus scaffolding |
| 5 | `Debug` impls | EdgeSetGraphStPer, AdjSeqGraphStPer, AdjMatrixGraphStPer | Rust debugging support |
| 6 | Parallel helper functions (7) | AdjMatrixGraphMtPer | Implementation detail for achieving prose span bounds |

## Phase 8: TOC / In-Out Table

### Table of Contents Compliance

All 14 files include a Table of Contents comment near the top of the `verus!` block. All files follow the standard section ordering: type definitions → view impls → traits → impls → derive impls.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | EdgeSetGraphStEph | ✅ in | - | - | - | - | - | - | - |
| 2 | EdgeSetGraphStPer | ✅ in | ✅ in (derive) | - | - | - | ✅ out | - | - |
| 3 | EdgeSetGraphMtPer | ✅ in | - | ✅ in | - | - | - | - | - |
| 4 | AdjTableGraphStEph | ✅ in | - | - | - | - | - | - | - |
| 5 | AdjTableGraphStPer | ✅ in | - | - | - | - | - | - | - |
| 6 | AdjTableGraphMtPer | ✅ in | - | ✅ in | - | - | - | - | - |
| 7 | AdjSeqGraphStEph | ✅ in | - | - | - | - | - | - | - |
| 8 | AdjSeqGraphStPer | ✅ in | ✅ in (derive) | - | - | - | ✅ out | - | - |
| 9 | AdjSeqGraphMtEph | ✅ in | - | - | - | - | - | - | - |
| 10 | AdjSeqGraphMtPer | ✅ in | - | - | - | - | - | - | - |
| 11 | AdjMatrixGraphStEph | ✅ in | - | - | - | - | - | - | - |
| 12 | AdjMatrixGraphStPer | ✅ in | ✅ in (derive) | - | - | - | ✅ out | - | - |
| 13 | AdjMatrixGraphMtEph | ✅ in | - | - | - | - | - | - | - |
| 14 | AdjMatrixGraphMtPer | ✅ in | - | - | - | - | - | - | - |

**Notes:**
- All Clone impls use `#[derive(Clone)]` inside `verus!` — correct placement.
- PartialEq/Eq on StPer types use `#[derive(PartialEq, Eq)]` inside `verus!` — correct placement but not using the recommended `PartialEqSpecImpl` pattern. Adequate for Phase 1.
- Debug impls are outside `verus!` — correct placement (Verus doesn't support fmt traits).
- No Display, Drop, Iterator, or Macro impls in any file.

## Action Items

| # | Priority | Action | Effort |
|---|---|---|---|
| 1 | P1 | Add `requires`/`ensures` contracts to all functions and begin removing `external_body` | High — 146 functions |
| 2 | P1 | Implement parallelism in AdjSeqGraphMtEph, AdjSeqGraphMtPer, AdjMatrixGraphMtEph | Medium |
| 3 | P1 | Parallelize `AdjTableGraphMtPer::delete_vertex` and `num_edges` (TODO in source) | Low |
| 4 | P2 | Add View types with proper abstraction (EdgeSet/AdjTable currently use `Self`) | Medium |
| 5 | P2 | Implement weighted graph variants (Section 3.5) | Medium |
| 6 | P2 | Implement map_vertices/map_edges operations per prose cost specs | Medium |
| 7 | P2 | Implement Exercise 52.5 (constant-span edge deletion via inject) | Low |
| 8 | P3 | Create missing source files: EdgeSetGraphMtEph, AdjTableGraphMtEph | Low |
| 9 | P3 | Create missing test files: TestAdjSeqGraphMtEph, TestAdjMatrixGraphMtEph | Low |
| 10 | P3 | Upgrade PartialEq impls on StPer types to use `PartialEqSpecImpl` pattern | Low |
| 11 | P3 | Update trait declaration annotations from old `/// claude-4-sonet:` to standard two-line format | Low |
| 12 | P3 | Add PartialEq/Eq to remaining types (MtPer, MtEph, StEph variants) | Low |

## Proof Holes Summary (updated 2026-02-18)

| Hole Type | Count | Files |
|---|---|---|
| `#[verifier::external_body]` | 148 | All 14 source files |
| `assume(...)` | 0 | — |
| `admit()` | 0 | — |
| `#[verifier::external]` | 0 | — |
| `#[verifier::external_fn_specification]` | 0 | — |
| **Total** | **148** | — |

**Errors:** 1 bare impl in file with trait definition (AdjSeqGraphMtPer.rs)

### external_body Distribution

| # | File | external_body Count |
|---|---|---|
| 1 | EdgeSetGraphStEph | 13 |
| 2 | EdgeSetGraphStPer | 13 |
| 3 | EdgeSetGraphMtPer | 14 |
| 4 | AdjTableGraphStEph | 12 |
| 5 | AdjTableGraphStPer | 12 |
| 6 | AdjTableGraphMtPer | 11 |
| 7 | AdjSeqGraphStEph | 9 |
| 8 | AdjSeqGraphStPer | 9 |
| 9 | AdjSeqGraphMtEph | 8 |
| 10 | AdjSeqGraphMtPer | 8 |
| 11 | AdjMatrixGraphStEph | 9 |
| 12 | AdjMatrixGraphStPer | 9 |
| 13 | AdjMatrixGraphMtEph | 8 |
| 14 | AdjMatrixGraphMtPer | 14 |
| | **Total** | **148** |

**Note:** Count increased from 146 to 148 due to agent2 adding `count_edges_parallel` helper functions in AdjSeqGraphMtEph.rs and AdjSeqGraphMtPer.rs.
