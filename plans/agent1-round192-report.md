# Agent 1 — Round 192 Report

## Task

Micro-split lemma_compress_step_wf to eliminate remaining matching loop.

## Results

| # | Metric | R191 | R192 |
|---|--------|------|------|
| 1 | Verified | 713 | 716 |
| 2 | Errors | 2 | 1 |
| 3 | Peak Z3 RSS | 3-7 GB | 2.2-4.7 GB |

## Changes

1. Split `lemma_compress_step_wf` into 3 micro-lemmas:
   - `lemma_compress_basic` — domain finiteness, length, rank-domain, rank bounded, root validity.
   - `lemma_compress_parent_in_dom` — parent-in-domain only. No rank quantifiers.
   - `lemma_compress_rank_inv` — rank invariant only. No parent-in-domain quantifiers.
   Each uses empty `by {}` blocks — Z3 handles the simple case splits from the pn
   characterization (insert axioms) without explicit hints.

2. Removed explicit pn characterization assert-foralls from the compression loop body.
   Z3 derives `pv(pn, k) == pv(po, k)` and domain equality from Map::insert axioms
   directly when checking micro-lemma preconditions.

3. Restored orig-wf foralls in compression loop invariant (was removed in failed
   experiment). Key insight: orig foralls trigger on `orig_parent.dom().contains(k)`
   while current foralls trigger on `self.parent@.dom().contains(k)` — different maps,
   no cross-pollination. Z3 RSS unchanged (2.3 GB) after adding them back.

4. Removed dead `lemma_find_preserved_size_rank_inv` (commented out per CLAUDE.md).
   Proof inlined at return point.

5. Fixed remaining trigger warnings at lines 1070, 1170.

## Remaining 1 error

| # | Location | Error | Z3 RSS | Root cause |
|---|----------|-------|--------|------------|
| 1 | find() while loop | rlimit | 2-5 GB | Compression loop body has 5 lemma calls + 2 assert-foralls. The `spec_pure_find` triggers in the find-preservation forall create a slow (but bounded-memory) matching chain. |

## Z3 RSS scaling

| rlimit | Z3 RSS | Elapsed |
|--------|--------|---------|
| 30 | 2.3 GB | 31s |
| 50 | 2.2 GB | 38s |
| 80 | 4.8 GB | 55s |
| 150 | 4.7 GB | 87s |
| 300 | 9.3 GB | 149s |

RSS grows linearly — unbounded matching loop on `spec_pure_find` triggers,
but at a much lower rate than before the micro-split.

## Path forward

The remaining matching loop is from `spec_pure_find` triggers in the
find-preservation invariant interacting with the loop body's lemma results.
Options:
1. Factor the find-propagation forall into its own lemma (reduces loop body context).
2. Use `assert ... by { ... }` isolation to prevent find-preservation triggers
   from leaking into the rest of the loop body.
3. Accept the high rlimit (500+) — the RSS stays bounded at ~5GB for rlimit ≤ 150.
