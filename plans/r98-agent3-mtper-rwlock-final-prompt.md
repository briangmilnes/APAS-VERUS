# R98 Agent 3 — Final push on MtPer rwlock assumes, STEP 20

## Objective

Prove the remaining rwlock:predicate assumes in AdjTableGraphMtPer using
the full toolkit now available:
- OrderedTableMtPer find/insert/delete/insert_wf ensures
- feq clone-view lemma (lemma_cloned_view_eq)
- Graph closure + finiteness → subset → wf technique
- adj.clone() instead of self.clone()

## The 5 rwlock assumes

Read `src/Chap52/AdjTableGraphMtPer.rs` and find all `rwlock:predicate` assumes.
For each:
1. Classify: is it neighbor-set-wf (find result) or graph-wf (after mutation)?
2. For neighbor-set-wf: use graph closure + finiteness chain (agent1 R96 technique)
3. For graph-wf: if ICE-blocked, leave with comment. If provable via insert_wf
   ensures chain, prove it.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — your file
- Agent1 R96 report — graph closure + finiteness technique
- Agent3 R97 report — feq clone-view lemma technique

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify Chap42 or Chap43 files.
- Do NOT add new assumes.
- If an assume is genuinely ICE-blocked, reclassify from `rwlock:predicate`
  to `algorithmic` with `// blocked by Verus ICE` comment.

## STEP 20

## Report

Write `plans/agent3-r98-rwlock-final-report.md`.
