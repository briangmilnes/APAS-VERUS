# R155 Agent 4 — Delegate Remaining OrderedTableStPer Methods. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — check what methods are available.
Read `src/Chap43/OrderedTableStPer.rs` — your file.

Report file: `plans/r155-agent4-ordtable-stper-delegate-remaining-report.md`

## Problem

Same as Agent 3 but for StPer. After R154, these methods still use
`self.tree.inner` directly:

- `split_key_iter` (needs disjointness)
- `empty` / `singleton` (need wf from new)
- `find` (requires find_pre, weaker than wf)
- `union` / `intersection` (need combiner fn)
- `domain` / `tabulate` / `map` / `filter` / `collect`
- `restrict` / `subtract` / `get_key_range` / `split_rank_key`
- `first_key` / `last_key`
- `bst_next_by_key` / `bst_prev_by_key` (bypassed with cfg(never), can delete)

## What to do

1. Check what OrdKeyMap methods are available on your branch
2. Delegate everything you can
3. Delete the `#[cfg(never)]` bypassed dead code
4. Delete dead bridge lemmas once their callers are gone
5. Track what's left

### Definitely deletable now

The three `#[cfg(never)]` blocks from R154:
- `lemma_cmp_antisymmetry` (~10 lines)
- `bst_next_by_key` (~355 lines)
- `bst_prev_by_key` (~335 lines)

These are already dead. Delete them outright (they're bypassed, not active).

## Validation

`scripts/validate.sh isolate Chap43` after each change.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or OrderedTableStEph.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report line count before/after.
