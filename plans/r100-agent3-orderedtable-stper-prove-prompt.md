# R100 Agent 3 — Prove OrderedTableStPer insert_wf/delete_wf external_body, STEP 20

## Objective

Chap43 has 7 holes — all external_body on OrderedTableStPer and OrderedTableMtPer.
The StPer versions (insert_wf, delete_wf) delegate to the underlying insert/delete
but were left external_body because the proof wasn't attempted. Try to prove them.

## The 2 StPer external_body

| # | File | Function | Why external_body |
|---|------|----------|------------------|
| 1 | OrderedTableStPer.rs | insert_wf | "Delegates to insert, can't prove value ensures" |
| 2 | OrderedTableStPer.rs | delete_wf | Same |

## Approach

`OrderedTableStPer` wraps a BST (AVLTreeSeqStPer or similar). The insert/delete
operations rebuild the tree. `insert_wf` needs to prove that stored values retain
wf after the rebuild.

The key: `OrderedTableStPer::insert` already has strong ensures (domain, value
mapping). `insert_wf` adds stored-value-wf. If the underlying BST clone preserves
wf (which we proved in R93 via ClonePreservesWf), the insert_wf proof may go through.

Read the existing `insert_wf` body — it may already be close to proving. The
external_body might have been premature.

## Read first

- `src/Chap43/OrderedTableStPer.rs` — insert_wf and delete_wf
- `src/Chap42/TableStPer.rs` — TableStPer insert_wf (fully proved, pattern reference)

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify OrderedTableMtPer (agent2 is working there).
- If the proof is genuinely hard (BST rotation structure), leave external_body
  and report what blocks it.

## STEP 20

## Report

Write `plans/agent3-r100-orderedtable-stper-report.md`.
