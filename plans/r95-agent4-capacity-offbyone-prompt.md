# R95 Agent 4 — Fix capacity off-by-one + remaining provable assumes, STEP 15

## Objective

Fix the capacity off-by-one assumes and any other provable assumes remaining
in AdjTableGraphStEph and AdjTableGraphStPer.

## Capacity off-by-one (2 assumes)

In insert_edge, the neighbor set insert needs `ns@.len() + 1 < usize::MAX`.
The graph wf gives `ns@.len() < usize::MAX` (from AVLTreeSetStEph wf). But
insert needs strictly less than MAX minus one.

The fix: prove `ns@.len() < dom.len()` from graph closure (`ns@ ⊆ domain`).
Then `dom.len() + 1 < usize::MAX` (from the function's requires) gives
`ns@.len() + 1 <= dom.len() < usize::MAX`.

Agent2 (R93) already proved the subset reasoning (`ns@ ⊆ domain` via graph
closure trigger). The off-by-one may just need one more step:
`vstd::set_lib::lemma_len_subset` gives `ns@.len() <= dom.len()`, and the
strict inequality `dom.len() + 1 < usize::MAX` gives the bound.

## Other provable assumes

Scan both StEph and StPer files for any assume that doesn't fall into the
ICE-blocked or clone-gap categories. Possible targets:

- `domain.spec_arraysetsteph_wf()` — if Table::domain ensures wf, prove it
- Any assume with a comment like "from Table ensures" — try removing the
  assume and letting Verus prove it directly

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — find capacity assumes in insert_edge
- `src/Chap52/AdjTableGraphStPer.rs` — same
- Agent2's R93 report (plans/agent2-r92-capacity-report.md) — subset proof technique

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify MtPer files.
- Do NOT add new assumes.
- Focus on StEph first, then StPer.

## STEP 15

## Report

Write `plans/agent4-r95-capacity-report.md`.
