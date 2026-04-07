# R158 Agent 4 — Dead Code Cleanup Across StEph + StPer. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap43/OrderedTableStEph.rs` and `src/Chap43/OrderedTableStPer.rs`.

Report file: `plans/r158-agent4-dead-code-report.md`

## Problem

After multiple rounds of delegation, both files likely have:
- Bridge lemmas with zero callers (moved to OrdKeyMap)
- Spec fns only used by deleted functions
- `#[cfg(never)]` bypassed code
- Dead imports

## What to do

For each file:

1. Delete all `#[cfg(never)]` blocks entirely.

2. For each proof fn / spec fn in section 6-7:
   - Search the SAME FILE for callers (grep for the function name)
   - If zero callers, delete it
   - If still called, leave it

3. Clean up dead `use` imports.

4. Validate after each batch: `scripts/validate.sh isolate Chap43`

## Validation

Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.
- Delete only genuinely dead code (zero callers in file).

## When done

RCP. Report what was deleted, line counts before/after.
