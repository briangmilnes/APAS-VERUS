# R154 Agent 4 — Migrate OrderedTableMtEph + MtPer to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs`.
Read `src/Chap43/OrderedTableMtEph.rs` and `src/Chap43/OrderedTableMtPer.rs`.

Report file: `plans/r154-agent4-ordtable-mt-migrate-report.md`

## Problem

The Mt variants of OrderedTable may still reference `ParamBST<Pair<K,V>>`
directly or access `self.tree` (which is now `OrdKeyMap`) through `.inner`.
Since agents 1+2 are delegating StEph/StPer to OrdKeyMap, the Mt wrappers
(which delegate to the St layer) should automatically benefit. But they may
need import updates and struct field type changes.

## What to do

### OrderedTableMtEph.rs

1. Check if it wraps OrderedTableStEph or has its own ParamBST
2. If it has `ParamBST` references, change to `OrdKeyMap`
3. If it wraps OrderedTableStEph, it should inherit the simplification —
   just fix any `.tree.inner` references that should be `.tree`
4. Update imports
5. Validate: `scripts/validate.sh isolate Chap43`

### OrderedTableMtPer.rs

Same steps.

### AugOrderedTableMtEph.rs

Check if it needs updates too — it wraps OrderedTable.

### AugOrderedTableStEph.rs and AugOrderedTableStPer.rs

These were touched in R153 to add `.inner`. Check if they still compile
correctly and simplify any `.tree.inner` references that can now be `.tree`.

## Expected reduction

Mt files are already thin (1,094 and 768 lines). The reduction may be modest
(fixing references, simplifying imports) but ensures the whole Chap43 family
is consistently using OrdKeyMap.

## Validation

`scripts/validate.sh isolate Chap43` after each file.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or OrderedTableStEph.rs or OrderedTableStPer.rs.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
