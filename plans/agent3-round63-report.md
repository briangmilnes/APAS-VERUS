# Agent 3 — Round 63 Report

## Objective

Close 1 proof hole in `src/Chap53/PQMinStEph.rs`: the `assume(frontier_updated@.len() + 1 < usize::MAX as nat)` at line 269 inside `pq_explore`'s inner loop.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 53 | PQMinStEph.rs | 1 | 0 | -1 |
| 2 | 53 | PQMinStPer.rs | 1 | 1 | 0 |

**Chapters closed:** None (PQMinStPer.rs still has 1 hole).

## Verification Counts

| Metric | Before | After |
|--------|--------|-------|
| Verified | 4504 | 4504 |
| Errors | 0 | 0 |
| Trigger warnings | 0 | 0 |
| Holes (total) | 6 | 5 |
| RTT | 2610 | 2610 |
| PTT | 147 | 147 |

## Technique: Injection-Based Cardinality Bound

The assume guarded `union` with a singleton, which requires `self@.len() + other@.len() < usize::MAX`. The frontier set contains `Pair(Pair(priority, vertex), vertex)` entries. The proof shows `frontier.len() <= vertex_universe.len()` via:

1. **View-determinism requires** added to `pq_min`, `pq_min_multi`, and `pq_explore`: same vertex view implies same priority view under `priority_fn`.

2. **Frontier canonical form invariant**: every entry `((p, v_inner), v_outer)` satisfies `v_inner == v_outer` and `p` comes from `priority_fn`. With view-determinism, the projection `e -> e.1` (vertex component) is injective on the frontier.

3. **vstd lemmas**: `lemma_map_size(frontier@, image, proj)` gives `frontier.len() == image.len()`, and `lemma_len_subset(image, vertex_universe)` gives `image.len() <= vertex_universe.len()`. Since `vertex_universe.len() + 1 < usize::MAX` (from requires), the capacity bound follows.

4. **clone_plus** instead of `clone`: regular `Clone::clone` has no Verus ensures about view equality. Changed to `ClonePlus::clone_plus()` which provides `ensures cloned(*self, res)`, enabling the canonical form proof through feq axioms.

## Key Changes

- Added view-determinism requires to trait signatures and all free functions.
- Added frontier canonical form invariants to outer loop, inner loop, and `pq_min_multi` initial loop.
- Replaced `assume` with injection proof using `vstd::relations::injective_on`, `vstd::set_lib::lemma_map_size`, `vstd::set_lib::lemma_len_subset`.
- Changed `clone()` to `clone_plus()` for `neighbor` and `neighbor_p` in both `pq_explore` and `pq_min_multi`.
- Added `obeys_feq_full_trigger::<P>()` assertions alongside existing `<V>` triggers.
- All trigger annotations explicit — zero auto-chosen trigger warnings.

## Remaining Holes (Chap53)

| # | Chap | File | Holes | What Blocks It |
|---|------|------|-------|----------------|
| 1 | 53 | PQMinStPer.rs | 1 | Same assume pattern; needs AVLTreeSetStPer analog of this proof |

## Global Hole Summary (5 remaining)

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 43 | OrderedSetMtEph.rs | 1 |
| 2 | 45 | (unknown) | 1 |
| 3 | 47 | ParaHashTableStEph.rs | 2 |
| 4 | 53 | PQMinStPer.rs | 1 |
