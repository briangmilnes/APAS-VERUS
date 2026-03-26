# R80 Agent 1 — UnionFind find: finish the proof (Chap65)

## Objective

You have been working on `find` in UnionFindStEph.rs. You have uncommitted work including:
- Named wf sub-predicates (13 spec fns)
- `lemma_non_root_rank_lt_root`
- `lemma_compression_preserves_wf`
- `find_root_loop` helper
- Removed `external_body` from `find`

**Finish the proof.** Get `find` to verify. Then cascade to `union`, `equals`, `num_sets`.

## Current state

- 4908 verified, 0 errors on main
- Your uncommitted changes: ~330 lines added to UnionFindStEph.rs
- Last validate either didn't run or timed out

## What's likely wrong

If validation is failing, the most likely issues are:

1. **Z3 rlimit exhaustion** in `lemma_compression_preserves_wf`. Fix: use `#[verifier::rlimit(50)]`
   or higher. Assert each named sub-predicate individually inside the proof body rather than
   expecting Z3 to derive wf from its conjuncts.

2. **Path compression loop invariant incomplete**. The compression loop changes `parent[v]`
   for each node on the path. The invariant must track:
   - Which nodes have been compressed (a ghost set of processed nodes)
   - `spec_parent_preserves_root` still holds (parent[v]@ → root, so roots[parent[v]@] == roots[v])
   - `spec_rank_ordering` still holds (rank[v] < rank[root] for compressed nodes)
   - All other sub-predicates unchanged (elements, roots, rank maps not modified)

3. **Clone equality** in the loop body. Each `parent.get().unwrap().clone()` needs equality
   established. Use the 3-clone + original pattern from `insert` (line 328).

## Strategy

1. Run `scripts/validate.sh` first to see current errors.
2. Read the errors carefully. They will tell you which sub-predicate fails.
3. Fix one sub-predicate at a time. Assert it, validate, move to the next.
4. Once `find` verifies, `equals` should be trivial (two finds, compare roots).
5. `union` needs its own frame lemma (changes parent + rank of one root).
6. `num_sets` is a counting loop over `find` results.

## Reference: lemma_insert_preserves_wf (line 91)

This is the proven pattern. One frame `assert forall` for unchanged keys, then
per-property assertions. Copy this structure for compression and union.

## Cascade after find

| Function | Difficulty | Notes |
|----------|-----------|-------|
| equals | Easy | Two finds, compare V results |
| num_sets | Medium | Loop with set of seen roots, count distinct |
| union | Hard | Two finds, then rank-based merge, frame lemma on parent+rank update |
| kruskal_mst | Medium | Loop calling insert/find/union/equals with graph edges |

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round80-report.md` with holes before/after (table with Chap column).
