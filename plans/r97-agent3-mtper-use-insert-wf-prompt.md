# R97 Agent 3 — Switch AdjTableGraphMtPer to stronger API, STEP 20

## Objective

Use the strengthened OrderedTableMtPer ensures (R95 agent1) and any new
insert_wf (if agent2 R97 adds it) to prove remaining MtPer assumes.

Focus on the assumes that agent1 R96 couldn't prove — the ones blocked by
"insert doesn't ensure value preservation" or "find doesn't ensure exec-level wf."

## Strategy

Read `src/Chap52/AdjTableGraphMtPer.rs` and categorize each remaining assume:

1. **Provable now** (OrderedTableMtPer has the ensures): prove it
2. **Needs insert value-preservation** (`updated@[k@] == v@`): check if
   OrderedTableMtPer::insert now ensures this (from R95)
3. **ICE-blocked**: leave with comment
4. **Needs insert_wf**: if agent2 adds it, switch to it. If not, leave.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — your file
- `src/Chap43/OrderedTableMtPer.rs` — current ensures
- Agent1 R96 report (`plans/agent1-r96-mtper-rwlock-report.md`) — what was proved and what blocks remain

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify Chap43 files (agent2 is on that).
- Do NOT add new assumes.
- Even proving 3-4 more assumes is a good round.

## STEP 20

## Report

Write `plans/agent3-r97-mtper-assumes-report.md`.
