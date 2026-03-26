# Agent 1 Round 80b Report

## Summary

Proved `find`, `equals`, and `num_sets` in UnionFindStEph.rs (Chap65).
Factored the wf predicate into 13 named sub-predicates for future lemma work.
Path compression deferred — `find` uses root-chasing without mutation for now.

## Verification

- 4912 verified, 0 errors, 0 warnings
- 2774 RTT passed
- 157 PTT passed

## Holes Before/After

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 65 | UnionFindStEph.rs | find | external_body | verified | No compression; delegates to find_root_loop |
| 2 | 65 | UnionFindStEph.rs | equals | external_body | verified | Uses find_root_loop directly (no mutation) |
| 3 | 65 | UnionFindStEph.rs | num_sets | external_body | verified | Loop with find_root_loop + HashSet |
| 4 | 65 | UnionFindStEph.rs | union | external_body | external_body | Needs frame lemma for parent+rank+roots update |
| 5 | 65 | KruskalStEph.rs | kruskal_mst | external_body | external_body | Blocked by union |

Holes: 5 → 2 (−3)

## Techniques

- **Named wf sub-predicates**: Factored 14-conjunct wf into 13 named `pub open spec fn`
  (conjunct 3 folded into `spec_feq_full`). Keeps the monolithic wf definition for callers
  (avoids solver overhead from 14 function-call unfoldings). Named specs available for
  future targeted lemmas.
- **find_root_loop**: Standalone `fn` taking `&UnionFindStEph<V>` (immutable). While loop
  with `feq` in condition so self-parent fact is available after exit. Decreases by rank
  difference to root.
- **No-mutation find**: Without path compression, `find` delegates to `find_root_loop`
  which takes `&self`. This lets `equals` and `num_sets` avoid `&mut self` state chaining.
- **Clone equality via strictly_cloned**: Explicit `assert(strictly_cloned(orig, clone))`
  + `assert(obeys_feq_full::<V>())` to establish view equality after clone.
- **Path compression commented out**: `compress_path` + 4 sub-lemmas + assembly lemma
  written but commented out. Each sub-lemma proves one parent-dependent wf conjunct.
  Assembly lemma decomposed requires to avoid wf context explosion. Needs rlimit work
  to finish.

## What Blocks Union

Union modifies parent (one root points to another), rank (equal-rank case), and roots
(all elements in both components get a new common root). Proving wf preservation requires
a frame lemma similar to compression but with the additional rank update case. The ghost
`roots` field must be updated with a `Map::new` construction. Estimated 4-6 sub-lemmas
following the same pattern as the compression proof.

## What Blocks Kruskal

Kruskal calls insert/find/union/equals in a loop over sorted edges. The loop invariant
needs graph vertex coverage (all vertices in UF) and MST edge provenance. Union's ensures
(roots merging) feeds the cycle-detection logic. Once union is proved, kruskal should be
medium difficulty.
