# Restart Plan — R69 Launch

## Current State

- **Main**: `461e13d03`, pushed, clean.
- **Verified**: 4435, 0 errors. RTT: 2528. PTT: 145.
- **Holes**: 34 total (Chap39: 1, Chap43: 33).
- **Round**: R68 complete. R69 prompts written but agents not yet launched.

## What Just Happened

1. **R68 complete**: All 5 agents merged. 132 → 34 holes (-98).
2. **Finiteness fixer**: Ran `veracity-fix-redundant-finites`. 118 of 145 removals kept.
   Reverted OrderedTableStEph.rs (23 removals) and OrderedTableMtEph.rs (4 removals)
   because StEph trait ensures cascade to MtEph wrapper. Committed and pushed.
3. **R69 prompts**: 5 agent prompts committed in `plans/r69-agent*.md`.

## R69 Agent Assignments (34 holes → target ~3)

| # | Agent | File(s) | Holes | Target |
|---|-------|---------|-------|--------|
| 1 | Agent 1 | Chap43/OrderedTableStEph.rs | 2 (rank_key, select_key external_body) | 0 |
| 2 | Agent 2 | Chap43/OrderedTableStPer.rs | 21 (constructor axioms) | ~3 |
| 3 | Agent 3 | Chap43/OrderedTableStEph.rs | 6 (tabulate axioms) + 1 wf ensures | ~1 |
| 4 | Agent 4 | Chap43/OrderedSetStEph.rs + Chap39/BSTTreapStEph.rs | 3 + 1 | 0 |
| 5 | Agent 5 | Chap43 various (eq/clone accepts, iter assumes) | ~6 standard pattern | ~2 |

**Conflict note**: Agents 1 and 3 both touch OrderedTableStEph.rs. Merge Agent 1 first
(it only changes rank_key/select_key), then Agent 3 (tabulate axioms). Agents 2, 4, 5
have no file conflicts.

## To Launch R69

1. Rebase agents onto main: `scripts/rebase-agents.sh`
2. Give each agent its prompt from `plans/r69-agent{N}-*.md`
3. Agents 2, 4, 5 can run in parallel. Agents 1 and 3 merge sequentially.

## Pending (Not R69)

- **Veracity tool prompts** in `plans/`:
  - `cost-analysis-matching-prompt.md` — match APAS cost specs vs source (held)
  - `veracity-cross-variant-spec-diff-prompt.md` — compare specs across St/Mt/Eph/Per
  - `veracity-review-status-tracker-prompt.md` — REVIEWED: annotation tracker
- **Verus upgrade**: 32 new commits available (IEEE float SMT, mut_ref_tracked). Held
  until experiments written.
- **Finiteness cascade**: OrderedTableStEph.rs `.dom().finite()` in trait ensures is needed
  by OrderedTableMtEph.rs. To fix: either add `.dom().finite()` to MtEph's wf, or make
  the fixer check for downstream dependents. Low priority.
