<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 52 Report

## Summary

Round 52 closed **Chap62 entirely** (now a clean chapter) and reduced Chap26 from 2 holes to 1 permanent hole.

- **Start**: 4472 verified, 25 holes, 37 clean chapters
- **End**: 4472 verified, 20 holes, 38 clean chapters
- **Net**: −5 holes, +1 clean chapter (Chap62)

## Holes Closed

| # | Chap | File | Line | Type | How Resolved |
|---|:----:|---|---:|---|---|
| 1 | 62 | StarPartitionStEph.rs | ~200 | `assume` | Full loop invariant proof; `spec_valid_partition_map` ensures added |
| 2 | 62 | StarContractionStEph.rs | ~100 | `assume` | Replaced with assert from new ensures |
| 3 | 26 | ETSPMtEph.rs | ~450 | `external_body` | `find_best_swap_par` proved; while loops + join closures |
| 4 | 62 | StarPartitionMtEph.rs | ensures | `ensures true` | Full proof of `spec_valid_partition_map` in parallel version |
| 5 | 62 | StarContractionMtEph.rs | 115 | `assume` | Replaced with assert from new ensures |

## Proof Techniques

### StarPartitionStEph.rs (Sequential)

Multi-phase loop proof with ghost snapshots:
- Outer loop over vertices: maintained `processed` prefix, `partition_map` domain, and range-validity invariants
- Inner loop over edges: same three invariants within each vertex's neighborhood scan
- Post-loop: connected `graph.V@` to `partition_map` domain via `to_seq` ensures + `index_of` witness
- Used `clone_view()` throughout for guaranteed view equality

### StarPartitionMtEph.rs (Parallel)

Six-loop proof requiring careful invariant threading:
- **Loop 1**: Built `vertex_to_index` with both a range invariant (`forall j < i: ...`) and a **domain invariant** (`forall v ∈ dom: ∃j < i: vertices_vec[j]@ == v`). The domain invariant was critical for the post-loop Part B proof.
- **Loop 2**: Built `coin_flips` for all vertices, tracking key coverage.
- **Loop 3**: Built `th_edges` (tail-index, head-vertex pairs) with 7-part invariant: tail index in bounds, tail is tails, head is heads, head in vertex_to_index, head index in bounds.
- **Loop 4**: Initialized `p_vec = vertices_vec` verbatim.
- **Loop 5**: Applied th_edges to p_vec, maintaining the **heads-preserve invariant** (coin_flips[vertices_vec[j]] == true ⟹ p_vec[j] == vertices_vec[j]) plus tails-modified=heads and p_vec-in-vertex_to_index invariants.
- **Loop 6**: Built `centers` and `partition_map`, tracking prefix coverage and heads-in-centers prefix.
- **Post-loop Part A**: All graph vertices in partition_map (via to_seq `index_of` witness).
- **Post-loop Part B**: All partition_map values in centers via case analysis:
  - If `p_vec[j] == vertices_vec[j]`: directly from prefix heads-in-centers invariant.
  - If `p_vec[j] != vertices_vec[j]`: p_vec[j] is a heads vertex h. Find h's index q_h via `vertex_to_index@[h]` (valid because domain invariant gives `∃j3: vertices_vec[j3]@ == h`; then `vertex_to_index[vertices_vec[j3]] == j3 == q_h`). From heads-preserve, `p_vec[q_h] == vertices_vec[q_h] == h`. From prefix invariant at q_h: `centers.contains(h)`. ✓

### ETSPMtEph.rs find_best_swap_par

Replaced `external_body` divide-and-conquer with verified fork-join:
- Converted for-loops to while-loops with explicit bounds invariants
- Used `arc_deref` to get `&Vec<Edge>` references from `Arc<Vec<Edge>>` for indexing
- Named closures with `ensures r.0 < lt1_ll && r.1 < rt1_rl` propagated index bounds through `join`
- Proved `decreases hi - lo` with explicit `assert(mid - lo < hi - lo)` and `assert(hi - mid < hi - lo)`

## Blockers / Remaining Holes

| # | Chap | File | Type | Reason |
|---|:----:|---|---|---|
| 1 | 26 | ETSPMtEph.rs | `external_body` | `point_distance` — f64 sqrt; no Verus axioms for `f64::sqrt` |
| 2 | 65 | PrimStEph.rs | `external_body` | `prim_mst` — complex while loop with BinaryHeapPQ; deferred |

## Chapter Status

| # | Chapter | Holes Before | Holes After | Status |
|---|:----:|---:|---:|---|
| 1 | 62 | 2 | 0 | CLOSED ✓ |
| 2 | 26 | 2 | 1 | Reduced (1 permanent) |
| 3 | 65 | 1 | 1 | Not started |

## Notes

The domain invariant for `vertex_to_index` in Loop 1 (`forall v ∈ dom: ∃j: vertices_vec[j]@ == v`) was not initially included and required a rewrite. This invariant was essential for the Part B post-loop proof: it establishes that any heads vertex `h` stored in `p_vec` corresponds to some `vertices_vec[q_h]`, allowing the proof to reach the heads-in-centers prefix invariant at index `q_h`.

The `fn_missing_ensures` warnings remaining in Chap62 are for higher-order functions (`star_contract_mt_fuel`, `star_contract_mt`) with generic return type `R` — meaningful postconditions cannot be written without knowing `R` and the callbacks. These are not actionable proof holes.
