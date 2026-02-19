<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 52: Graphs and Their Representation — Review Against Prose

- **Date**: 2026-02-18
- **Reviewer**: Claude-Opus-4.6
- **Project**: APAS-VERUS-agent2
- **Prose Source**: `prompts/Chap52.txt` (Chapter 52 of APAS textbook)

## Phase 1: Summary

8 of 14 source files (all AdjSeq and AdjMatrix variants) are **fully verified** with 0 proof holes. All `external_body` annotations on those files were removed and replaced with `requires`/`ensures` specs, loop invariants, spec functions, and proof functions. The remaining 6 files (EdgeSetGraph 3 variants, AdjTableGraph 3 variants) are still blocked on unverified Chap41 (AVLTreeSet) and Chap43 (OrderedTable) dependencies.

| Metric | Value |
|---|---|
| Source files | 14 |
| Fully verified (0 holes) | 8 (AdjSeq × 4, AdjMatrix × 4) |
| Holed (blocked on deps) | 6 (EdgeSetGraph × 3, AdjTableGraph × 3) |
| Total `external_body` proof holes | 75 |
| Other proof holes (`assume`, `admit`) | 0 |
| Clean proof functions | 13 |
| Spec functions (verified files) | ~48 (spec_sum_of, spec_count_true, spec_wf, spec_n, spec_edge, spec_degree, spec_neighbor across 8 files) |
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

The 8 verified AdjSeq/AdjMatrix files now have **verified specs** (`requires`/`ensures` contracts with loop invariants) rather than just cost annotations. The cost annotations remain as documentation but the actual correctness is enforced by Verus verification.

The 6 holed EdgeSetGraph/AdjTableGraph files still carry two-line cost annotations on their `external_body` functions:
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
| 6 | complement | Θ(n²) | Θ(1) | Θ(n²) | Θ(n²) | Work ✓, Span ✗ (sequential, parallel helpers removed) |

### Spec Fidelity Notes

1. **View types are well-defined.** AdjSeq graph views use `Seq<Seq<int>>`, AdjMatrix graph views use `Seq<Seq<bool>>`, both reflecting the mathematical structure. EdgeSet and AdjTable views map to `Self` (no abstraction), which is adequate for `external_body` but will need revision when those files are verified.

2. **AdjSeq/AdjMatrix files have full `requires`/`ensures` contracts.** All 8 verified files have spec functions (`spec_sum_of`, `spec_count_true`, `spec_wf`, `spec_n`, `spec_edge`, `spec_degree`, `spec_neighbor`) and all trait methods carry verified contracts. The 6 EdgeSetGraph/AdjTableGraph files remain `external_body` with no contracts.

3. **`delete_vertex` implementations exceed APAS cost bounds** because the code removes all incident edges rather than assuming the vertex is isolated. The APAS cost specs assume isolated vertex deletion; the implementations handle non-isolated vertices. This is a deliberate implementation choice that is more robust but less efficient.

4. **Constructors use `tabulate` and `from_vec`.** The verified files build their internal sequences via `ArraySeq*::tabulate` or `ArraySeq*::from_vec`, with loop invariants proving the resulting view matches the spec.

## Phase 4: Parallelism Review for Mt Modules

| # | Module | True Parallelism? | Operations with Threads | Notes |
|---|---|---|---|---|
| 1 | AdjSeqGraphMtEph | No | None | All loops sequential despite Mt backing type |
| 2 | AdjSeqGraphMtPer | No | None | All loops sequential; `map_vertices` (outside verus!) also sequential |
| 3 | AdjMatrixGraphMtEph | No | None | All loops sequential despite Mt backing type |
| 4 | AdjMatrixGraphMtPer | No | None | Parallel helpers removed; all operations now sequential verified loops |
| 5 | AdjTableGraphMtPer | No | None | `delete_vertex` has TODO for parallelization; `num_edges` sequential |
| 6 | EdgeSetGraphMtPer | Partial | None explicit | Uses MtPer sets which may parallelize internally (filter, etc.) |

### Parallelism Details

No file in Chapter 52 now has true thread-based parallelism. AdjMatrixGraphMtPer previously had 7 parallel helper functions (`count_edges_parallel`, `count_row_parallel`, `collect_neighbors_parallel`, `complement_matrix_parallel`, etc.) using `thread::spawn` with divide-and-conquer. These were removed during verification and replaced with sequential verified loops.

### Parallelism Gaps

- **All Mt files**: No true parallelism despite using Mt backing types. All operations are sequential loops.
- **AdjSeqGraphMtEph/MtPer**: Should use parallel tabulate/reduce for `num_edges`, parallel inject for `has_edge`.
- **AdjMatrixGraphMtEph/MtPer**: Should use parallel operations for `num_edges`, `out_neighbors`, `out_degree`, `complement` to achieve APAS span bounds.
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
| 6 | Spec/proof functions (spec_sum_of, lemma_sum_of_monotone, etc.) | AdjSeq/AdjMatrix files (all 8) | Verification infrastructure for proving correctness of graph operations |

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

| # | Priority | Action | Effort | Status |
|---|---|---|---|---|
| 1 | P1 | ~~Add `requires`/`ensures` contracts to AdjSeq/AdjMatrix files~~ | ~~High~~ | **Done** — all 8 files fully verified |
| 2 | P1 | Verify EdgeSetGraph files (3 variants) — blocked on Chap41 AVLTreeSet | High — 40 external_body | Blocked |
| 3 | P1 | Verify AdjTableGraph files (3 variants) — blocked on Chap41/43 | High — 35 external_body | Blocked |
| 4 | P1 | Re-add parallelism to Mt files (AdjSeq × 2, AdjMatrix × 2) with verified specs | Medium | Open |
| 5 | P2 | Add View types with proper abstraction (EdgeSet/AdjTable currently use `Self`) | Medium | Open |
| 6 | P2 | Implement weighted graph variants (Section 3.5) | Medium | Open |
| 7 | P2 | Implement map_vertices/map_edges operations per prose cost specs | Medium | Open |
| 8 | P2 | Implement Exercise 52.5 (constant-span edge deletion via inject) | Low | Open |
| 9 | P3 | Create missing source files: EdgeSetGraphMtEph, AdjTableGraphMtEph | Low | Open |
| 10 | P3 | Create missing test files: TestAdjSeqGraphMtEph, TestAdjMatrixGraphMtEph | Low | Open |
| 11 | P3 | Upgrade PartialEq impls on StPer types to use `PartialEqSpecImpl` pattern | Low | Open |
| 12 | P3 | Update trait declaration annotations from old `/// claude-4-sonet:` to standard two-line format | Low | Open |
| 13 | P3 | Add PartialEq/Eq to remaining types (MtPer, MtEph, StEph variants) | Low | Open |

## Proof Holes Summary (updated 2026-02-18)

| Hole Type | Count | Files |
|---|---|---|
| `#[verifier::external_body]` | 75 | 6 holed files (EdgeSetGraph × 3, AdjTableGraph × 3) |
| `assume(...)` | 0 | — |
| `admit()` | 0 | — |
| `#[verifier::external]` | 0 | — |
| `#[verifier::external_fn_specification]` | 0 | — |
| **Total** | **75** | — |

### Clean Files (0 holes, fully verified)

| # | File | Proof Functions | Notes |
|---|---|---|---|
| 1 | AdjSeqGraphStEph | 2 | lemma_sum_of_monotone + 1 other |
| 2 | AdjSeqGraphStPer | 1 | lemma_sum_of_monotone |
| 3 | AdjSeqGraphMtEph | 1 | lemma_sum_of_monotone |
| 4 | AdjSeqGraphMtPer | 1 | lemma_sum_of_monotone |
| 5 | AdjMatrixGraphStEph | 2 | lemma_sum_of_monotone, lemma_count_true_monotone |
| 6 | AdjMatrixGraphStPer | 2 | lemma_sum_of_monotone, lemma_count_true_monotone |
| 7 | AdjMatrixGraphMtEph | 2 | lemma_sum_of_monotone, lemma_count_true_monotone |
| 8 | AdjMatrixGraphMtPer | 2 | lemma_sum_of_monotone, lemma_count_true_monotone |
| | **Total proof fns** | **13** | |

All 8 files have spec functions (`spec_sum_of`, `spec_count_true`, `spec_wf`, `spec_n`, `spec_edge`, `spec_degree`, `spec_neighbor`), full `requires`/`ensures` contracts on all trait methods, and loop invariants on all implementations.

### Holed Files (blocked on unverified dependencies)

| # | File | external_body Count | Blocking Dependency |
|---|---|---|---|
| 1 | EdgeSetGraphStEph | 13 | Chap41 (AVLTreeSet) |
| 2 | EdgeSetGraphStPer | 13 | Chap41 (AVLTreeSet) |
| 3 | EdgeSetGraphMtPer | 14 | Chap41 (AVLTreeSet) |
| 4 | AdjTableGraphStEph | 12 | Chap41 (AVLTreeSet), Chap43 (OrderedTable) |
| 5 | AdjTableGraphStPer | 12 | Chap41 (AVLTreeSet), Chap43 (OrderedTable) |
| 6 | AdjTableGraphMtPer | 11 | Chap41 (AVLTreeSet), Chap43 (OrderedTable) |
| | **Total** | **75** | |

### Progress

| Metric | Previous (2026-02-17) | Current (2026-02-18) | Delta |
|---|---|---|---|
| Total external_body | 148 | 75 | -73 |
| Clean files | 0 | 8 | +8 |
| Holed files | 14 | 6 | -8 |
| Clean proof functions | 0 | 13 | +13 |
