# R80b Agent 1 — UnionFind find: drop path compression, STEP 20

## Objective

Drop path compression from `find`. The spec doesn't care — `find` returns `roots[v@]`
either way. Compression is a performance optimization that costs 300+ lines of proof
for zero spec benefit.

## What to do

Keep all the compression lemmas — they're good work for later. Write a simpler `find`
that skips compression for now.

1. Rewrite the `find` body to just call `find_root_loop` (already proven, no mutation):
   ```rust
   fn find(&mut self, v: &V) -> (root: V) {
       find_root_loop(self, v)
   }
   ```
   Note: `find` takes `&mut self` (trait signature). That's fine — it just doesn't
   mutate. The ensures still hold: `root@ == self@.roots[v@]`.

2. Validate. `find` should verify immediately.

3. Cascade: prove `equals`, `union`, `num_sets`, `kruskal_mst` with the remaining steps.

Do NOT delete `compress_path`, `lemma_compress_*`, or `lemma_compression_preserves_wf`.
Leave them in the file for future use.

## STEP 20

At most 20 edit/verify iterations. Then stop and report what verified.

## Priority order

1. `find` — should be 1 step (delete compression, delegate to find_root_loop)
2. `equals` — two finds, compare roots, trivial
3. `num_sets` — loop with find, count distinct roots
4. `union` — two finds, rank-based merge, needs frame lemma on parent+rank update
5. `kruskal_mst` — loop calling insert/find/union/equals

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round80b-report.md` with holes before/after (table with Chap column).
