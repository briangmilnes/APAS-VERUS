# Agent 3 — Round 197 Report

## Summary

Implemented `EdgeSetGraphMtEph` — the mutable, multi-threaded edge-set graph representation for Chap52.

---

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|:---:|:---:|:---:|
| 1 | 52 | EdgeSetGraphMtEph.rs | — (new) | 1 (Clone accept) | — |

No holes were added or removed from existing files. The `accept(cloned@ == self@)` in `Clone::clone` is the standard project-approved bridge pattern (same as all other Clone impls).

---

## Chapters Closed

None closed — Chap52 was already verified. One new file added clean.

---

## Verification Counts

| Stage | Before | After |
|-------|--------|-------|
| Verus verified | 5688 | 5690 |
| RTT | 3760 | 3776 |
| PTT | 221 | 221 |

---

## Deliverables

### 1. `src/Chap52/EdgeSetGraphMtEph.rs` (new)
Full Verus-verified implementation. Key design decisions:

- **Type bound**: `V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static`
- **Backing store**: `AVLTreeSetMtEph<V>` and `AVLTreeSetMtEph<Pair<V, V>>` with `&mut self` insert/delete
- **No top-level RwLock**: consistent with sibling `AdjSeqGraphMtEph` and `AdjMatrixGraphMtEph`
- **`out_neighbors` uses loop, not filter**: closure capturing `&V` can't satisfy `Pred<T>: Send + Sync + 'static`
- **Capacity bounds** in `delete_vertex`, `insert_vertex`, `insert_edge`, `delete_edge`, `out_neighbors` — all extra preconditions from `AVLTreeSetMtEph`'s insert/delete/to_seq
- **`group_avltreeseqsteph_len_bound`** added to broadcast use to auto-derive `seq@.len() < usize::MAX` from `spec_avltreeseqsteph_wf()`

### 2. `tests/Chap52/TestEdgeSetGraphMtEph.rs` (replaced 5-line stub)
16 tests including:
- Basic functional: empty, from_vertices_and_edges, insert/delete vertex/edge, out_neighbors, out_degree, default
- Edge cases: idempotent insert, delete nonexistent elements
- Concurrent: `test_concurrent_build_stress` (20 threads building independent graphs), `test_shared_graph_concurrent_reads` (4 threads reading Arc<Mutex<graph>>)
- All 16 pass.

### 3. `benches/Chap52/BenchEdgeSetGraphMtEph.rs` (new)
Three criterion benchmark groups: `insert_edge`, `out_neighbors`, `delete_vertex`, each at sizes 16/64/256. `sample_size(10)`, `warm_up_time(100ms)`, `measurement_time(300ms)`, `harness = false`.

### 4. `src/lib.rs` — added `pub mod EdgeSetGraphMtEph;` to Chap52 block

### 5. `Cargo.toml` — added `[[bench]]` entry for `BenchEdgeSetGraphMtEph`

---

## Techniques Used

- **Broadcast group** `group_avltreeseqsteph_len_bound` to prove `seq@.len() < usize::MAX as nat` automatically from wf
- **Loop-count invariant** `neighbors@.len() <= i as nat` to bound neighbors set size at each insert
- **Explicit arithmetic chain** in proof: `neighbors@.len() + 1 <= seq@.len() < usize::MAX as nat`
- **`lemma_eq_spec_iff_view_eq`** local proof fn (same pattern as StEph and MtPer) for PartialEq bridge
- **`accept(cloned@ == self@)`** in Clone::clone (project-standard bridge pattern)

---

## Remaining Holes

| # | Chap | File | Count | What Blocks |
|---|------|------|:---:|---|
| 1 | 52 | EdgeSetGraphMtEph.rs | 1 | Clone/eq bridge — standard approved pattern |

No new non-standard holes introduced.

---

## Validation Results

```
Verus: 5690 verified, 0 errors
RTT:   3776 tests, 0 failures
PTT:   221 tests, 0 failures
```
