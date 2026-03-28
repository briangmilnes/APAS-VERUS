# R98 Agent 1 — Switch AdjTableGraphMtPer to insert_wf, STEP 20

## Objective

Agent2 R97 added `insert_wf` to OrderedTableMtPer. Now switch
AdjTableGraphMtPer from `insert` to `insert_wf` and remove stored-value-wf
assumes — same pattern agent1 R97 did for StEph/StPer.

## What to do

Find every `self.adj.insert(key, value)` in AdjTableGraphMtPer.rs. Replace
with `self.adj.insert_wf(key, value)` where the combine closure uses
`clone_wf()` and the ensures include wf preservation.

Target functions:
- `insert_vertex` — inserts empty set
- `insert_edge` — inserts neighbor set with combine
- `delete_edge` — inserts modified neighbor set

For each, remove the `assume(... spec_avltreesetmtper_wf())` or
`assume(updated.spec_adjtablegraphmtper_wf())` that follows.

## Technique (from R97 agent1 on StEph/StPer)

Agent1 R97 strengthened Clone::clone for AVLTreeSetStEph/StPer to conditionally
preserve wf. Check if AVLTreeSetMtPer::clone has similar ensures. If not, you
may need to use `clone_wf()` directly in the combine closure, or strengthen
MtPer's Clone similarly.

Use `lemma_cloned_view_eq` (from agent3 R97) to bridge the clone-view gap
for generic V in domain containment proofs.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — your file
- `src/Chap52/AdjTableGraphStEph.rs` — R97 agent1's insert_wf pattern (template)
- `src/Chap43/OrderedTableMtPer.rs` — insert_wf signature
- `src/Chap41/AVLTreeSetMtPer.rs` — Clone ensures, clone_wf if available

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 20

## Report

Write `plans/agent1-r98-mtper-insert-wf-report.md`.
