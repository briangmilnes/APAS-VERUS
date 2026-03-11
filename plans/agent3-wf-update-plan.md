# Agent3 Plan: spec_wf Updates for Chap52, 53, 56, 57, 58, 59

Date: 2026-03-11
Branch: agent3/ready

## Summary

Add `spec_<module_no_underscores>_wf` predicates to data-structure and result modules
per `src/standards/spec_wf_standard.rs`. No multi_struct work needed (0 files).

## Investigation Results

The plan assigned 23 files. After reading every file, the actual breakdown is:

| # | Status | Count | Files |
|---|--------|-------|-------|
| 1 | Needs wf (data structure) | 10 | Chap52: AdjSeq×4, AdjTable×3, EdgeSet×3 |
| 2 | Needs wf (result struct) | 2 | Chap53: PQMinStEph, PQMinStPer |
| 3 | Needs wf (mirror I64 pattern) | 4 | Chap56: AllPairsF64×2, SSSPF64×2 |
| 4 | No meaningful wf | 1 | Chap57: StackStEph (Vec wrapper, any Vec is valid) |
| 5 | Placeholder / no struct | 3 | Chap58: BellmanFordStEphF64, Chap59: JohnsonStEphF64, JohnsonMtEphF64 |
| 6 | Algorithm-only, no local struct | 4 | Chap58: BellmanFordStEphI64, Chap59: JohnsonStEph/MtEphI64 |

**Actual work: 16 files need wf added. 7 files have no struct to put wf on.**

## Reference Patterns

Already-verified wf patterns to mirror:

- **Chap52 AdjMatrixGraphStEph**: Free spec fn + abstract trait method + open impl.
  Wf checks square n×n matrix. Threaded through all requires/ensures.
- **Chap56 SSSPResultStEphI64**: Static-parameter trait method (`s: &Self`).
  Wf checks distances.len() == predecessors.len() && source < len.
  Note: wf declared but NOT yet threaded through method requires/ensures.
- **Chap56 AllPairsResultStEphI64**: Static-parameter trait method.
  Wf checks both 2D matrices are n×n.

## Proposed Work

### Phase 1: Chap52 Graph Representations (10 files)

Work Chap52 first because downstream chapters (53-59) depend on graph representations.

#### 1a. AdjSeqGraph (4 files: StEph, StPer, MtEph, MtPer)

**Wf predicate**: All neighbor indices are valid vertex indices.
```rust
// In trait:
spec fn spec_adjseqgraphsteph_wf(&self) -> bool;

// In impl:
open spec fn spec_adjseqgraphsteph_wf(&self) -> bool {
    forall|u: int, j: int|
        0 <= u < self.adj.spec_len()
        && 0 <= j < self.adj.spec_index(u).spec_len()
        ==> #[trigger] self.adj.spec_index(u).spec_index(j) < self.adj.spec_len()
}
```

**Threading**:
- `new()` ensures wf (trivially—all degrees 0, no neighbors).
- `from_seq()` requires valid-neighbor precondition, ensures wf.
- `num_vertices`, `has_edge`, `out_neighbors`, `out_degree` require wf.
- `set_neighbors`, `set_edge` require old(self) wf, ensure self wf.

#### 1b. AdjTableGraph (3 files: StEph, StPer, MtPer)

**Wf predicate**: All edge endpoints are in the vertex domain.
```rust
// In trait:
spec fn spec_adjtablegraphsteph_wf(&self) -> bool;

// In impl:
open spec fn spec_adjtablegraphsteph_wf(&self) -> bool {
    forall|u: V, v: V|
        self.spec_adj().dom().contains(u)
        && #[trigger] self.spec_adj().index(u).contains(v)
        ==> self.spec_adj().dom().contains(v)
}
```

**Threading**:
- `empty()` ensures wf (trivially—no edges).
- `from_table()` requires closed-neighborhood precondition, ensures wf.
- All query methods require wf.
- `insert_vertex`, `delete_vertex`, `insert_edge`, `delete_edge`: require old wf, ensure new wf.

#### 1c. EdgeSetGraph (3 files: StEph, StPer, MtPer)

**Wf predicate**: All edge endpoints are in the vertex set.
```rust
// In trait:
spec fn spec_edgesetgraphsteph_wf(&self) -> bool;

// In impl:
open spec fn spec_edgesetgraphsteph_wf(&self) -> bool {
    forall|u: V, v: V|
        #[trigger] self.spec_edges().contains((u, v))
        ==> self.spec_vertices().contains(u) && self.spec_vertices().contains(v)
}
```

**Threading**: Same pattern as AdjTable.

### Phase 2: Chap53 PQMin Results (2 files)

#### PQMinStEph, PQMinStPer

**Wf predicate**: Result consistency—visited vertices have priorities.
```rust
spec fn spec_pqminsteph_wf(&self) -> bool;

// Reasonable invariant: visited set is finite.
// (priorities and parent are AVLTreeSets which are inherently finite.)
open spec fn spec_pqminsteph_wf(&self) -> bool {
    self.visited@.finite()
}
```

**Threading**: `pq_min` and `pq_min_multi` ensure wf on the returned result.

### Phase 3: Chap56 F64 Result Structures (4 files)

Mirror the existing I64 patterns exactly.

#### AllPairsResultStEphF64, AllPairsResultStPerF64

```rust
spec fn spec_allpairsresultstephf64_wf(s: &AllPairsResultStEphF64) -> bool;

open spec fn spec_allpairsresultstephf64_wf(s: &AllPairsResultStEphF64) -> bool {
    s.distances.spec_len() == s.n as nat
    && s.predecessors.spec_len() == s.n as nat
    && forall|r: int| #![trigger s.distances.spec_index(r)]
        0 <= r < s.n ==> s.distances.spec_index(r).spec_len() == s.n as nat
    && forall|r: int| #![trigger s.predecessors.spec_index(r)]
        0 <= r < s.n ==> s.predecessors.spec_index(r).spec_len() == s.n as nat
}
```

#### SSSPResultStEphF64, SSSPResultStPerF64

```rust
spec fn spec_ssspresultstephf64_wf(s: &SSSPResultStEphF64) -> bool;

open spec fn spec_ssspresultstephf64_wf(s: &SSSPResultStEphF64) -> bool {
    s.distances.seq@.len() == s.predecessors.seq@.len()
    && s.source < s.distances.seq@.len()
}
```

### Phase 4: Files with No Work Needed

These files lack data structures to put wf on:

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 | 57 | StackStEph.rs | Vec wrapper — no structural invariant beyond Vec itself |
| 2 | 58 | BellmanFordStEphF64.rs | Placeholder (blocked on WeightedDirGraphStEphF64) |
| 3 | 58 | BellmanFordStEphI64.rs | Algorithm trait only — returns SSSPResultStEphI64 (Chap56) |
| 4 | 59 | JohnsonStEphF64.rs | Placeholder (blocked on f64 graph types) |
| 5 | 59 | JohnsonMtEphF64.rs | Placeholder (blocked on f64 graph types) |
| 6 | 59 | JohnsonStEphI64.rs | Algorithm trait only — returns AllPairsResultStEphI64 (Chap56) |
| 7 | 59 | JohnsonMtEphI64.rs | Algorithm trait only — returns AllPairsResultStEphI64 (Chap56) |

## Execution Order

1. **Chap52 AdjSeqGraph** (4 files) — simplest, one struct field
2. **Chap52 AdjTableGraph** (3 files) — generic types, more complex wf
3. **Chap52 EdgeSetGraph** (3 files) — similar to AdjTable
4. Validate Chap52 batch
5. **Chap53 PQMin** (2 files) — result structs
6. Validate Chap53
7. **Chap56 F64 results** (4 files) — mechanical mirror of I64
8. Validate Chap56
9. Final validate + rtt + ptt
10. Fix trigger warnings
11. Commit

## Risks

- **AdjSeqGraph wf may break downstream callers** if `from_seq` gains a precondition.
  Mitigation: grep for callers before adding precondition; if found, may need to add
  wf only to ensures (not requires on from_seq).
- **Trigger warnings** from new quantifiers in wf predicates. Fix immediately per policy.
- **AdjTable/EdgeSet wf quantifiers** may need careful trigger selection to avoid
  matching loops.

## Questions for User

1. **StackStEph**: The plan lists it, but `StackStEph` wraps `Vec<T>` with no structural
   invariant. Add a trivially-true wf, or skip it?
2. **AdjSeqGraph wf content**: Should wf check "all neighbor indices < num_vertices"
   (meaningful graph invariant, may break callers) or something simpler?
3. **Chap58/59 algorithm files**: These have no data structures — just algorithm traits
   returning Chap56 result types. Confirm skip?
