# R70 Restart Plan

## State

- Main: `d656155b3` — Verus upgraded to `ff454ab0f`, 4435 verified, 0 errors
- Rust toolchain: 1.94.0
- Holes: 32 (Chap39: 1, Chap43: 31)
- Clean chapters: 44 of 46
- R69 agent work was not merged. Agents reset to main.

## Hole Breakdown

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 39 | BSTTreapStEph.rs | 1 | assume (eq/clone) + 28 fn_missing_wf |
| 2 | 43 | OrderedTableStEph.rs | 8 | 6 assumes + 2 external_body |
| 3 | 43 | OrderedTableStPer.rs | 20 | 20 constructor axiom assumes |
| 4 | 43 | OrderedSetStEph.rs | 3 | 2 clone assumes + 1 unsafe |

## Agent Assignments

| # | Agent | File | Holes | Approach |
|---|-------|------|-------|----------|
| 1 | Agent 1 | OrderedTableStEph.rs | 8 | Lift constructor axioms to requires + prove rank_key/select_key |
| 2 | Agent 2 | OrderedTableStPer.rs | 20 | Lift constructor axioms to requires (empty/singleton/tabulate) |
| 3 | Agent 3 | BSTTreapStEph.rs | 1+28 | wf propagation to _st helpers + reduce clone assume |
| 4 | Agent 4 | OrderedSetStEph.rs | 3 | clone_plus for clone assumes + iterator unsafe |

## Target: 32 → 0 holes

## Prompt Files

- `plans/r70-agent1-orderedtablesteph-holes.md`
- `plans/r70-agent2-orderedtablestper-holes.md`
- `plans/r70-agent3-bsttreapsteph-wf.md`
- `plans/r70-agent4-orderedsetsteph-cleanup.md`
