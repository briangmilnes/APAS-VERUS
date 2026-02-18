<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 52: Graphs and Their Representation — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Chapter:** 52 — Graphs and Their Representation

## Critical Finding

**Chapter 52 contains NO Verus verification.** All 14 source files are plain Rust without `verus!{}` blocks. There are no spec functions, no proof functions, no `requires`/`ensures` clauses, and no proof holes (trivially). The veracity-review-module-fn-impls tool extracted 0 entries from this chapter. All spec strengths are **none**.

---

## Phase 1: Inventory (Tool-Generated)

The `veracity-review-module-fn-impls -d src/Chap52` tool found **0 verus functions** across 14 source files. All code is plain Rust outside of any `verus!{}` block.

| # | Module | Functions | V! | -V! | Spec | Holes |
|---|--------|:---------:|:--:|:---:|:----:|:-----:|
| 1 | EdgeSetGraphStEph | 12 | 0 | 12 | none | 0 |
| 2 | EdgeSetGraphStPer | 12 | 0 | 12 | none | 0 |
| 3 | EdgeSetGraphMtPer | 12 | 0 | 12 | none | 0 |
| 4 | AdjTableGraphStEph | 12 | 0 | 12 | none | 0 |
| 5 | AdjTableGraphStPer | 12 | 0 | 12 | none | 0 |
| 6 | AdjTableGraphMtPer | 10 | 0 | 10 | none | 0 |
| 7 | AdjSeqGraphStEph | 9 | 0 | 9 | none | 0 |
| 8 | AdjSeqGraphStPer | 9 | 0 | 9 | none | 0 |
| 9 | AdjSeqGraphMtEph | 7 | 0 | 7 | none | 0 |
| 10 | AdjSeqGraphMtPer | 7 | 0 | 7 | none | 0 |
| 11 | AdjMatrixGraphStEph | 9 | 0 | 9 | none | 0 |
| 12 | AdjMatrixGraphStPer | 9 | 0 | 9 | none | 0 |
| 13 | AdjMatrixGraphMtEph | 8 | 0 | 8 | none | 0 |
| 14 | AdjMatrixGraphMtPer | 7 + 7 helpers | 0 | 14 | none | 0 |

**Missing source modules:**
- `EdgeSetGraphMtEph` — no source file exists
- `AdjTableGraphMtEph` — no source file exists

---

## Phase 2: Prose Inventory

### Definitions

| # | Name | Type | Section |
|---|------|------|---------|
| 1 | Graph / Network | Concept | §1 |
| 2 | Relation as subset of Cartesian product | Definition | §1 |
| 3 | Edge Set representation: G = (V set, (V × V) set) | Definition | §3.1 |
| 4 | Adjacency Table representation: G = (V × (V set)) table | Definition 52.2 | §3.2 |
| 5 | Adjacency Sequence representation: G = (int seq) seq | Definition 52.4 | §3.3 |
| 6 | Adjacency Matrix representation: G = (bool seq) seq | Definition | §3.4 |
| 7 | Weighted/Labeled graphs | Concept | §3.5 |
| 8 | Label Table representation | Concept | §3.5 |

### Cost Specifications

| # | Name | Section | Implemented? |
|---|------|---------|:------------:|
| 1 | Cost Spec 52.1 — Edge Sets | §3.1 | Yes |
| 2 | Cost Spec 52.3 — Adjacency Tables | §3.2 | Yes |
| 3 | Cost Spec 52.5 — Adjacency Sequences | §3.3 | Yes |
| 4 | Cost Spec 52.6 — Adjacency Matrix | §3.4 | Yes |

### Exercises

| # | Exercise | Description | Implemented? |
|---|----------|-------------|:------------:|
| 1 | Exercise 52.1 | Prove lg m = O(lg n) | No — text-only proof |
| 2 | Exercise 52.2 | Cost of deleting vertex with out-degree d | No — text answer |
| 3 | Exercise 52.3 | Cost of finding out-neighbors in AdjSeq | No — text answer |
| 4 | Exercise 52.4 | Why map over edges requires Ω(n) work | No — text answer |
| 5 | Exercise 52.5 | Constant-span edge deletion algorithm | No |
| 6 | Exercise 52.6 | Constant-span graph complement | Yes — in AdjMatrixGraphMtPer |

### Applications (§2)

The prose describes 14 application domains (social networks, transportation, compilers, etc.). These are descriptive text, not algorithms to implement.

### Weighted Graphs (§3.5)

The prose describes label tables and adjacency-table/sequence extensions for weighted edges. **No weighted graph implementations exist in the codebase.** Example 52.6-52.8 describe weighted graph representations but none are implemented.

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

APAS/Claude-Opus-4.6 cost comment pairs have been added to all exec functions across all 14 source files.

### 3b. Implementation Fidelity

**Edge Set (Cost Spec 52.1):**

| # | Function | APAS | Actual | Fidelity |
|---|----------|------|--------|----------|
| 1 | has_edge | Work Θ(lg n) | Work Θ(lg m) | Agrees (lg m = O(lg n)) |
| 2 | out_neighbors | Work Θ(m), Span Θ(lg n) | Work Θ(m), Span Θ(m) for St | Span differs — St is sequential |
| 3 | out_degree | Work Θ(m), Span Θ(lg n) | Work Θ(m), Span Θ(m) for St | Delegates to out_neighbors |
| 4 | insert/delete vertex | Work Θ(lg n) | Work Θ(m) for non-isolated | Implementation removes incident edges; APAS assumes isolated vertex |
| 5 | insert/delete edge | Work Θ(lg n) | Work Θ(lg n) | Agrees |

**Adjacency Table (Cost Spec 52.3):**

| # | Function | APAS | Actual | Fidelity |
|---|----------|------|--------|----------|
| 1 | has_edge | Θ(lg n) | Θ(lg n) | Agrees |
| 2 | out_neighbors | Θ(lg n + d(v)) | Θ(lg n) | Agrees |
| 3 | num_edges | Θ(m) | Θ(n + m) | Sequential sum, agrees on work order |
| 4 | delete_vertex | Θ(lg n) isolated | Θ(n lg n) | Iterates all vertices; APAS assumes isolated |
| 5 | vertices() | Θ(n) | Θ(n lg n) | Builds new AVL set via sequential inserts |

**Adjacency Sequence (Cost Spec 52.5):**

| # | Function | APAS | Actual | Fidelity |
|---|----------|------|--------|----------|
| 1 | has_edge | Work Θ(d(u)), Span Θ(lg d(u)) | Work Θ(d(u)), Span Θ(d(u)) | Span differs — linear scan, not parallel reduce |
| 2 | out_neighbors | Θ(1) | Θ(1) | Agrees |
| 3 | out_degree | Θ(1) | Θ(1) | Agrees |
| 4 | insert_edge | Work Θ(n), Span Θ(1) | Work Θ(n), Span Θ(n) | Span differs — sequential copy |
| 5 | delete_edge | Work Θ(n), Span Θ(1) | Work Θ(n), Span Θ(n) | Span differs — sequential copy |

**Adjacency Matrix (Cost Spec 52.6):**

| # | Function | APAS | Actual | Fidelity |
|---|----------|------|--------|----------|
| 1 | has_edge | Θ(1) | Θ(1) | Agrees |
| 2 | out_neighbors | Work Θ(n), Span Θ(1) | Work Θ(n), Span Θ(n) for St | Span differs for St variants |
| 3 | out_degree | Work Θ(n), Span Θ(lg n) | Work Θ(n), Span Θ(n) for St | Span differs for St variants |
| 4 | complement | Work Θ(n²), Span Θ(1) | MtPer: Span Θ(lg n), St: Span Θ(n²) | MtPer achieves Θ(lg n) span via parallel D&C |
| 5 | set_edge | Work Θ(n) | Eph: Θ(1), Per: Θ(n) | Ephemeral achieves O(1) via in-place update |

### 3c. Spec Fidelity

**Not applicable.** No functions have `requires`/`ensures` since there is no Verus verification in this chapter.

---

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

**EdgeSetGraphMtPer** — 12 functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | empty | N/A | Constructor |
| 2 | from_vertices_and_edges | N/A | Constructor |
| 3 | num_vertices | Sequential | Delegates to size() |
| 4 | num_edges | Sequential | Delegates to size() |
| 5 | vertices/edges | Sequential | Returns reference |
| 6 | has_edge | Sequential | Single AVL find |
| 7 | out_neighbors | **Sequential** | Filter may use parallel AVL internals, but insert loop is sequential |
| 8 | out_degree | Sequential | Delegates to out_neighbors |
| 9 | insert_vertex | Sequential | Single AVL insert |
| 10 | delete_vertex | **Partially parallel** | Filter uses move closure (parallel filter possible), but the rest is sequential |
| 11 | insert_edge | Sequential | AVL inserts |
| 12 | delete_edge | Sequential | AVL delete |

**AdjTableGraphMtPer** — 10 functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | empty | N/A | Constructor |
| 2 | num_vertices | Sequential | Table size |
| 3 | num_edges | **Sequential** | Sequential for-loop over domain |
| 4 | has_edge | Sequential | Table find + set find |
| 5 | out_neighbors | Sequential | Table lookup |
| 6 | out_degree | Sequential | Delegates |
| 7 | insert_vertex | Sequential | Table insert |
| 8 | delete_vertex | **Sequential** | Has TODO to parallelize; sequential loop |
| 9 | insert_edge | Sequential | Table operations |
| 10 | delete_edge | Sequential | Table operations |

**AdjSeqGraphMtEph** — 7 functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | new | Sequential | Sequential loop |
| 2 | num_vertices | Sequential | Length |
| 3 | num_edges | **Sequential** | Sequential loop |
| 4 | has_edge | Sequential | Linear scan |
| 5 | out_neighbors | Sequential | Clone |
| 6 | out_degree | Sequential | Length |
| 7 | set_edge | Sequential | Linear scan + rebuild |

**AdjSeqGraphMtPer** — 7 functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | new | Sequential | Sequential loop |
| 2 | num_vertices | Sequential | Length |
| 3 | num_edges | **Sequential** | Comment says "avoids Verus Ghost in cargo build" |
| 4 | has_edge | Sequential | Linear scan |
| 5 | out_neighbors | Sequential | Clone |
| 6 | out_degree | Sequential | Length |
| 7 | map_vertices | **Sequential** | Comment says "avoids Verus Ghost in cargo build" |

**AdjMatrixGraphMtEph** — 8 functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | new | Sequential | Sequential loop |
| 2 | num_vertices | Sequential | Stored field |
| 3 | num_edges | **Sequential** | Sequential double loop |
| 4 | has_edge | Sequential | Direct access |
| 5 | out_neighbors | **Sequential** | Sequential row scan |
| 6 | out_degree | **Sequential** | Sequential row count |
| 7 | set_edge | Sequential | In-place update |
| 8 | complement | **Sequential** | Sequential double loop |

**AdjMatrixGraphMtPer** — 7 + 7 helper functions:

| # | Function | Classification | Notes |
|---|----------|:-------------:|-------|
| 1 | new | Sequential | Sequential loop |
| 2 | num_vertices | Sequential | Stored field |
| 3 | num_edges | **Parallel** | Uses count_edges_parallel with thread::spawn/join D&C |
| 4 | has_edge | Sequential | Direct access |
| 5 | out_neighbors | **Parallel** | Uses collect_neighbors_parallel with thread::spawn/join D&C |
| 6 | out_degree | **Parallel** | Uses count_row_parallel with thread::spawn/join D&C |
| 7 | complement | **Parallel** | Uses complement_matrix_parallel with nested thread::spawn/join D&C |

### 4b. Span Audit

| # | Module | Function | APAS Span | Actual Span | Match? |
|---|--------|----------|-----------|-------------|:------:|
| 1 | AdjMatrixGraphMtPer | num_edges | Θ(1) | Θ(lg n) | No — Θ(lg n) via D&C, not Θ(1) |
| 2 | AdjMatrixGraphMtPer | out_neighbors | Θ(1) | Θ(lg n) | No — Θ(lg n) via D&C |
| 3 | AdjMatrixGraphMtPer | out_degree | Θ(lg n) | Θ(lg n) | Yes |
| 4 | AdjMatrixGraphMtPer | complement | Θ(1) | Θ(lg n) | No — Θ(lg n) via D&C |
| 5 | EdgeSetGraphMtPer | out_neighbors | Θ(lg n) | Θ(m) | No — insert loop is sequential |
| 6 | EdgeSetGraphMtPer | delete_vertex | Θ(lg n) | Θ(lg m) | Partially — filter may be parallel |
| 7 | AdjSeqGraphMtPer | num_edges | Θ(1) | Θ(n+m) | No — sequential loop |
| 8 | AdjSeqGraphMtPer | map_vertices | Θ(1) | Θ(n+m) | No — sequential loop |
| 9 | AdjMatrixGraphMtEph | complement | Θ(1) | Θ(n²) | No — sequential loop |

Note: APAS Span values assume full parallelism. The Θ(1) spans in APAS Cost Specs 52.5 and 52.6 assume parallel tabulate/map operations. The implementations use sequential loops except for AdjMatrixGraphMtPer which uses explicit thread::spawn/join achieving Θ(lg n) span.

### 4c. Parallelism Gap Table

| # | Module | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|--------|----------|-----------|-------------|:---------:|-------|
| 1 | EdgeSetGraphMtPer | out_neighbors | Θ(lg n) | Θ(m) | No | Insert loop is sequential |
| 2 | EdgeSetGraphMtPer | delete_vertex | Θ(lg n) | Θ(lg m) | Partial | Filter may parallelize, vertex delete is sequential |
| 3 | AdjTableGraphMtPer | num_edges | Θ(lg n) | Θ(n+m) | No | Sequential for-loop |
| 4 | AdjTableGraphMtPer | delete_vertex | Θ(lg² n) | Θ(n lg n) | No | Sequential loop; has TODO comment |
| 5 | AdjSeqGraphMtEph | all functions | Various | Work=Span | No | All sequential |
| 6 | AdjSeqGraphMtPer | num_edges | Θ(1) | Θ(n+m) | No | Sequential; "avoids Verus Ghost" comment |
| 7 | AdjSeqGraphMtPer | map_vertices | Θ(1) | Θ(n+m) | No | Sequential; "avoids Verus Ghost" comment |
| 8 | AdjMatrixGraphMtEph | num_edges | Θ(1) | Θ(n²) | No | Sequential double loop |
| 9 | AdjMatrixGraphMtEph | out_neighbors | Θ(1) | Θ(n) | No | Sequential row scan |
| 10 | AdjMatrixGraphMtEph | out_degree | Θ(lg n) | Θ(n) | No | Sequential row count |
| 11 | AdjMatrixGraphMtEph | complement | Θ(1) | Θ(n²) | No | Sequential double loop |
| 12 | AdjMatrixGraphMtPer | num_edges | Θ(1) | Θ(lg n) | **Yes** | Parallel D&C |
| 13 | AdjMatrixGraphMtPer | out_neighbors | Θ(1) | Θ(lg n) | **Yes** | Parallel D&C |
| 14 | AdjMatrixGraphMtPer | out_degree | Θ(lg n) | Θ(lg n) | **Yes** | Parallel D&C |
| 15 | AdjMatrixGraphMtPer | complement | Θ(1) | Θ(lg n) | **Yes** | Parallel D&C, Exercise 52.6 |

**Summary:** Only `AdjMatrixGraphMtPer` achieves genuine parallelism (4 functions with thread::spawn/join D&C). All other Mt modules are thread-safe but sequential. `AdjMatrixGraphMtEph` is the most notable gap — it has the Mt type signature but every function is sequential.

---

## Phase 5: Runtime Test Review

### 5a. Coverage Check

| # | Source Module | RTT File | Exists? | Notes |
|---|-------------|----------|:-------:|-------|
| 1 | EdgeSetGraphStEph | TestEdgeSetGraphStEph | Yes | 4 test functions |
| 2 | EdgeSetGraphStPer | TestEdgeSetGraphStPer | Yes | 7 test functions |
| 3 | EdgeSetGraphMtPer | TestEdgeSetGraphMtPer | Yes | 7 test functions |
| 4 | AdjTableGraphStEph | TestAdjTableGraphStEph | Yes | 7 test functions |
| 5 | AdjTableGraphStPer | TestAdjTableGraphStPer | Yes | 9 test functions |
| 6 | AdjTableGraphMtPer | TestAdjTableGraphMtPer | Yes | 7 test functions |
| 7 | AdjSeqGraphStEph | TestAdjSeqGraphStEph | Yes | 6 test functions |
| 8 | AdjSeqGraphStPer | TestAdjSeqGraphStPer | Yes | 5 test functions |
| 9 | AdjSeqGraphMtEph | — | **No** | No test file |
| 10 | AdjSeqGraphMtPer | TestAdjSeqGraphMtPer | Yes | 7 test functions |
| 11 | AdjMatrixGraphStEph | TestAdjMatrixGraphStEph | Yes | 6 test functions |
| 12 | AdjMatrixGraphStPer | TestAdjMatrixGraphStPer | Yes | 5 test functions |
| 13 | AdjMatrixGraphMtEph | — | **No** | No test file |
| 14 | AdjMatrixGraphMtPer | TestAdjMatrixGraphMtPer | Yes | 9 test functions |

**Missing source modules** (no test needed):
- EdgeSetGraphMtEph — test stub exists noting source doesn't exist
- AdjTableGraphMtEph — no source and no test

### 5b. Test Quality

Tests generally cover:
- Empty graph construction
- Edge insertion and verification (has_edge)
- Edge deletion
- Vertex deletion (for set/table types)
- Out-neighbors and out-degree
- Persistence (for StPer/MtPer: original graph unchanged after operations)

Tests generally lack:
- Edge cases for boundary conditions
- Large graph testing (most tests use 3-5 vertices)
- No complement testing for matrix types (only in MtPer)
- No map_vertices testing for AdjSeqGraphMtPer

### 5c. Missing Tests

| # | Priority | Module | Missing Coverage |
|---|:--------:|--------|-----------------|
| 1 | High | AdjSeqGraphMtEph | No test file at all |
| 2 | High | AdjMatrixGraphMtEph | No test file at all |
| 3 | Medium | AdjMatrixGraphStEph | Missing complement test |
| 4 | Medium | AdjMatrixGraphStPer | Missing complement test |
| 5 | Low | AdjSeqGraphMtPer | Missing map_vertices test |

---

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 52 has no Verus code — no `verus!{}` blocks, no iterators with ghost state, no verified loops. PTTs are not applicable.

---

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Description | Status |
|---|-----------|-------------|--------|
| 1 | Weighted graphs (§3.5) | Label table, weighted adjacency table/sequence | **Not implemented** |
| 2 | Example 52.6-52.8 | Weighted graph examples | Not implemented |
| 3 | Exercise 52.5 | Constant-span edge deletion via inject | Not implemented |
| 4 | EdgeSetGraphMtEph | Ephemeral multi-threaded edge set | **Source missing** |
| 5 | AdjTableGraphMtEph | Ephemeral multi-threaded adjacency table | **Source missing** |
| 6 | Mixed adj seq/table (§3.3) | Mixed representations mentioned in prose | Not implemented (by design) |
| 7 | Adjacency list variant (§3.3) | Linked-list inner representation | Not implemented (by design) |

### Code With No Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | `from_vertices_and_edges`, `from_table`, `from_seq`, `from_matrix` | Convenience constructors |
| 2 | `set_neighbors` (AdjSeqStEph) | Direct neighbor-list setter |
| 3 | `set_edge` (AdjSeq/AdjMatrix Eph) | Unified insert/delete via boolean flag |
| 4 | `Default` impls (MtPer types) | Rust trait implementation |
| 5 | Parallel helper functions (AdjMatrixGraphMtPer) | 7 internal D&C helpers |
| 6 | `vertices()` accessor (Adj Table types) | Returns vertex set from table domain |

---

## Phase 8: Table of Contents Review

**No TOC headers present in any file.** Since all 14 files are plain Rust (no `verus!{}` blocks), the standard TOC ordering (sections 1-13) does not fully apply. The files use a simple structure: module > imports > struct > trait > impl.

### In/Out Table

Not applicable — no `verus!{}` blocks exist, so there is no in/out placement to audit. Derive impls are all outside non-existent verus blocks:

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|
| 1 | EdgeSetGraphStEph | ✅ out | - | - | - | - | - | - |
| 2 | EdgeSetGraphStPer | ✅ out | ✅ out | - | - | - | ✅ out | - |
| 3 | EdgeSetGraphMtPer | ✅ out | - | ✅ out | - | - | - | - |
| 4 | AdjTableGraphStEph | ✅ out | - | - | - | - | - | - |
| 5 | AdjTableGraphStPer | ✅ out | - | - | - | - | - | - |
| 6 | AdjTableGraphMtPer | ✅ out | - | ✅ out | - | - | - | - |
| 7 | AdjSeqGraphStEph | ✅ out | - | - | - | - | - | - |
| 8 | AdjSeqGraphStPer | ✅ out | ✅ out | - | - | - | ✅ out | - |
| 9 | AdjSeqGraphMtEph | ✅ out | - | - | - | - | - | - |
| 10 | AdjSeqGraphMtPer | ✅ out | - | - | - | - | - | - |
| 11 | AdjMatrixGraphStEph | ✅ out | - | - | - | - | - | - |
| 12 | AdjMatrixGraphStPer | ✅ out | ✅ out | - | - | - | ✅ out | - |
| 13 | AdjMatrixGraphMtEph | ✅ out | - | - | - | - | - | - |
| 14 | AdjMatrixGraphMtPer | ✅ out | - | - | - | - | - | - |

Note: When/if Verus verification is added, Clone/PartialEq/Eq should move inside `verus!{}` with specifications per the project rules.

---

## Proof Holes Summary

```
veracity-review-proof-holes output:

✓ AdjMatrixGraphMtEph.rs
✓ AdjMatrixGraphMtPer.rs
✓ AdjMatrixGraphStEph.rs
✓ AdjMatrixGraphStPer.rs
✓ AdjSeqGraphMtEph.rs
✓ AdjSeqGraphMtPer.rs
✓ AdjSeqGraphStEph.rs
✓ AdjSeqGraphStPer.rs
✓ AdjTableGraphMtPer.rs
✓ AdjTableGraphStEph.rs
✓ AdjTableGraphStPer.rs
✓ EdgeSetGraphMtPer.rs
✓ EdgeSetGraphStEph.rs
✓ EdgeSetGraphStPer.rs

SUMMARY
  Modules: 14 clean, 0 holed, 14 total
  Proof Functions: 0 total
  Holes Found: 0

No proof holes found (trivially — no Verus code exists).
```

---

## Spec Strength Summary

| Classification | Count |
|:--------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | ~138 |

**All functions have spec_strength = none** because the chapter contains no Verus verification. No `requires`/`ensures` clauses exist anywhere.

---

## Overall Assessment

### Strengths

1. **Complete representation coverage.** All four graph representations from the prose (Edge Set, Adjacency Table, Adjacency Sequence, Adjacency Matrix) are implemented.
2. **Variant coverage.** 14 files covering StEph, StPer, MtEph, and MtPer variants for most representations. Only 2 MtEph modules are missing (EdgeSetGraphMtEph, AdjTableGraphMtEph).
3. **Good test coverage.** 13 test files covering 12 of 14 source modules (2 missing for MtEph).
4. **Genuine parallelism in AdjMatrixGraphMtPer.** The complement, num_edges, out_neighbors, and out_degree operations use explicit thread::spawn/join divide-and-conquer achieving Θ(lg n) span.
5. **Exercise 52.6 implemented.** Parallel graph complement in AdjMatrixGraphMtPer.
6. **Cost annotations now complete.** APAS/Claude-Opus-4.6 cost comment pairs added to all impl functions.

### Weaknesses

1. **No Verus verification.** This is the most significant gap. The chapter is pure Rust with zero formal verification. No spec functions, no proof functions, no requires/ensures.
2. **Most Mt modules are not parallel.** Only AdjMatrixGraphMtPer has genuine parallelism. EdgeSetGraphMtPer, AdjTableGraphMtPer, AdjSeqGraphMtEph, AdjSeqGraphMtPer, and AdjMatrixGraphMtEph are all thread-safe but sequential.
3. **Weighted graphs not implemented.** Section 3.5 of the prose describes weighted/labeled graph representations (label tables, weighted adjacency tables/sequences). None are implemented.
4. **Missing source modules.** EdgeSetGraphMtEph and AdjTableGraphMtEph do not exist.
5. **delete_vertex implementations exceed APAS costs.** All delete_vertex implementations iterate over all edges/vertices to remove incident edges, giving Work Θ(m) or Θ(n lg n). APAS states Θ(lg n) but assumes isolated vertex deletion.
6. **Sequential span in St types.** All St (single-threaded) variants have Span = Work, which is expected but differs from the APAS Cost Specifications that state parallel spans. This is inherent to the St design.

### Priority Actions

| # | Priority | Action |
|---|:--------:|--------|
| 1 | High | Add Verus verification to at least the StPer variants |
| 2 | High | Implement weighted graph representations (§3.5) |
| 3 | Medium | Add parallelism to AdjMatrixGraphMtEph (currently all sequential) |
| 4 | Medium | Implement EdgeSetGraphMtEph and AdjTableGraphMtEph |
| 5 | Medium | Add parallelism to AdjSeqGraphMtPer (num_edges, map_vertices) |
| 6 | Low | Add test files for AdjSeqGraphMtEph and AdjMatrixGraphMtEph |
| 7 | Low | Parallelize AdjTableGraphMtPer.delete_vertex (has TODO comment) |
