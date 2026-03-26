# R84 Agent 3 — Prove kruskal_mst (1 hole), STEP 20

## Objective

Remove `external_body` from `kruskal_mst` in `KruskalStEph.rs:178`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh`.
Push to `agent3/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Context

The function body is already written (lines 179-293) with loop invariants and
proof blocks. All called functions have full specs with requires/ensures:

- `UnionFindStEph::new`, `insert`, `find`, `equals`, `union` — all have specs
  (union is external_body but its ensures are correct and usable)
- `SetStEph::empty`, `insert` — all have specs
- `sort_edges_by_weight` — already proved
- `vertices`, `labeled_edges`, `iter`, `next` — all have specs
- `clone_view` — preserves view

## What to do

1. Read `src/Chap65/KrustalStEph.rs` fully.
2. Remove `#[verifier::external_body]` from line 178.
3. Run `validate.sh isolate Chap65`, see what Verus complains about.
4. Fix loop invariants and proof blocks as needed.

The ensures (line 188) is `mst_edges.spec_setsteph_wf()` — the MST set is well-formed.

## Likely challenges

- **Vertex insertion loop** (lines 198-213): proving all vertices end up in UF.
  The invariant tracks that `vseq[0..vit@.0]` are in UF. After the loop,
  need `forall|v| graph@.V.contains(v) ==> uf@.parent.contains_key(v)`.

- **Edge collection loop** (lines 224-238): straightforward, invariant already
  tracks `edges_vec@[j] == eseq[j]`.

- **Greedy loop** (lines 246-290): key invariants are `uf.spec_unionfindsteph_wf()`
  and `mst_edges.spec_setsteph_wf()`. Both maintained by their operations' ensures.
  The proof block at lines 268-283 establishes edge endpoint membership via graph wf.

- **equals return**: `equals` takes `&mut self` but preserves all state. Its ensures
  guarantee `self@.roots =~= old(self)@.roots` and `self@.parent.dom() =~= old(self)@.parent.dom()`.

- **Post-loop**: `mst_edges.spec_setsteph_wf()` is the loop invariant, holds at exit.

## Important

- Do NOT modify `UnionFindStEph.rs` — another agent is refactoring it.
  Union's external_body spec is what you use.
- Do NOT modify `PrimStEph.rs`.
- Do NOT add `assume` or `accept`.
- Do NOT weaken the ensures clause.

## STEP 20

## Report

Write `plans/agent3-round84-report.md`.
