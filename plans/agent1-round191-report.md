# Agent 1 — Round 191 Report

## Task

Split lemma_compress_step to kill Z3 matching loop. Fix triggers.

## Results

| # | Metric | R190 | R191 |
|---|--------|------|------|
| 1 | Verified | 746 | 713 |
| 2 | Errors | 2 | 2 |
| 3 | Peak Z3 RSS | 27 GB | 3-7 GB |

Verified count dropped because compression loop invariant was changed from
full wf to light wf (no size_rank_inv in loop). Functions that were trivially
verified with full wf now need explicit size_rank_inv restoration at return
points.

## Changes

1. Split `lemma_compress_step` into `lemma_compress_step_wf` (rank-only) and
   `lemma_compress_step_find` (find-only). Eliminates the 27GB matching loop
   from rank-find quantifier cross-pollination.
2. Added `lemma_compress_preserves_find_all` wrapper — wraps the per-z
   lemma into a forall. Called from compress_step_find.
3. Moved `rank[curr] < rank[root]` to lemma_compress_step_wf requires.
   Caller calls lemma_rank_lt_find before lemma_compress_step_wf. This
   keeps the recursive rank lemma OUT of the forall body.
4. Inlined size_rank_inv proof at the `return root` point inside find().
   Uses `lemma_find_is_root` on orig state + subset reasoning + 
   `lemma_len_subset` for finiteness.
5. Fixed 13 auto-trigger warnings: explicit `#[trigger]` on all `pv(pn, k)`,
   `po.dom().contains(z)`, `st_new.contains(k)` quantifiers.
6. Removed lemma_find_preserved_size_rank_inv (commented out) — postcondition
   evaluation can't unfold spec_subtree through spec_pure_find's decreases_when
   guard. Proof is inlined at call site instead.

## Remaining 2 errors

| # | Location | Error | Z3 RSS | Root cause |
|---|----------|-------|--------|------------|
| 1 | lemma_compress_step_wf | rlimit | 3-7 GB | po wf quantifiers (parent-in-domain, rank invariant) create matching chain with pn characterization foralls |
| 2 | find() compression loop | rlimit | cascade | From error 1 |

## Path forward

Split lemma_compress_step_wf into 3 micro-lemmas:
1. `lemma_compress_parent_in_dom` — proves only pn parent-in-domain
2. `lemma_compress_rank_inv` — proves only pn rank invariant
3. `lemma_compress_basic` — domain finiteness/length/root

Each has minimal requires (only the specific po wf it needs). This prevents
cross-pollination between the parent-in-domain chain and the rank chain.
