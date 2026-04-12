# Agent 1 — Round 190 Report

## Task

Add path compression to HashMap UnionFind (UnionFindPCStEph.rs).

## Results

| # | Metric | Value |
|---|--------|-------|
| 1 | Verified | 746 |
| 2 | Errors | 2 |

## Changes

1. Created `src/UnionFind/UnionFindPCStEph.rs` — HashMap UnionFind with
   two-pass path compression (CLRS §21.3).
2. Added to `src/lib.rs`: `pub mod UnionFindPCStEph` in UnionFind block.
3. Struct `UnionFindPC<V>` — same fields as UnionFind.
4. Same spec fns: pv, spec_pure_find, spec_is_root_map, spec_subtree,
   spec_size_rank_inv_map, spec_uf_wf (self-contained, no cross-module deps).
5. Same lemmas: lemma_find_in_dom, lemma_find_is_root, lemma_find_after_link,
   lemma_find_insert_unchanged, lemma_rank_lt_n_minus_1.
6. NEW `lemma_rank_lt_find` — for non-roots, rank[v] < rank[find(v)].
   Used by compression proof to show rank[curr] < rank[root].
7. NEW `lemma_compress_preserves_find` — compressing one node (setting
   parent[v] = root where root = find(v)) preserves find(z) for all z.
   Proof by cases: z is root → trivial; z == v → find(v) = root preserved;
   z != v → parent unchanged, recurse. Rank-based decreases.
8. NEW `lemma_compress_step` — wraps single compression step proving:
   parent-in-domain, rank invariant, rank bounded, domain finiteness,
   domain length preserved, find preservation for all z, root validity.
9. Modified `find()`: `&self` → `&mut self`. Two-pass algorithm:
   - Pass 1: chase to root (read-only, same as non-PC).
   - Pass 2: walk from v to root, compressing each node.
   Loop invariant: light wf (no size_rank_inv), find preservation,
   rank tracking for termination.
10. Modified trait: find/equals take `&mut self`.
11. `union_sets()`: calls `&mut self` find. Proves roots remain valid after
    compression via lemma_find_is_root + find preservation.
12. `equals()`: calls find twice, uses find-preservation postcondition
    to chain root equality through both compressions.

## Remaining 2 errors

| # | Location | Error | Root cause |
|---|----------|-------|------------|
| 1 | lemma_compress_step | rlimit exceeded | Matching loop: compress_preserves_find called inside assert-forall creates exponential quantifier instantiation (8GB Z3 RSS) |
| 2 | find() compression loop | rlimit exceeded | Cascades from error 1 — lemma_compress_step postcondition unavailable |

## Root cause analysis

The F* version solves this with `compress_preserves_find_all` — a wrapper
that calls the per-element lemma inside `FStar.Classical.forall_intro`.
In Verus, the equivalent `assert forall|z| ... by { lemma_call(z); }` puts
the lemma's 8+ forall-quantifier requires into Z3's context for EACH z,
causing O(n²) quantifier instantiation.

## Profiling

`--profile` on `lemma_compress_step` shows Z3 RSS peaks at 27GB.
`lemma_compress_preserves_find` (per-element) verifies in 2s.
`lemma_compress_preserves_find_all` (wrapper forall) triggers matching loop
from nested quantifiers in pn wf leaking into assert-forall scope.

## Path forward

The matching loop is in lemma_compress_step, specifically the interaction
between the rank invariant proof (calls lemma_rank_lt_find inside forall)
and the find-preservation call (lemma_compress_preserves_find_all). Both
create quantifier instantiation chains that Z3 cross-pollinates.

Fix: split lemma_compress_step into two SEPARATE proof fns:
1. `lemma_compress_step_wf` — proves parent-in-domain, rank invariant,
   rank bounded, domain finiteness. Does NOT call compress_preserves_find.
2. `lemma_compress_step_find` — takes pn wf as requires, calls
   compress_preserves_find_all. Does NOT re-derive rank invariant.

This prevents the rank-invariant quantifiers from leaking into the
compress-find proof context (the root cause of the matching loop).

Alternatively: use `assert(...) by { ... }` isolation blocks more
aggressively inside a single lemma, checking that Z3 contexts don't
bleed across by-blocks.
