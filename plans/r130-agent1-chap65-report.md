# R130 Agent 1 — Chap65 UnionFindStEph + KruskalStEph Report

## Summary

Proved all 11 holes in Chap65 (9 in UnionFindStEph, 2 in KruskalStEph).
Chap65 is now fully clean.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 65 | UnionFindStEph.rs | 9 | 0 | -9 |
| 2 | 65 | KruskalStEph.rs | 2 | 0 | -2 |
| **Total** | | | **11** | **0** | **-11** |

## Techniques

### UnionFindStEph.rs (9 holes eliminated)

**1. Proved `union` trait method (removed external_body + 2 rank assumes = -3 holes)**

The `union` method had `external_body` with two `assume(rank[root] < elements.len())`
inside. The rank bound is provable from `spec_rank_lt_elements` (a sub-predicate of wf).
Added `lemma_root_rank_lt_elements` helper that chains:
`roots.contains_key(root) → rank.contains_key(root) → rank[root] < elements.len()`.

For the same-root case (no merge needed): added `lemma_union_result_identity` that
proves `spec_union_result(roots, roots, u, v)` when `roots[u] == roots[v]`.

For the different-root case: used existing `lemma_prove_union_result` to bridge from
`spec_roots_changed_by_merge` to `spec_union_result`.

**2. Fixed experiment functions (removed 6 assumes = -6 holes)**

Both `union_experiment_merge_no_wf_ensures` and `union_experiment_merge_with_wf` had
3 assumes each (`root_u != root_v`, two rank bounds). Added the `if !feq(...)` guard
(matching the real `union` pattern) and replaced rank assumes with
`lemma_root_rank_lt_elements` calls.

**3. Added wf to union_merge_exec requires (fixed fn_missing_wf_requires warning)**

Added `old(uf).spec_unionfindsteph_wf()` to `union_merge_exec` requires. The function
body doesn't reveal wf (closed predicate = opaque boolean in Z3), so no solver impact.

### KruskalStEph.rs (2 holes eliminated)

**4. Removed opaque from opaque_spec_unionfindsteph_wf (-1 opaque hole)**

The `#[verifier::opaque]` wrapper was redundant because `spec_unionfindsteph_wf` is
already a `closed spec fn`. Double opacity was unnecessary — removing the opaque marker
lets Z3 see through the open wrapper to the closed predicate automatically.

**5. Proved kruskal_process_edge (removed external_body = -1 hole)**

With the opaque removed and `union` proved, `kruskal_process_edge` verifies
automatically — the trait ensures of `equals`, `union`, and `insert` chain directly.

## Verification

- `scripts/validate.sh isolate Chap65`: 2478 verified, 0 errors
- `scripts/rtt.sh`: 3536 passed
- `scripts/ptt.sh`: 221 passed
- `scripts/holes.sh src/Chap65/`: 0 holes

## Global Status

- Clean chapters: 41 (was 40)
- Holed chapters: 5 (was 6)
- Global holes: 9 (was 20, delta -11)
