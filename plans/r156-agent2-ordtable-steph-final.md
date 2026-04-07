# R156 Agent 2 — Final OrderedTableStEph Delegation. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — check ALL available methods.
Read `src/Chap43/OrderedTableStEph.rs` — your file.

Report file: `plans/r156-agent2-ordtable-steph-final-report.md`

## Problem

R155 added many new OrdKeyMap methods that StEph can now delegate to.
These methods were NOT available when R154 agent1 ran:

- `union_with` — replaces StEph's combiner union
- `intersect_with` — replaces StEph's combiner intersect
- `split` (now with disjointness) — replaces bst_split_by_key
- `first_key` / `last_key` — replaces first_key_iter / last_key_iter
- `get_key_range` — replaces get_key_range_iter
- `split_rank_key` — replaces split_rank_key_iter
- `new()` (now with wf) — replaces empty's manual construction

## What to delegate

| # | StEph method | OrdKeyMap method | Expected savings |
|---|---|---|---|
| 1 | empty | new (now ensures wf) | ~5 lines |
| 2 | singleton | new + insert | ~10 lines |
| 3 | union | union_with | ~255 lines |
| 4 | intersection | intersect_with | ~130 lines |
| 5 | split_key_iter | split (now with disjointness) | ~410 lines (bst_split_by_key) |
| 6 | first_key_iter | first_key | ~5 lines |
| 7 | last_key_iter | last_key | ~5 lines |
| 8 | get_key_range_iter | get_key_range | ~50 lines |
| 9 | split_rank_key_iter | split_rank_key | ~50 lines |

After delegating, delete dead code:
- `bst_split_by_key` (~410 lines) if no longer called
- `bst_find_by_key` (~145 lines) if no longer called
- Remaining bridge lemmas that have no callers
- Dead spec fns

## Expected reduction

From ~3,967 lines to ~2,000-2,500 lines.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.
- COMMENT OUT dead code first, validate, then delete.

## When done

RCP. Report line count before/after, list of dead code deleted.
