# R81 Agent 2 — Prove `kruskal_mst` in KruskalStEph, STEP 20

## Objective

Remove `external_body` from `kruskal_mst` in `src/Chap65/KruskalStEph.rs:178` and prove it.

## Context

The body (lines 179-293) is already fully written:
1. Create empty MST edge set, create empty UF
2. Insert all graph vertices into UF (loop over `vertices.iter()`)
3. Collect edges into a Vec (loop over `labeled_edges.iter()`)
4. Sort edges by weight (`sort_edges_by_weight`, already proved)
5. Greedy loop: for each sorted edge, if endpoints not in same component (`equals`), add edge to MST and `union` the components

The ensures (line 188) is just `mst_edges.spec_setsteph_wf()` — the MST set is well-formed.

## What's already there

The body has loop invariants and proof blocks already written (lines 198-293). The vertex
insertion loop, edge collection loop, sort call, and greedy loop all have invariants.
The proof block at lines 268-283 establishes edge endpoint membership in UF via graph wf.

All called functions have full specs:
- `UnionFindStEph::new`, `insert`, `find`, `equals`, `union` — all have requires/ensures
- `SetStEph::empty`, `insert` — all have requires/ensures
- `sort_edges_by_weight` — proved, ensures sorted + permutation
- `vertices`, `labeled_edges`, `iter`, `next` — all have specs
- `clone_view` — preserves view

## What to do

1. Remove `#[verifier::external_body]` from line 178
2. Run validate, see what Verus complains about
3. Fix loop invariants and proof blocks as needed

The main challenge will likely be:
- **Vertex insertion loop**: proving all vertices end up in UF. The invariant at line 203-204
  tracks that elements `vseq[0..vit@.0]` are in UF. After the loop, need
  `forall|v| graph@.V.contains(v) ==> uf@.parent.contains_key(v)`. The connection goes
  through `vseq.map(|i, k| k@).to_set() == vertices@` (line 205).
- **Edge collection**: straightforward, invariant already tracks `edges_vec@[j] == eseq[j]`.
- **Greedy loop**: the key invariants are `uf.spec_unionfindsteph_wf()` and
  `mst_edges.spec_setsteph_wf()`. Both are maintained by their respective operations'
  ensures. The `union` call needs `uf@.parent.contains_key(u@)` and `contains_key(v@)`,
  which the proof block at 268-283 establishes from graph wf.
- **Post-loop**: `mst_edges.spec_setsteph_wf()` is the loop invariant, so it holds at exit.

## The `equals` return type

`equals` returns `B` (a bool). When it returns false, the endpoints are in different
components. The `equals` ensures says `same == (old(self)@.roots[u@] == old(self)@.roots[v@])`.
Note `equals` takes `&mut self` (trait signature) but `find` doesn't actually mutate
(no compression). The ensures guarantee `self@.roots =~= old(self)@.roots` and
`self@.parent.dom() =~= old(self)@.parent.dom()`, so UF state is preserved through `equals`.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken the ensures clause.
- Do NOT modify `sort_edges_by_weight` — it's already proved.
- Do NOT modify `UnionFindStEph.rs` — union's spec is sufficient even though it's external_body.

## STEP 20

At most 20 edit/verify iterations. Then stop and report.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`.
Push to `agent2/ready`.

## Report

Write `plans/agent2-round81-report.md` with holes before/after (table with Chap column).
