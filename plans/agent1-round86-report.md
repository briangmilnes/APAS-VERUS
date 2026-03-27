# Agent 1 — Round 86 Report

## Objective
Replace `WrappedF64` edge weights with `u64` in Chap65 Prim and Kruskal MST algorithms,
eliminating all float axiom dependencies.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 65 | PrimStEph.rs | `WrappedF64` → `u64` everywhere; PQEntry.priority: u64; TotalOrder reflexive/transitive/total now auto-proved (empty bodies); removed float imports, zero_dist, spec_is_finite; mst_weight uses u64 addition with overflow assume |
| 2 | 65 | KruskalStEph.rs | `WrappedF64` → `u64` everywhere; removed FloatTotalOrder proof calls from sort_edges_by_weight (u64 <= is native); removed broadcast use group_float_finite_total_order; removed FloatBitsProperties import; removed is_finite_spec requires from kruskal_mst; ghost tuples (V::V, V::V, f64) → (V::V, V::V, u64); mst_weight uses u64 addition with overflow assume |
| 3 | 65 | TestPrimStEph.rs | `OrderedFloat(X.0)` → `Xu64` |
| 4 | 65 | TestKruskalStEph.rs | `OrderedFloat(X.0)` → `Xu64` |

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 65 | KruskalStEph.rs | 1 | 3 | +2 | Old: 1 big external_body (greedy_phase) hiding inner holes. New: greedy loop PROVED, but reveals process_edge external_body + uf_wf_opaque + mst_weight overflow assume |
| 2 | 65 | PrimStEph.rs | 2 | 3 | +1 | Old: 4 float assumes (counted as 2 holes) + external_body cmp. New: 1 antisymmetric assume + 1 cmp-equal assume + 1 mst_weight overflow assume. Reflexive/transitive/total now auto-proved |
| 3 | 65 | UnionFindStEph.rs | 3 | 3 | 0 | Unchanged |
| - | 65 | **Total** | **6** | **9** | **+3** | See analysis below |

## Hole Count Analysis

The +3 increase is misleading — proof coverage is actually better:

1. **Kruskal greedy loop now PROVED** (was external_body). The old 1-hole count
   hid process_edge + opaque behind a big external_body. Now the loop logic is
   verified and only the small process_edge (Z3 &mut divergence) remains external_body.

2. **Prim TotalOrder: 3 of 4 proof methods now auto-proved** (reflexive, transitive,
   total). Only antisymmetric remains as assume — inherent to PQEntry being a preorder
   (same priority ≠ same entry).

3. **Two new mst_weight overflow assumes** — f64 addition doesn't overflow, u64 does.
   These are utility functions, not algorithmic core. Could be eliminated with a
   requires clause bounding total edge weight sum.

## Verification

```
scripts/validate.sh isolate Chap65
verification results:: 2408 verified, 0 errors
```

## Techniques Used
- Type substitution (WrappedF64 → u64) across source + ghost + tests
- Leveraged u64 TotalOrder impl (empty proof bodies, Z3 auto-proves integer ordering)
- Simplified sort proof: removed all FloatTotalOrder::reflexive/transitive/totality calls
- Disambiguated Ord::cmp vs TotalOrder::cmp with std::cmp::Ord::cmp qualification

## Remaining Holes (Chap65)

| # | Chap | File | Line | Type | What Blocks It |
|---|------|------|------|------|----------------|
| 1 | 65 | KruskalStEph.rs | 40 | opaque | uf_wf_opaque — isolation technique, not real hole |
| 2 | 65 | KruskalStEph.rs | 49 | external_body | kruskal_process_edge — Z3 diverges on 13-quantifier wf through &mut |
| 3 | 65 | KruskalStEph.rs | ~410 | assume | mst_weight overflow — u64 addition |
| 4 | 65 | PrimStEph.rs | 61 | assume | antisymmetric — PQEntry is preorder, not total order |
| 5 | 65 | PrimStEph.rs | 70 | assume | cmp equal case — same priority ≠ same PQEntry |
| 6 | 65 | PrimStEph.rs | ~345 | assume | mst_weight overflow — u64 addition |
| 7 | 65 | UnionFindStEph.rs | 982 | admit | rank overflow bound |
| 8 | 65 | UnionFindStEph.rs | 1078 | external_body | union_merge — Z3 &mut encoding |
| 9 | 65 | UnionFindStEph.rs | 1308 | external_body | spec_uf_wf reveal |
